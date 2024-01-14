use std::{process::Command, net::TcpListener};

use tcp_utils::{tcp_recv_price_data, tcp_recv_verify_key};

mod tcp_utils;

/// make sure ports 8000 and 8080 are free before running this.
fn main(){
    let mut child_handlers = vec![];
    let key_listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    let start_time = chrono::Utc::now().timestamp() + 1; // Start all child processes in 1 second from now

    for _ in 0..5 {
        let child = Command::new("./client")
            .arg(format!("--start={start_time}"))
            .arg("--mode=cache")
            .arg("--times=10")
            .spawn()
            .expect("failed to execute client. Make sure the client executable is at the root of the program.");

        child_handlers.push(child);
    }

    let mut key_list = vec![];
    for stream in key_listener.incoming() {
        match stream {
            Ok(stream) => {
                tcp_recv_verify_key(stream, &mut key_list);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
        if key_list.len() == 5 {
            break;
        }
    }

    let mut rates_list: Vec<f64> = vec![];
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tcp_recv_price_data(stream, &mut rates_list, &key_list);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
        if rates_list.len() == 5 {
            break;
        }
    }
    
    for mut child in child_handlers {
        let ecode = child.wait()
            .expect("failed to wait on child");
        assert!(ecode.success());
    }

    let avg_rate = rates_list.iter().sum::<f64>() / rates_list.len() as f64;
    println!("Average USD price of BTC is: {}", avg_rate);
}
