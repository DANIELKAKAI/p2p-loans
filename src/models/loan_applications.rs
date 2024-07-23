use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::loan_applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoanApplication {
    pub id: u32,
    pub amount: f32,
    pub user_id: u32,
    pub loan_id: u32,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::loan_applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoanApplication {
    pub amount: f32,
    pub user_id: u32,
    pub loan_id: u32,
}
