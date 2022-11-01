use crate::utils::CustomError;
use actix_web::{get, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World 123")
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[derive(Serialize)]
struct MyPathObj {
    name: String,
    first_path: u32,
    second_path: String,
}

#[get("/json")]
pub async fn json() -> impl Responder {
    let obj = MyObj {
        name: "this is the json endpoint".to_string(),
    };
    web::Json(obj)
}
#[derive(Deserialize)]
struct PathReq {
    first_path: u32,
    second_path: String,
}

#[get("/paths/{first_path}/{second_path}/")]
async fn path(path: Result<web::Path<PathReq>, Error>) -> impl Responder {
    match path {
        Ok(path) => {
            let obj = MyPathObj {
                name: "this is the json endpoint".to_string(),
                first_path: path.first_path,
                second_path: format!("{}", path.second_path),
            };
            Ok(web::Json(obj))
        }
        Err(error) => {
            return Err(CustomError::BadClientData {
                field: error.to_string(),
            })
        }
    }
}
