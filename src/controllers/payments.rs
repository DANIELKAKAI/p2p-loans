use crate::db_operations::payments::insert_payment;
use crate::db_operations::loans::update_loan_deposit;
use crate::models::app_state::AppState;
use crate::models::payments::{NewPayment, PaymentCallbackForm};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn payment_callback(
    query: web::Query<PaymentCallbackForm>,
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

    match update_loan_deposit(query.orderReference, true, &mut connection_guard) {
        Ok(_) => {},
        Err(e) => {
            println!("Error occurred: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError(
                "Error updating loan",
            ));
        }
    }

    Ok(HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, "/dashboard"))
            .finish())
}
