use crate::constants::{TLS_CERT_FILE, TLS_KEY_FILE};
use crate::prometheus;
use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

use crate::security;

pub async fn serve() -> Result<(), std::io::Error> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls_server()).unwrap();
    builder
        .set_private_key_file(TLS_KEY_FILE, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(TLS_CERT_FILE).unwrap();
    return HttpServer::new(|| {
        App::new()
            .wrap(security::Default)
            .service(prometheus::info)
            .service(prometheus::config_get)
            .service(prometheus::endpoint_get)
            .service(prometheus::endpoint_add)
            .service(prometheus::endpoint_delete)
            .service(prometheus::endpoint_update)
    })
    .bind_openssl("0.0.0.0:9090", builder)
    .unwrap()
    .run()
    .await;
}
