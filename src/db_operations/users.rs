use crate::models::users::{NewUser, User};

mod crate::schema::users;
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub fn get_all_users(connection: &mut PgConnection) -> Vec<User> {
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
    users
        .filter(users::email.eq(user_email))
        .first::<User>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn get_a_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<User> {
    users
        .filter(users::id.eq(user_id))
        .first::<User>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}

pub fn add_user(
    new_user: NewUser,
    connection: &mut PgConnection,
) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
}
