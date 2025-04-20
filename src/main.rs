use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use types::blockchain::BlockChain;
pub mod types;

use types::web_types::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new(Mutex::new(BlockChain::new())));

    const PORT: &str = "127.0.0.1:8080";
    println!("on port {}", PORT);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/", web::get().to(ping))
            .route("/init", web::get().to(initsialize_or_500))
            .route("/mine", web::get().to(mine_or_500))
    })
    .bind(PORT)?
    .run()
    .await
}

async fn initsialize_or_500(data: web::Data<AppState>) -> impl Responder {
    let mut bc = data.blockchain.lock().unwrap();
    bc.initiate().unwrap();

    HttpResponse::Ok().json(bc.clone())
}

async fn mine_or_500(data: web::Data<AppState>) -> impl Responder {
    let mut bc = data.blockchain.lock().unwrap();
    match bc.mine() {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
async fn ping(data: web::Data<AppState>) -> impl Responder {
    let bc = data.blockchain.lock().unwrap();

    HttpResponse::Ok().json(format!("Hellow {:#?}", bc))
}
