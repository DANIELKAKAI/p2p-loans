
mod db_operations;
mod models;
mod schema;
mod controllers;


use db_operations::db;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use dotenvy::dotenv;
use crate::controllers::users::{dashboard_page, login_page, login_user, register_page, register_user};
use log::info;

use std::sync::Mutex;
use actix_files as fs;
use actix_web::{cookie::Key, web, App, HttpServer};
use crate::models::app_state::AppState;

use crate::controllers::dashboard::{protected, unprotected};
use crate::controllers::home::default_handler;
use actix_web::cookie::SameSite;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    info!("starting HTTP server at http://localhost:8080");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        // Initialize application state
        let app_state = web::Data::new(AppState { db_connection: Mutex::new(db::establish_connection())  });
        // todo improve above to use a  pool not a single connection

        App::new()
            .app_data(app_state.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // set to true if using HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build()
            )
            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true)
            )
            .route("/dashboard", web::get().to(dashboard_page))
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login_user))
            .route("/register", web::get().to(register_page))
            .route("/register", web::post().to(register_user))
            .service(web::resource("/protected").route(web::get().to(protected)))
            .service(web::resource("/unprotected").route(web::get().to(unprotected)))
            .default_service(web::to(default_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}