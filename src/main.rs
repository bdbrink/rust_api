use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct MyData {
    id: u32,
    name: String,
}

struct AppState {
    data: Mutex<Vec<MyData>>,
}

async fn handle_get(state: web::Data<AppState>) -> impl Responder {
    let data = state.data.lock().unwrap();
    HttpResponse::Ok().json(data.clone())
}

async fn handle_get_by_id(state: web::Data<AppState>, web::Path(id): web::Path<u32>) -> impl Responder {
    let data = state.data.lock().unwrap();
    if let Some(item) = data.iter().find(|d| d.id == id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

async fn handle_post(state: web::Data<AppState>, item: web::Json<MyData>) -> impl Responder {
    let mut data = state.data.lock().unwrap();
    data.push(item.into_inner());
    HttpResponse::Created().body("Item added")
}

async fn handle_put(state: web::Data<AppState>, web::Path(id): web::Path<u32>, item: web::Json<MyData>) -> impl Responder {
    let mut data = state.data.lock().unwrap();
    if let Some(existing) = data.iter_mut().find(|d| d.id == id) {
        *existing = item.into_inner();
        HttpResponse::Ok().body("Item updated")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

async fn handle_delete(state: web::Data<AppState>, web::Path(id): web::Path<u32>) -> impl Responder {
    let mut data = state.data.lock().unwrap();
    if let Some(pos) = data.iter().position(|d| d.id == id) {
        data.remove(pos);
        HttpResponse::Ok().body("Item deleted")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        data: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(handle_get))
            .route("/", web::post().to(handle_post))
            .route("/{id}", web::get().to(handle_get_by_id))
            .route("/{id}", web::put().to(handle_put))
            .route("/{id}", web::delete().to(handle_delete))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
