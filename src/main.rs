use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("{\"status\":0, \"msg\":\"ok\"}")
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

