use crate::api::api::{hello, json, path};
use actix_web::{
    middleware::{self, Logger},
    App, HttpServer,
};

mod api;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Always,
            ))
            .service(hello)
            .service(json)
            .service(path)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
