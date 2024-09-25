use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::users::User;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub id: i32,
    pub loan_name: String,
    pub loan_amount: f64,
    pub lender_id: i32,
    pub interest_rate: f64,
    pub repayment_period: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub amount_deposited: Option<bool>,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoan {
    pub loan_name: String,
    pub loan_amount: f64,
    pub interest_rate: f64,
    pub repayment_period: i32,
    pub lender_id: i32,
    pub amount_deposited: Option<bool>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddLoanForm {
    pub loan_name: String,
    pub loan_amount: f64,
    pub interest_rate: f64,
    pub repayment_period: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct FullLoan {
    pub loan:Loan,
    pub lender: User,
}
