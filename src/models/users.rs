use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;
use std::fmt;

use crate::schema;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::UserTypeEnum)]
pub enum UserType {
    BORROWER,
    LENDER,
    ADMIN,
}

impl FromStr for UserType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BORROWER" => Ok(UserType::BORROWER),
            "LENDER" => Ok(UserType::LENDER),
            "ADMIN" => Ok(UserType::ADMIN),
            _ => Err(format!("'{}' is not a valid UserType", s)),
        }
    }
}

impl ToSql<crate::schema::sql_types::UserTypeEnum, Pg> for UserType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserType::BORROWER => out.write_all(b"BORROWER")?,
            UserType::LENDER => out.write_all(b"LENDER")?,
            UserType::ADMIN => out.write_all(b"ADMIN")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::UserTypeEnum, Pg> for UserType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"BORROWER" => Ok(UserType::BORROWER),
            b"LENDER" => Ok(UserType::LENDER),
            b"ADMIN" => Ok(UserType::ADMIN),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user_type_str = match self {
            UserType::BORROWER => "BORROWER",
            UserType::LENDER => "LENDER",
            UserType::ADMIN => "ADMIN",
        };
        write!(f, "{}", user_type_str)
    }
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub user_type: UserType,
}

#[derive(Queryable, Selectable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub user_type: UserType,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_type: String,
    pub password: String,
}