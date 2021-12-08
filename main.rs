#[macro_use] extern crate serde;

use reqwest::Client;

mod codegen {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use self::codegen::client::Sendable;
use self::codegen::pet::Pet;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let _pets = Pet::<()>::list_pets().send(&client).await.unwrap();
}