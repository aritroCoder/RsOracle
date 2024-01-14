use std::io::Write;
use std::net::TcpStream;
use std::{thread::sleep, time::Duration};

use clap::Parser;
use ed25519_dalek::{Signer, SigningKey, SIGNATURE_LENGTH};
use rand::rngs::OsRng;
use tungstenite::connect;
use url::Url;

use data_models::Args;
use utils::send_sock_msg;

pub mod data_models;
pub mod utils;

const SOCK_ADDR: &str = "wss://ws-api.binance.com:443/ws-api/v3";

/// Connect to socket, and send signed data to aggregator.
fn cache(argv: Args, signing_key: SigningKey) {
    let (mut socket, _response) = connect(Url::parse(SOCK_ADDR).unwrap()).expect("Can't connect");
    let mut ex_rates: Vec<f64> = vec![];
    for _i in 1..=argv.times {
        send_sock_msg(&mut socket, &mut ex_rates);
    }
    let avg_price = ex_rates.iter().sum::<f64>() / ex_rates.len() as f64;
    ex_rates.push(avg_price);
    let signature = signing_key.sign(avg_price.to_be_bytes().as_ref());
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to parent");
    let mut data: [u8; SIGNATURE_LENGTH + 8] = [0; SIGNATURE_LENGTH + 8]; // first 64 bytes for signature, last 8 bytes for avg_price

    data[..SIGNATURE_LENGTH].clone_from_slice(&signature.to_bytes());
    data[SIGNATURE_LENGTH..].clone_from_slice(&avg_price.to_be_bytes());

    stream
        .write_all(data.as_ref())
        .expect("Failed to send message");
    let close_response = socket.close(None);
    if let Err(e) = close_response {
        println!("Error closing the socket: {}", e);
    }
}

fn main() {
    let argv = Args::parse();
    // start only when the start time is reached
    let now = chrono::Utc::now().timestamp();
    if now < argv.start {
        let sleep_time = argv.start - now;
        sleep(Duration::from_secs(sleep_time as u64));
    }
    // println!("Starting the client at {}", argv.start);
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Failed to connect to parent");
    stream
        .write_all(signing_key.verifying_key().as_bytes())
        .expect("Failed to send public key to aggregator");
    match argv.mode {
        ref mode if mode == "cache" => cache(argv, signing_key),
        _ => println!("Invalid mode. Please use 'cache'"),
    }
}
