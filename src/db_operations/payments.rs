use crate::models::payments::{NewPayment, Payment};
use crate::schema::payment::dsl::*;
use diesel::dsl::sum;
use diesel::prelude::*;

pub fn insert_payment(
    new_payment: NewPayment,
    connection: &mut PgConnection,
) -> Result<Payment, diesel::result::Error> {
    diesel::insert_into(payment)
        .values(&new_payment)
        .get_result::<Payment>(connection)
}

pub fn get_total_by_loan_id(loan_id_value: i32, connection: &mut PgConnection) -> f64 {
    let result: Result<Option<f64>, diesel::result::Error> = payment
        .select(sum(amount))
        .filter(loan_id.eq(loan_id_value))
        .first(connection);

    match result {
        Ok(Some(total)) => total,
        Ok(None) => 0.0,
        Err(_) => 0.0,
    }
}
