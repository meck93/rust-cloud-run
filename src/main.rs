mod health;
mod index;

use actix_web::{App, HttpServer};
use health::get_health;
use index::get_index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_index).service(get_health))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
