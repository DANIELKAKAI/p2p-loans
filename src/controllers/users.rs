use crate::db_operations::loans::{get_all_loans, get_loans_by_lender_id};
use crate::db_operations::users::{add_user, get_a_user_by_id, get_a_user_by_mail};
use crate::models::app_state::AppState;
use crate::models::loans::Loan;
use crate::models::ui::{DashboardTemplate, LoginTemplate, RegisterTemplate};
use crate::models::users::{LoginForm, NewUser, RegisterForm, User, UserType};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use bcrypt::{hash, verify, DEFAULT_COST};
use log::{debug, error, info};
use std::str::FromStr;

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate {
        error: Some(error.to_string()),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn register_page(error: Option<String>) -> impl Responder {
    let template = RegisterTemplate { error };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

async fn handle_login_information(error: &str) -> HttpResponse {
    let template = LoginTemplate {
        error: None,
        message: Some(error.to_string()),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn login_page(error: Option<String>, message: Option<String>) -> impl Responder {
    let template = LoginTemplate { error, message };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn dashboard_page(
    state: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("Attempting to retrieve user_id from session");

    let result = match session.get::<i32>("user_id") {
        Ok(Some(user_id)) => {
            info!("user id found in session: {}", user_id);

            let mut connection_guard = state.db_connection.lock().unwrap();

            // context
            let mut loans: Vec<Loan> = Vec::new();
            let user = get_a_user_by_id(&mut connection_guard, user_id).unwrap();

            if user.user_type.as_str() == "LENDER" {
                loans = get_loans_by_lender_id(&mut connection_guard, user.id);
            }
            if user.user_type.as_str() == "BORROWER" {
                loans = get_all_loans(&mut connection_guard);
            }

            let dashboard_template = DashboardTemplate { user, loans };
            Ok(HttpResponse::Ok().content_type("text/html").body(
                dashboard_template.render().map_err(|e| {
                    error!("Template rendering error: {:?}", e);
                    actix_web::error::ErrorInternalServerError("Template error")
                })?,
            ))
        }
        Ok(None) => {
            info!("No user_id found in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        }
        Err(e) => {
            error!("Session error: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    };

    result.map_err(|e| {
        error!("Unexpected error in dashboard_page: {:?}", e);
        e
    })
}

pub async fn login_user(
    form: web::Form<LoginForm>,
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_a_user_by_mail(&mut *connection_guard, form.email.clone());
    match user_exist {
        Some(user) => {
            if verify(&form.password, &user.password).unwrap_or(false) {
                session.insert("user_id", user.id)?;
                // Redirect to the dashboard route
                Ok(HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                    .finish())
            } else {
                let error_message = "Wrong password.".to_string();
                let template = LoginTemplate {
                    error: Some(error_message),
                    message: None,
                };
                Ok(HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template.render().unwrap()))
            }
        }
        None => {
            let error_message = "Email not found".to_string();
            let template = LoginTemplate {
                error: Some(error_message),
                message: None,
            };
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap()))
        }
    }
}

pub async fn register_user(
    item: web::Form<RegisterForm>,
    state: web::Data<AppState>,
) -> HttpResponse {
    println!("Data is {:#?}", item);
    if item.first_name.is_empty()
        || item.last_name.is_empty()
        || item.email.is_empty()
        || item.user_type.is_empty()
        || item.password.is_empty()
    {
        println!("Empty fields detected");
        return handle_register_error("All fields are required").await;
    }

    println!("All fields have content");

    // Hash and salt the password
    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(er) => {
            println!("error {}", er);
            return handle_register_error("error hashing password").await;
        }
    };

    let user_type_enum = match UserType::from_str(&item.user_type.clone()) {
        Ok(user_type) => user_type,
        Err(_) => {
            return handle_register_error("Invalid user type").await;
        }
    };

    let new_user = NewUser {
        first_name: item.first_name.clone(),
        last_name: item.last_name.clone(),
        email: item.email.clone(),
        password: hashed_password,
        user_type: user_type_enum,
    };

    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_a_user_by_mail(&mut *connection_guard, item.email.clone());

    if user_exist.is_some() {
        return handle_register_error("User with email exists").await;
    }

    let res = add_user(new_user, &mut *connection_guard);

    match res {
        Ok(user) => {
            return handle_login_information("Account created, please login to continue").await;
        }
        Err(err) => {
            println!("db error {:#?}", err);
            return handle_register_error("error creating account").await;
        }
    }
}

async fn update_user(item: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}
