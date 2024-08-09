use actix_session::Session;
use actix_web::{ Responder, HttpResponse, Error};
use serde::Serialize;

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}
pub async fn protected(session: Session) -> Result<HttpResponse, Error> {
    if let Some(user_id) = session.get::<String>("user_id")? {
        Ok(HttpResponse::Ok().json(LoginResponse {
            message: format!("Welcome to the protected route, {}!", user_id),
        }))
    } else {
        Ok(HttpResponse::Unauthorized().json(LoginResponse {
            message: "Access denied".to_string(),
        }))
    }
}

pub async fn unprotected() -> impl Responder {
    HttpResponse::Ok().json(LoginResponse {
        message: "This is an unprotected route".to_string(),
    })
}