


use ntex::web;
use serde::Deserialize;

extern crate base64;

// import verify_json from verify_blockchain.rs
mod encode_qrcode;
mod decode_qrcode;
mod utils;
mod matrix;

#[derive(Deserialize)]
struct Info {
    r#type: String,
    data: String,
    // timestamp is optional
    timestamp: Option<String>,
}


static mut client: Option<reqwest::Client> = None;
// test connection only
#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

// respond to blockchain request
#[web::get("/qrcode")]
async fn add_new_tx(raw_data: web::types::Query<Info>) -> impl web::Responder {
    let start = std::time::Instant::now();
    if raw_data.r#type == "encode" {
        // record time used to encode
        let encoded_json = encode_qrcode::encode(&raw_data.data);
        let elapsed = start.elapsed();
        println!("Time used to Encode: {:?}", elapsed);
        return web::HttpResponse::Ok().body(&encoded_json);
    }
    else if raw_data.r#type == "decode" {
        let timestamp = raw_data.timestamp.clone().unwrap();
        let my_client: &reqwest::Client;
        unsafe {
            my_client = client.as_ref().unwrap();
        }
        let verify_response = decode_qrcode::get_verify_response(my_client, &raw_data.data, &timestamp );
        let res = verify_response.await.unwrap();
        println!("Time used to Decode: {:?}", start.elapsed());
        return web::HttpResponse::Ok().body(&res);
    } else {
        return web::HttpResponse::Ok().body("Invalid request type");
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //create http server and bind urls
    println!("Starting HTTP server...");

    unsafe {
        client = Some(reqwest::Client::new());
    }

    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(add_new_tx)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}