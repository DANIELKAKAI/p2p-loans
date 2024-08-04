mod db_operations;
mod models;
mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

//use db_operations::loans::{add_loan, get_all_loans, get_loans_by_lender_id};
//use db_operations::users::{add_user, get_a_user_by_id};
use db_operations::loan_applications::{
    add_loan_application, get_all_loan_applications, get_loan_applications_by_borrower_id,
    get_loan_applications_by_lender_id,
};

//use models::loans::NewLoan;
//use models::users::NewUser;
use models::loan_applications::NewLoanApplication;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();

    let loans = get_all_loan_applications(&mut connection);
    for loan in loans {
        println!("{:?}", loan);
    }
}
