use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn handle_get() -> impl Responder {
    HttpResponse::Ok().body("Hello from the GET endpoint!\n")
}

async fn handle_post() -> impl Responder {
    HttpResponse::Ok().body("Hello from the POST endpoint!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "127.0.0.1:8888";

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handle_get))
            .route("/", web::post().to(handle_post))
    })
    .bind(bind_address)?
    .run()
    .await
}
