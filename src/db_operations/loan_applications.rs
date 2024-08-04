use crate::models::loan_applications::{LoanApplication, NewLoanApplication};
use diesel::prelude::*;

pub fn get_all_loan_applications(connection: &mut PgConnection) -> Vec<LoanApplication> {
    use crate::schema::loan_applications::dsl::*;

    let mut all_loan_applications: Vec<LoanApplication> = Vec::new();
    let results = loan_applications
        .select(LoanApplication::as_select())
        .load(connection);

    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                all_loan_applications.push(loan)
            }
        }
        Err(e) => println!("Error occured {:?}", e),
    }

    return all_loan_applications;
}

pub fn get_loan_applications_by_lender_id(
    connection: &mut PgConnection,
    user_id: i32,
) -> Vec<LoanApplication> {
    use crate::schema::loan_applications::dsl::*;
    use crate::schema::loans::dsl::*;

    let results = loan_applications
        .inner_join(loans)
        .filter(lender_id.eq(user_id))
        .select(crate::schema::loan_applications::all_columns)
        .load::<LoanApplication>(connection);

    match results {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error occurred: {:?}", e);
            Vec::new()
        }
    }
}

pub fn get_loan_applications_by_borrower_id(
    connection: &mut PgConnection,
    user_id: i32,
) -> Vec<LoanApplication> {
    use crate::schema::loan_applications::dsl::*;

    let mut all_loan_applications: Vec<LoanApplication> = Vec::new();

    let results = loan_applications
        .filter(borrower_id.eq(user_id))
        .select(crate::schema::loan_applications::all_columns)
        .load(connection);

    match results {
        Ok(data) => {
            for loan_app in data.into_iter() {
                all_loan_applications.push(loan_app)
            }
        }
        Err(e) => println!("Error occured {:?}", e),
    }

    return all_loan_applications;
}

pub fn add_loan_application(
    new_loan_application: NewLoanApplication,
    connection: &mut PgConnection,
) -> Result<LoanApplication, diesel::result::Error> {
    diesel::insert_into(crate::schema::loan_applications::table)
        .values(&new_loan_application)
        .get_result::<LoanApplication>(connection)
}
