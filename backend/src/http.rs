use actix_web::{HttpResponse, Responder};
use serde::Serialize;

pub fn respond(result: Result<impl Serialize, impl Serialize>) -> impl Responder {
    match result {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
