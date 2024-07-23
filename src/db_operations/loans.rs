use crate::models::loans::{Loan, NewLoan};
use diesel::prelude::*;

pub fn get_all_users(connection: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let mut all_users: Vec<User> = Vec::new();
    let results = users.select(User::as_select()).load(connection);
    match results {
        Ok(data) => {
            for post in data.into_iter() {
                all_users.push(post)
            }
        }
        Err(e) => println!("Error occured {:?}", e),
    }

    return all_users;
}

pub fn get_a_user_by_mail(connection: &mut PgConnection, user_email: String) -> Option<User> {
    use crate::schema::users::dsl::*;

    users
        .filter(email.eq(user_email))
        .first::<User>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn get_loans_by_lender_id(connection: &mut PgConnection, user_id: i32) -> Option<User> {
    use crate::schema::users::dsl::*;

    let mut loans: Vec<Loan> = Vec::new();

    let results = loans.filter(lender_id.eq(user_id)).load(connection);

    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                loans.push(loan)
            }
        }
        Err(e) => println!("Error occured {:?}", e),
    }

    return all_users;
}

pub fn add_loan(
    new_loan: NewLoan,
    connection: &mut PgConnection,
) -> Result<Loan, diesel::result::Error> {
    diesel::insert_into(crate::schema::loans::table)
        .values(&new_loan)
        .get_result::<Loan>(connection)
}
