
# Distributed Oracle protoype in Rust

This is Rust based aggregator and client that collects real time price of BTC in USD from the binance websocket API using multiple parallel clients, and computes the average to show in the terminal.

## What is this project about?
This project aims to create a Rust based distributed [Blockchain Oracle](https://chain.link/education/blockchain-oracles#:~:text=Blockchain%20oracles%20are%20entities%20that,outputs%20from%20the%20real%20world.), that provides data about the USD/BTC rates using the data from Binance exchange platform. To ensure security like any distributed systems, here each of the collector process (process that collects data from the exchange) signs the data it is recieving from the exchange. This signature is getting verified and only then the aggregator process accepts the data recieved. This is entirely built using Rust and uses concepts of distributed systems, computer networking, cryptographic signatures and verifications, and IPC protocols. 

## Run Locally

Clone the project

```bash
  git clone https://github.com/aritroCoder/RsOracle
```

Go to the project directory

```bash
  cd RsOracle
```

Run with a single command (make sure Rust is installed and device port 8000, 8080 is free)

```bash
bash run.sh
```

## Working

This project has a parent-child process structure where the parent is the aggregator, and child is the client. Two ports, 8000 and 8080 are used as the control port and data port respectively(similiar to how [FTP](https://datatracker.ietf.org/doc/html/rfc959) protocol works). The aggregator, when run spawns five instances of client processes, which are synchronized to start at the same tick of clock(upto microsecond accuracy). The client processes first generate a [ED25519](https://ed25519.cr.yp.to/) key-pair, and send the public key to the server using a TCP connection on the control port. The client processes then collect data from the binance API websocket independently 10 times, computes average, and sends the data to the server using a TCP stream opened at the data port, along with the signature appended with the data, all serialized into a byte array. Server, upon recieving the data, seperates the signature from it, deserializes and verifies the signature against the data and the public key already recieved, and, on successful verification, adds it to the list of values it recieves from the clients. At the end, it calculated the average of all the values in the data and shows the result in the terminal.
