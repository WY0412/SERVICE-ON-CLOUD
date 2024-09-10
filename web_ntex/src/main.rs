

use ntex::web;
use serde::Deserialize;
extern crate base64;

// import verify_json from verify_blockchain.rs
mod verify_blockchain;
#[derive(Deserialize)]
struct Info {
    cc: String,
}

// test connection only
#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

// respond to blockchain request
#[web::get("/blockchain")]
async fn add_new_tx(raw_data: web::types::Query<Info>) -> impl web::Responder {
    let encoded_json = &raw_data.cc;
    let response_str = verify_blockchain::validate_encoded_json(encoded_json);
    web::HttpResponse::Ok().body(&response_str)
    // validate 
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //create http server and bind urls
    println!("Starting HTTP server...");

    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(add_new_tx)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}