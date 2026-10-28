use crate::routes::health_check;
use crate::routes::subscribe;

use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self},
};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
