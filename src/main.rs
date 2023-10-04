use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn health() -> impl Responder {
    let resp = json!({"status":0,"msg":"ok"});
    HttpResponse::Ok().body(serde_json::to_string_pretty(&resp).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/health", web::get().to(health))
    })
        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}

