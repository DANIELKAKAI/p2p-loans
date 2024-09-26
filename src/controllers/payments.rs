use crate::db_operations::loans::{get_loan_by_id, update_loan_deposit};
use crate::db_operations::payments::{get_total_by_loan_id, insert_payment};
use crate::db_operations::users::get_a_user_by_id;
use crate::models::app_state::AppState;
use crate::models::payments::{MpesaCheckoutQuery, NewPayment, PaymentCallbackQuery};
use crate::models::ui::MpesaCheckoutTemplate;
use crate::utils::utils::initiate_mpesa_transaction;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use log::{debug, error, info};

pub async fn payment_callback(
    query: web::Query<PaymentCallbackQuery>,
    state: web::Data<AppState>,
    _session: Session,
    _req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    if query.transactionStatus != "SUCCESS" {
        return Ok(HttpResponse::Ok().body("success"));
    }

    let new_payment = NewPayment {
        payment_code: query.transactionReference.clone(),
        loan_id: query.orderReference,
        amount: query.transactionAmount,
    };

    match insert_payment(new_payment, &mut connection_guard) {
        Ok(payment) => {
            println!("Successfully added payment: {:?}", payment);
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError(
                "Error adding payment",
            ));
        }
    }

    if let Some(loan) = get_loan_by_id(&mut connection_guard, query.orderReference) {
        let total = get_total_by_loan_id(query.orderReference, &mut connection_guard);

        if loan.loan_amount == total {
            match update_loan_deposit(query.orderReference, true, &mut connection_guard) {
                Ok(_) => {
                    println!("Successfully updated loan status");
                }
                Err(e) => {
                    println!("Error occurred: {:?}", e);
                    return Err(actix_web::error::ErrorInternalServerError(
                        "Error updating loan",
                    ));
                }
            }
        }
    }

    Ok(HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, "/dashboard"))
        .finish())
}

pub async fn mpesa_checkout_page(
    query: web::Query<MpesaCheckoutQuery>,
    state: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();

            let template = MpesaCheckoutTemplate {
                customerPhone: query.customerPhone.clone(),
                user: user,
            };

            let _res = initiate_mpesa_transaction(
                &query.customerPhone.clone(),
                &query.loan_id.clone(),
                &query.orderAmount.clone(),
            ).unwrap();

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
