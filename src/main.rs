use std::fs;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use serde_json::json;

use settings::settings::Settings;

mod settings;

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
    let contents = fs::read_to_string("settings.toml")
        .expect("Failed to read settings.toml");
    let settings: Settings = toml::from_str(&contents)
        .expect("Failed to parse settings.toml");
    println!("{:#?}", settings);

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

