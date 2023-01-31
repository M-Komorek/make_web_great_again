use actix_web::{get, web, HttpResponse, Result};
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
struct Transaction {
    id: u32,
    sender: String,
    recipient: String,
    framework: String,
}

#[get("/transactions")]
async fn transactions() -> Result<HttpResponse> {
    let mut transactions = Vec::with_capacity(101);
    for i in 1..101 {
        transactions.push(Transaction {
            id: i,
            sender: format!("Anonymous Sender {}", i),
            recipient: format!("Anonymous Recipient {}", i),
            framework: format!("Rust Actix-Web"),
        })
    }

    Ok(HttpResponse::Ok().json(transactions))
}

#[derive(Serialize)]
struct ResponseBack {
    result: String,
    performance: String,
}

#[get("/fib/{n_str}")]
async fn fib(n_str: web::Path<String>) -> Result<HttpResponse> {
    let now = Instant::now();
    let mut f0: u128 = 0;
    let mut f1: u128 = 1;

    let n = n_str.parse::<u128>().expect("Should be valid u128!");
    for _ in 0..n {
        let f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
    }

    Ok(HttpResponse::Ok().json(ResponseBack {
        result: f0.to_string(),
        performance: format!(
            "{}s and {}ns",
            now.elapsed().as_secs(),
            now.elapsed().as_nanos(),
        ),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(move || App::new().service(transactions).service(fib))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
