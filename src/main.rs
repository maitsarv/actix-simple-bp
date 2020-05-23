#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use crate::server::server;

mod auth;
mod config;
mod database;
pub mod handlers;
mod middleware;
mod models;
mod routes;
mod server_helpers;
mod server;
mod tests;
mod validate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
