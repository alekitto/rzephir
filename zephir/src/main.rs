#![feature(in_band_lifetimes)]
#[macro_use]
extern crate lazy_static;

mod err;
mod handlers;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use libzephir::storage::StorageManager;

fn get_serve_port() -> u16 {
    let serve_port = std::env::var("SERVE_PORT");
    match serve_port {
        Result::Err(_) => 8091,
        Result::Ok(serve_port) => {
            if serve_port.is_empty() {
                8091
            } else {
                serve_port.parse().unwrap_or(8091)
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@localhost:30042/zephir")
        .await
        .unwrap();

    let storage_manager = StorageManager::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(storage_manager.clone())
            .wrap(Logger::default())
            .service(handlers::get_status)
            .service(handlers::allowed_action)
            .service(handlers::get_group)
            .service(handlers::get_group_identities)
            .service(handlers::patch_group_identities)
            .service(handlers::upsert_group)
            .service(handlers::get_identity)
            .service(handlers::upsert_identity)
            .service(handlers::get_policy)
            .service(handlers::upsert_policy)
    })
    .bind(("0.0.0.0", get_serve_port()))?
    .run()
    .await
    //
    // let manager = StorageManager::new(pool);
    // let identity = manager
    //     .find_identity("urn:giocaresport::::identity:aec8b9dd-84a3-409f-aa44-72b991463ab6")
    //     .await
    //     .unwrap();
    //
    // let identity = manager
    //     .find_identity("urn:giocaresport::::identity:aec8b9dd-84a3-409f-aa44-72b991463ab6")
    //     .await
    //     .unwrap();
    //
    // let identity = manager
    //     .find_identity("urn:giocaresport::::identity:aa9f5701-4729-4e9c-a694-f97fc4d39a94")
    //     .await
    //     .unwrap();
    //
    // println!("{:#?}", identity.unwrap().allowed::<&str, String>(Option::Some("core:GetSport"), Option::None));
}
