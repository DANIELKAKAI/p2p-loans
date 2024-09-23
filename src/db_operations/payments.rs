use crate::models::payments::{NewPayment, Payment};
use diesel::prelude::*;

pub fn insert_payment(
    new_payment: NewPayment,
    connection: &mut PgConnection,
) -> Result<Payment, diesel::result::Error> {
    diesel::insert_into(crate::schema::payment::table)
        .values(&new_payment)
        .get_result::<Payment>(connection)
}
