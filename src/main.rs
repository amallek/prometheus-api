#[macro_use]
extern crate actix_web;

mod constants;
mod httpd;
mod prometheus;
mod security;

#[actix_web::main]
async fn main() {
    println!(
        "Booting up... v{}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("NOT_FOUND")
    );
    httpd::serve().await.expect("Error starting server");
}
