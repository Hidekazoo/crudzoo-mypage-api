use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crudzoo_mypage_api::auth::jwt::{get_token, validate_token};
use serde::{Serialize};
use dotenv::dotenv;


#[derive(Serialize)]
struct UnauthorizedErrorResponse {
    message: String,
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

#[get("/post")]
async fn auth_test(req: HttpRequest) -> impl Responder {
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
                    text: "test".to_string(),
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
            .service(auth_test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}