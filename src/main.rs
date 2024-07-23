use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db_operations;
use db_operations::users::{add_user};

mod models;
use models::users::NewUser;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();

    let new_user = NewUser{
        first_name:"daniel".to_string(),
        last_name:"kakai".to_string(),
        email:"daniel@gmail.com".to_string(),
        password:"password".to_string()
    }

    match add_user(new_user, &mut connection) {
        Ok(user) => {
            println!("Successfully added user: {:?}", user);
        }
        Err(e) => {
            println!("Error occurred: {:?}", e);
        }
    }
}
