use crate::models::loans::{Loan, NewLoan};

mod crate::schema::loans;
use crate::schema::loans;
use crate::schema::loans::dsl::*;
use diesel::prelude::*;

pub fn get_all_loans(connection: &mut PgConnection) -> Vec<Loan> {
    let mut all_loans: Vec<Loan> = Vec::new();
    let results = loans.select(Loan::as_select()).load(connection);
    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                all_loans.push(loan)
            }
        }
        Err(e) => println!("Error occured {:?}", e),
    }

    return all_loans;
}

pub fn get_loans_by_lender_id(connection: &mut PgConnection, user_id: i32) -> Option<Loan> {
    let mut all_loans: Vec<Loan> = Vec::new();

    let results = loans
        .filter(loans::lender_id.eq(user_id))
        .select(loans::all_columns)
        .load(connection);

    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                all_loans.push(loan)
            }
        }
        Err(e) => println!("Error occured {:?}", e),
    }

    return all_loans;
}

pub fn add_loan(
    new_loan: NewLoan,
    connection: &mut PgConnection,
) -> Result<Loan, diesel::result::Error> {
    diesel::insert_into(loans::table)
        .values(&new_loan)
        .get_result::<Loan>(connection)
}
