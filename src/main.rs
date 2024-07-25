mod db_operations;
mod models;
mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use db_operations::loans::get_loans_by_lender_id;
//use db_operations::users::{add_user, get_a_user_by_id};

use models::users::NewUser;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();

    let loans = get_loans_by_lender_id(&mut connection, 3);

    for loan in loans {
        println!("{:?}", loan);
    }
}
