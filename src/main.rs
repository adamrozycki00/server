#![allow(dead_code)]

mod http;
mod server;
mod website_handler;

use http::{Method, Request};
use server::Server;
use std::env;
use std::fmt::format;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // env! is used for reading the envs in compile time
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path={}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
