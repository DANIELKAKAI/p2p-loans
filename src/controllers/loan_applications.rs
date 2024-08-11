use actix_session::Session;
use actix_web::{web, HttpResponse, HttpRequest};
use crate::db_operations::users::get_a_user_by_id;
use crate::db_operations::loan_applications::{get_loan_applications_by_lender_id, get_loan_applications_by_borrower_id};
use crate::models::app_state::AppState;
use crate::models::ui::{LendedLoansTemplate, AppliedLoansTemplate};
use askama::Template;
use log::{error, info, debug};



pub async fn lended_loans_page(state: web::Data<AppState>, session: Session, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("user id found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();
            let lended_loans = get_loan_applications_by_lender_id(&mut connection_guard, user.id);

            let dashboard_template = LendedLoansTemplate {
                user,
                lended_loans
            };
            Ok(HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().map_err(|e| {
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


pub async fn applied_loans_page(state: web::Data<AppState>, session: Session, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("user id found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();
            let applied_loans = get_loan_applications_by_borrower_id(&mut connection_guard, user_id);

            let dashboard_template = AppliedLoansTemplate {
                user,
                applied_loans
            };
            Ok(HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().map_err(|e| {
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
