use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::payment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Payment {
    pub id: i32,
    pub payment_code: String,
    pub amount: f64,
    pub loan_id: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::payment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPayment {
    pub payment_code: String,
    pub loan_id: i32,
    pub amount: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentCallbackForm {
    pub responseStatus: String,
    pub transactionStatus: String,
    pub orderReference: i32,
    pub extraData: String,
    pub transactionReference: String,
    pub transactionDate: String,
    pub transactionAmount: f64,
    pub transactionCurrency: String,
    pub message: String,
    pub paymentChannel: String,
    pub orderItems: String,
    pub secureResponse: String,
}
