use crate::db_operations::loans::{get_loan_by_id, insert_loan};
use crate::db_operations::users::get_a_user_by_id;
use crate::models::app_state::AppState;
use crate::models::loans::{AddLoanForm, NewLoan};
use crate::models::payments::PaymentConfig;
use crate::models::ui::{AddLoanTemplate, CompleteLoanPaymentTemplate};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_lab::extract::Path;
use askama::Template;
use log::{debug, error, info};
use std::env;

use crate::utils::utils::get_jenga_payment_token;

pub async fn add_loan_page(
    state: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("email found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();

            let add_loan_template = AddLoanTemplate { user };
            Ok(HttpResponse::Ok().content_type("text/html").body(
                add_loan_template.render().map_err(|e| {
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
            Err(actix_web::error::ErrorInternalServerError(
                "Error adding loan",
            ))
        }
    }
}

pub async fn complete_loan_payment(
    Path(loan_id): Path<i32>,
    state: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("email found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();
            let loan = get_loan_by_id(&mut connection_guard, loan_id).unwrap();
            //let token: String = get_jenga_payment_token().unwrap();
            let payment_config = PaymentConfig {
                token: String::from("token"),
                merchantCode: env::var("JENGA_MERCHANT_CODE").unwrap_or("".to_string()),
                orderReference: loan.id.to_string(),
                productType: String::from("Service"),
                paymentTimeLimit: env::var("JENGA_PAYMENT_TIME_LIMIT").unwrap_or("".to_string()),
                callbackUrl: env::var("JENGA_CALLBACK_URL").unwrap_or("".to_string()),
                countryCode: env::var("JENGA_COUNTRY_CODE").unwrap_or("".to_string()),
                currency: env::var("JENGA_CURRENCY").unwrap_or("".to_string()),
                signature: format!(
                    "{}{}{}{}",
                    env::var("JENGA_MERCHANT_CODE").unwrap_or("".to_string()),
                    loan.id.to_string(),
                    env::var("JENGA_CURRENCY").unwrap_or("".to_string()),
                    env::var("JENGA_CALLBACK_URL").unwrap_or("".to_string())
                ),
            };

            let template = CompleteLoanPaymentTemplate {
                user,
                loan,
                payment_config,
            };
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().map_err(|e| {
                    error!("Template rendering error: {:?}", e);
                    actix_web::error::ErrorInternalServerError("Template error")
                })?))
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
