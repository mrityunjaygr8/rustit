use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::{Logger, self}, web, Error};
use serde::{Serialize, Deserialize};

pub mod utils;
use utils::CustomError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World 123")
}

#[derive(Serialize)]
struct MyObj {
    name: String
}

#[derive(Serialize)]
struct MyPathObj {
    name: String,
    first_path: u32,
    second_path: String,

}

#[get("/json")]
async fn json() -> impl Responder {
    let obj = MyObj {
        name: "this is the json endpoint".to_string()
    };
    web::Json(obj)
}

#[derive(Deserialize)]
struct PathReq {
    first_path: u32,
    second_path: String
}


#[get("/paths/{first_path}/{second_path}/")]
async fn path(path: Result<web::Path<PathReq>, Error>) -> impl Responder {
    match path {
        Ok(path) => {
            let obj = MyPathObj {
                name: "this is the json endpoint".to_string(),
                first_path: path.first_path,
                second_path: format!("{}", path.second_path)
            };
            Ok( web::Json(obj) )

        },
        Err(error) => return Err(CustomError::BadClientData{field: error.to_string()})
    }
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
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Always))
            .service(hello)
            .service(json)
            .service(path)
    })

    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
