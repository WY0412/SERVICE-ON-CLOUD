use ntex::web;
use serde::Deserialize;
use std::env;
use sqlx::mysql::{MySqlConnectOptions,MySqlPool};

const MALFORMED_RESPONSE: &str = "CCprojectFor3,851725245278\nINVALID";

mod twitter;
mod kmp;

#[derive(Deserialize)]
struct Info {
    user_id: Option<i64>,
    r#type: Option<String>,
    phrase: Option<String>,
    hashtag: Option<String>,
}

// test connection only
#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

// static mut CONN_GLOBAL: Option<MySqlPool> = None;

// respond to twitter request
#[web::get("/twitter")]
async fn do_job(
    raw_data: web::types::Query<Info>,
    pool: web::types::State<MySqlPool>,
) -> impl web::Responder {
    let user_id = raw_data.user_id;
    let tweet_type = &raw_data.r#type;
    let phrase = &raw_data.phrase;
    let hashtag = &raw_data.hashtag;
    if user_id.is_none() || tweet_type.is_none() || phrase.is_none() || hashtag.is_none() {
        return web::HttpResponse::Ok().body(MALFORMED_RESPONSE);
    }

    // let pool: &MySqlPool = unsafe { CONN_GLOBAL.as_mut().unwrap()};

    let response_str = twitter::calculate_ranking_score(&pool, user_id.unwrap(),  &tweet_type.as_ref().unwrap(), &phrase.as_ref().unwrap(), &hashtag.as_ref().unwrap());
    return web::HttpResponse::Ok().body(&response_str.await.unwrap());
    // validate 
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //create http server and bind urls
    println!("Starting HTTP server...");

    let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set");
    let host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set");
    let user = env::var("MYSQL_USER").expect("MYSQL_USER is not set");
    let database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE is not set");

    print!("password: {}, host: {}, user: {}, database: {}", password, host, user, database);

    let opt = MySqlConnectOptions::new().host(&*host).username(&*user).password(&*password).database(&*database);
    let pool: MySqlPool = MySqlPool::connect_with(opt).await.unwrap();

    // unsafe {
    //     CONN_GLOBAL = Some(pool);
    // }

    web::HttpServer::new(move || {
        web::App::new()
            .state(pool.clone())
            .service(hello)
            .service(do_job)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}