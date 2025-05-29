use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::u8;
use lazy_static::lazy_static;

static mut CONT:u8 = 0;
lazy_static! {
    static ref URL_MAP: Mutex<HashMap<u8, String>> = Mutex::new(HashMap::new());
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
     
    unsafe {
        let short_id:u8  = CONT + 1;
        CONT += 1;
        map.insert(short_id, url_data.url.clone());
        HttpResponse::Ok().json(UrlOutput {short: format!("->{}", short_id),})
    }
}

async fn redirecionar(path: web::Path<String>) -> impl Responder {
    let map = URL_MAP.lock().unwrap();
    let end = path.parse::<u8>().expect("ero na conversão para u8");
    if let Some(url) = map.get(&end) {
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
