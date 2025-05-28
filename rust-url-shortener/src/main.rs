use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use rand::{distributions::Alphanumeric, Rng};
use lazy_static::lazy_static;

lazy_static! {
    static ref URL_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[derive(Deserialize)]
struct UrlInput {
    url: String,
}

#[derive(Serialize)]
struct UrlOutput {
    short: String,
}
async fn encurtar(url_data: web::Json<UrlInput>) -> impl Responder {
    let mut map = URL_MAP.lock().unwrap();
    let short_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    map.insert(short_id.clone(), url_data.url.clone());

    HttpResponse::Ok().json(UrlOutput {
        short: format!("->{}", short_id),
    })
}

async fn redirecionar(path: web::Path<String>) -> impl Responder {
    let map = URL_MAP.lock().unwrap();
    if let Some(url) = map.get(&path.into_inner()) {
        HttpResponse::Found()
            .append_header(("Location", url.clone()))
            .finish()
    } else {
        HttpResponse::NotFound().body("URL não encontrada")
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/encurtar", web::post().to(encurtar))
            .route("/{id}", web::get().to(redirecionar))
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
