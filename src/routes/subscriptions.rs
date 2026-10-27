use actix_web::{
    HttpResponse, Responder,
    web::{self},
};

#[derive(serde::Deserialize)]
#[expect(dead_code, reason = "Subscription persistence is not implemented yet")]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
