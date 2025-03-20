use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: usize,
    name: String,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

async fn add_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Created().finish()
}

async fn update_item(data: web::Data<AppState>, path: web::Path<usize>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let id = path.into_inner();
    if let Some(existing_item) = items.iter_mut().find(|i| i.id == id) {
        existing_item.name = item.name.clone();
        return HttpResponse::Ok().json(existing_item);
    }
    HttpResponse::NotFound().finish()
}

async fn delete_item(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let id = path.into_inner();
    if let Some(pos) = items.iter().position(|i| i.id == id) {
        items.remove(pos);
        return HttpResponse::NoContent().finish();
    }
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        items: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(|| async { "Welcome to Rust REST API!" }))
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(add_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
