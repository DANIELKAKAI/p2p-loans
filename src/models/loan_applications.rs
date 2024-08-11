use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::users::User;
use crate::models::loans::Loan;

#[derive(Queryable, Debug, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::loan_applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoanApplication {
    pub id: i32,
    pub amount: f64,
    pub borrower_id: i32,
    pub loan_id: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::loan_applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoanApplication {
    pub amount: f64,
    pub borrower_id: i32,
    pub loan_id: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct LoanApplicationWithBorrower {
    pub loan_application: LoanApplication,
    pub borrower: User,
    pub loan:Loan
}


#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct LoanApplicationWithLender {
    pub loan_application: LoanApplication,
    pub loan:Loan,
    pub lender: User,
}
