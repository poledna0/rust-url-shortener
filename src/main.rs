use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::u8;
use lazy_static::lazy_static;
use std::env;

static mut CONT: u8 = 0;
static mut CONTA: u8 = 100;

lazy_static! {
    static ref URL_MAP: Mutex<HashMap<u8, String>> = Mutex::new(HashMap::new());
}

#[derive(Deserialize)]
struct UrlInput {
    url: String,
    senha: Option<String>,
    duracao_horas: u8,
}


#[derive(Serialize)]
struct UrlOutput {
    short: String,
}
fn make(cont: u8, url_data: &web::Json<UrlInput>) -> UrlOutput{

    let mut map = URL_MAP.lock().unwrap();
    let short_id:u8  = cont + 1;
    map.insert(short_id, url_data.url.clone());
    let short = format!("IP + {}", short_id);
    let saida = UrlOutput { short };
    saida
}
async fn encurtar(url_data: web::Json<UrlInput>) -> impl Responder {
    let senhaenv = env::var("MASTER_KEY").expect(" MASTER_KEY=password !<- .env ");

    if url_data.duracao_horas <= 24 {
        if let Some(senha) = &url_data.senha {
            if senha == &senhaenv {
                unsafe {
                    CONTA += 1;
                    let saida: UrlOutput = make(CONTA, &url_data);
                    return  HttpResponse::Ok().json(saida)
                }
            }
        }
        unsafe {
            CONT += 1;
            let saida: UrlOutput = make(CONT, &url_data);
            return HttpResponse::Ok().json(saida)
        }
    }
    HttpResponse::BadRequest().body(" err time > 24")
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

    dotenv::dotenv().ok();
    
    HttpServer::new(|| {
        App::new()
            .route("/encurtar", web::post().to(encurtar))
            .route("/{id}", web::get().to(redirecionar))
    })
    .bind(("0.0.0.0", 9999))?
    .run()
    .await
}
