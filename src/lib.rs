use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    dev::Server,
    web::{self},
};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
#[expect(dead_code, reason = "Subscription persistence is not implemented yet")]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
