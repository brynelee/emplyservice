#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
//log4rs for logging
#[macro_use]
extern crate log;
extern crate log4rs;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod employees;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    log4rs::init_file("log.yaml",Default::default()).unwrap();
    info!("emplyservice is starting...");

    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
