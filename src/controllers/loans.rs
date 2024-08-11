use actix_session::Session;
use actix_web::{web, HttpResponse, HttpRequest};
use crate::models::loans::{NewLoan, AddLoanForm};
use crate::db_operations::users::{get_a_user_by_id};
use crate::db_operations::loans::insert_loan;
use crate::models::app_state::AppState;
use crate::models::ui::AddLoanTemplate;
use askama::Template;
use log::{error, info, debug};



pub async fn add_loan_page(state: web::Data<AppState>, session: Session, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("email found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();

            let add_loan_template = AddLoanTemplate {
                user
            };
            Ok(HttpResponse::Ok().content_type("text/html").body(add_loan_template.render().map_err(|e| {
                error!("Template rendering error: {:?}", e);
                actix_web::error::ErrorInternalServerError("Template error")
            })?))
        },
        Ok(None) => {
            info!("No user_id found in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        },
        Err(e) => {
            error!("Session error: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    };

    result.map_err(|e| {
        error!("Unexpected error in dashboard_page: {:?}", e);
        e
    })
}


pub async fn add_loan(
    form: web::Form<AddLoanForm>,
    state: web::Data<AppState>,
    session: Session,
    _req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let user_id = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => user_id,
        Ok(None) => {
            info!("No user_id found in session");
            return Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish());
        }
        Err(e) => {
            error!("Session error: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError("Session error"));
        }
    };

    // Obtain database connection from AppState
    let mut connection_guard = state.db_connection.lock().unwrap();

    let new_loan = NewLoan {
        loan_name: form.loan_name.clone(),
        loan_amount: form.loan_amount.clone(), 
        interest_rate: form.interest_rate.clone(), 
        repayment_period: form.repayment_period.clone(),
        lender_id: user_id,
    };

    match insert_loan(new_loan, &mut connection_guard) {
        Ok(loan) => {
            println!("Successfully added loan: {:?}", loan);
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                .finish())
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Error adding loan"))
        }
    }
}
