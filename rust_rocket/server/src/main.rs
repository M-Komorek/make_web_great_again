use std::time::Instant;

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Transaction {
    id: u32,
    sender: String,
    recipient: String,
    framework: String,
}

#[get("/transactions")]
fn transactions() -> Json<Vec<Transaction>> {
    let mut transactions = Vec::with_capacity(101);
    for i in 1..101 {
        transactions.push(Transaction {
            id: i,
            sender: format!("Anonymous Sender {}", i),
            recipient: format!("Anonymous Recipient {}", i),
            framework: format!("Rust Rocket"),
        })
    }

    Json(transactions)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResponseBack {
    result: String,
    performance: String,
}

#[get("/fib?<n>")]
fn fib(n: u128) -> Json<ResponseBack> {
    let now = Instant::now();
    let mut f0: u128 = 0;
    let mut f1: u128 = 1;

    for _ in 0..n {
        let f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
    }

    Json(ResponseBack {
        result: f0.to_string(),
        performance: format!(
            "{}s and {}ns",
            now.elapsed().as_secs(),
            now.elapsed().as_nanos(),
        ),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![transactions, fib])
}
