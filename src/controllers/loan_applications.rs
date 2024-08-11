use crate::db_operations::loan_applications::{
    add_loan_application, get_loan_applications_by_borrower_id, get_loan_applications_by_lender_id,
};
use crate::db_operations::loans::get_loan_by_id;
use crate::db_operations::users::get_a_user_by_id;
use crate::models::app_state::AppState;
use crate::models::loan_applications::NewLoanApplication;
use crate::models::ui::{AppliedLoansTemplate, LendedLoansTemplate};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_lab::extract::Path;
use askama::Template;
use log::{debug, error, info};

pub async fn lended_loans_page(
    state: web::Data<AppState>,
    session: Session,
    _req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("user id found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();
            let lended_loans = get_loan_applications_by_lender_id(&mut connection_guard, user.id);

            let dashboard_template = LendedLoansTemplate { user, lended_loans };
            Ok(HttpResponse::Ok().content_type("text/html").body(
                dashboard_template.render().map_err(|e| {
                    error!("Template rendering error: {:?}", e);
                    actix_web::error::ErrorInternalServerError("Template error")
                })?,
            ))
        }
        Ok(None) => {
            info!("No user_id found in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        }
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

pub async fn applied_loans_page(
    state: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("user id found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();
            let applied_loans =
                get_loan_applications_by_borrower_id(&mut connection_guard, user_id);

            let dashboard_template = AppliedLoansTemplate {
                user,
                applied_loans,
            };
            Ok(HttpResponse::Ok().content_type("text/html").body(
                dashboard_template.render().map_err(|e| {
                    error!("Template rendering error: {:?}", e);
                    actix_web::error::ErrorInternalServerError("Template error")
                })?,
            ))
        }
        Ok(None) => {
            info!("No user_id found in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        }
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

pub async fn apply_loan(
    Path(loan_id): Path<i32>,
    state: web::Data<AppState>,
    session: Session,
    _req: HttpRequest,
) -> impl Responder {
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

    let loan = get_loan_by_id(&mut connection_guard, loan_id).unwrap();

    let new_loan_app = NewLoanApplication {
        amount: loan.loan_amount,
        borrower_id: user_id,
        loan_id: loan_id,
    };

    match add_loan_application(new_loan_app, &mut connection_guard) {
        Ok(loan) => {
            println!("Successfully added loan: {:?}", loan);
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/applied-loans"))
                .finish())
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Error adding loan",
            ))
        }
    }
}
