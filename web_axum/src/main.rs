use axum::{
    routing::get,
    extract::Query,
    http::StatusCode,
    Router,
};
use axum::http::header;
use serde::{Deserialize, Serialize};
use axum::response::Response;

extern crate base64;
mod verify_blockchain;
#[derive(Deserialize)]
struct Info {
    cc: String,
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
async fn add_new_tx(Query(params): Query<Info>) -> Response<String> {
    let encoded_json = &params.cc;
    let response_str = verify_blockchain::validate_encoded_json(encoded_json);
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(response_str)
        .unwrap() // In real code, handle errors properly
}
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/blockchain", get(add_new_tx));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

