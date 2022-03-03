extern crate openssl;
#[macro_use]
extern crate diesel;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn accounts() -> impl Responder {
    "Accounts"
}

async fn blocks() -> impl Responder {
    "Blocks"
}

async fn transactions() -> impl Responder {
    "Transactions"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/accounts", web::get().to(accounts))
                    .route("/blocks", web::get().to(blocks))
                    .route("/transactions", web::get().to(transactions))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
