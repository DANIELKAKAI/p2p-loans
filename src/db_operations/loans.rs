use crate::{models::loans::{FullLoan, Loan, NewLoan}, schema::loans::amount_deposited};
use diesel::prelude::*;

pub fn get_all_loans(connection: &mut PgConnection) -> Vec<FullLoan> {
    use crate::schema::loans::dsl::{created_at as loan_date, loans};
    use crate::schema::users::dsl::*;

    let results = loans
        .inner_join(users)
        .order(loan_date.desc())
        .load::<FullLoan>(connection);

    match results {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error occurred: {:?}", e);
            Vec::new()
        }
    }
}

pub fn get_loans_by_lender_id(connection: &mut PgConnection, user_id: i32) -> Vec<FullLoan> {
    use crate::schema::loans::dsl::{created_at as loan_date, lender_id, loans};
    use crate::schema::users::dsl::*;

    let mut all_loans: Vec<FullLoan> = Vec::new();

    let results = loans
        .inner_join(users)
        .filter(lender_id.eq(user_id))
        .order(loan_date.desc())
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

pub fn get_loan_by_id(connection: &mut PgConnection, loan_id: i32) -> Option<Loan> {
    use crate::schema::loans::dsl::*;

    loans
        .filter(id.eq(loan_id))
        .first::<Loan>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn insert_loan(
    new_loan: NewLoan,
    connection: &mut PgConnection,
) -> Result<Loan, diesel::result::Error> {
    diesel::insert_into(crate::schema::loans::table)
        .values(&new_loan)
        .get_result::<Loan>(connection)
}

pub fn update_loan_deposit(
    loan_id: i32,         
    value: bool, 
    connection: &mut PgConnection,
) -> Result<Loan, diesel::result::Error> {
    diesel::update(crate::schema::loans::table.find(loan_id))
        .set(
            crate::schema::loans::amount_deposited.eq(value),
        )
        .get_result::<Loan>(connection)
}
