

use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder};
use sha2::{Sha256, Digest};
use serde::Deserialize;

#[derive(Deserialize)]
struct HashInput {
    data: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!!!!")
}

#[post("/hash")]
async fn hash_data(info: web::Json<HashInput>) -> impl Responder {
    let mut hasher = Sha256::new();
    hasher.update(info.data.as_bytes());
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);
    web::Json(hash_hex)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hash_data)
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}