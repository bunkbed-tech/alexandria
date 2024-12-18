use actix_web::scope;

#[scope("/bgg")]
pub mod bgg;
pub use bgg::*;
#[scope("/resource")]
pub mod resource;
pub use resource::*;
