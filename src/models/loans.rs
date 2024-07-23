use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub id: u32,
    pub loan_name: String,
    pub loan_amount: f32,
    pub interest_rate: f32,
    pub repayment_period: u32,
    pub lender_id: u32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoan {
    pub loan_name: String,
    pub loan_amount: f32,
    pub interest_rate: f32,
    pub repayment_period: u32,
    pub lender_id: u32,
}
