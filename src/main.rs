use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use serde::Serialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World 123")
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/json")]
async fn json() -> impl Responder {
    let obj = MyObj {
        name: "this is the json endpoint".to_string()
    };
    web::Json(obj)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(hello)
            .service(json)
    })

    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
