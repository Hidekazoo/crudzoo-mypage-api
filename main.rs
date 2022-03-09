use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use rust_auth0_practice::auth::jwt::{get_token, validate_token};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    iss: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize)]
struct Metadata {
    api: String,
    buranch: String,
}
#[derive(Serialize)]
struct PostResponse {
    metadata: Metadata,
    text: String,
}
#[derive(Serialize)]
struct UnauthorizedErrorResponse {
    message: String,
}

#[get("/post")]
async fn auth2(req: HttpRequest) -> impl Responder {
    let jwt = match get_token(&req) {
        Ok(v) => v,
        _ => return HttpResponse::Ok().body(format!("invalid token")),
    };
    match validate_token(&jwt) {
        Ok(v) => {
            if v == true {
                return HttpResponse::Ok().json(PostResponse {
                    metadata: Metadata {
                        api: "test".to_string(),
                        buranch: "test".to_string(),
                    },
                    text: "aabbb".to_string(),
                });
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
                message: "invalid token".to_string(),
            })
        }
    }
    return HttpResponse::Unauthorized().json(UnauthorizedErrorResponse {
        message: "invalid token".to_string(),
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(auth2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
