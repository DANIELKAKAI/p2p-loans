use crate::models::loan_applications::{
    LoanApplication, LoanApplicationWithBorrower, LoanApplicationWithLender, NewLoanApplication,
};
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
) -> Vec<LoanApplicationWithBorrower> {
    use crate::schema::loan_applications::dsl::*;
    use crate::schema::loans::dsl::*;
    use crate::schema::users::dsl::*;

    let results = loan_applications
        .inner_join(users)
        .inner_join(loans)
        .filter(lender_id.eq(user_id))
        .load::<LoanApplicationWithBorrower>(connection);

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
) -> Vec<LoanApplicationWithLender> {
    use crate::schema::loan_applications::dsl::*;
    use crate::schema::loans::dsl::*;
    use crate::schema::users::dsl::{users, id as user_id_field};

    let results = loan_applications
        .filter(borrower_id.eq(user_id))
        .inner_join(loans)
        .inner_join(users.on(user_id_field.eq(lender_id)))
        .load::<LoanApplicationWithLender>(connection);

    match results {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error occurred: {:?}", e);
            Vec::new()
        }
    }
}

pub fn add_loan_application(
    new_loan_application: NewLoanApplication,
    connection: &mut PgConnection,
) -> Result<LoanApplication, diesel::result::Error> {
    diesel::insert_into(crate::schema::loan_applications::table)
        .values(&new_loan_application)
        .get_result::<LoanApplication>(connection)
}
