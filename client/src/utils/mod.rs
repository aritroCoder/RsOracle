use std::fs::OpenOptions;
use std::io::{self, Write};
use std::net::TcpStream;
use std::{thread::sleep, time::Duration};

use tungstenite::Message;
use serde_json::from_str;
use uuid::Uuid;

use crate::data_models::SocketResponse;

pub fn write_to_file(filename: &str, numbers: &[f64]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;

    for &num in numbers {
        writeln!(file, "{}", num)?;
    }
    Ok(())
}

/// Query the socket connected with `socket` for the price of BTCUSDC, and push the price to `ex_rates`.
pub fn send_sock_msg(socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>, ex_rates: &mut Vec<f64>) {
    let id = Uuid::new_v4();
    let response = socket.write_message(Message::Text(r#"{
            "id": ""#.to_string() + &id.to_string() + r#"",
            "method": "ticker.price",
            "params": {
                "symbol": "BTCUSDC"
            }
        }"#.into()));
    if let Err(e) = response {
        println!("Error sending message: {}", e);
    }
    let msg = socket.read_message().expect("Error reading message");
    let response_parsed = from_str::<SocketResponse>(&msg.to_string());
    match response_parsed {
        Ok(parsed) => {
            let ex_rate = parsed.result.price.parse::<f64>().unwrap_or_default();
            ex_rates.push(ex_rate);
        },
        Err(e) => {
            println!("Error parsing message: {}", e);
        }
    }
    sleep(Duration::from_secs(1));
}