use std::{net::TcpStream, io::Read};

use ed25519_dalek::{PUBLIC_KEY_LENGTH, VerifyingKey, SIGNATURE_LENGTH, Signature, Verifier};

/// Receives a verifying key from a child process and adds it to the key_list
pub fn tcp_recv_verify_key(mut stream: TcpStream, key_list: &mut Vec<VerifyingKey>) {
    let mut buffer = [0; PUBLIC_KEY_LENGTH];
    match stream.read(&mut buffer) {
        Ok(_size) => {
            let verifying_key = VerifyingKey::from_bytes(&buffer).unwrap();
            key_list.push(verifying_key);
        }
        Err(e) => {
            eprintln!("Error reading from child process: {}", e);
        }
    }
}

/// Receives a price data from a child process and adds it to the rates_list after verifying it's signature.
pub fn tcp_recv_price_data(mut stream: TcpStream, rates_list: &mut Vec<f64>, key_list: &[VerifyingKey]) {
    let mut buffer = [0; SIGNATURE_LENGTH + 8];

    match stream.read(&mut buffer) {
        Ok(_size) => {
            let received = f64::from_be_bytes(buffer[SIGNATURE_LENGTH..].try_into().unwrap_or_default());
            let signature = Signature::try_from(&buffer[..SIGNATURE_LENGTH]).unwrap();
            for key in key_list {
                if key.verify(received.to_be_bytes().as_ref(), &signature).is_ok() {
                    rates_list.push(received);
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading from child process: {}", e);
        }
    }
}
