use std::net::{TcpListener, TcpStream};
use tungstenite::accept;
use tungstenite::protocol::Message;

use crate::actions::{add_time, subtract_time, reset_lane, reset_all, status};

fn handle_connection(stream: TcpStream) {
    let mut ws_stream = accept(stream).expect("Error during WebSocket handshake");

    loop {
        let msg = ws_stream.read().expect("Error reading message");

        match msg {            
            Message::Text(text) => {
                let response_message = String::new();

                if text.starts_with("/add_time") {
                    let response_message = text.trim_start_matches("/add_time").trim();
                    let lane = response_message.parse().expect("Not a valid number");
                    add_time(lane);

                    let response = format!("Processed: {}", response_message);
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");
                } else if text.starts_with("/subtract_time") {
                    let response_message = text.trim_start_matches("/subtract_time").trim();
                    let lane = response_message.parse().expect("Not a valid number");
                    subtract_time(lane);

                    let response = format!("Processed: {}", response_message);
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");
                } else if text.starts_with("/reset_lane") {
                    let response_message = text.trim_start_matches("/reset_lane").trim();
                    let lane = response_message.parse().expect("Not a valid number");
                    reset_lane(lane);

                    let response = format!("Processed: {}", response_message);
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");
                } else if text.starts_with("/reset_all") {

                    reset_all();
                    let response = format!("Processed: {}", response_message);
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");
                } else if text.starts_with("/status") {
                    let response = status();
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");

                    let response = format!("Processed: {}", response_message);
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");
                }
            }
            Message::Close(_) => {
                break;
            }
            _ => {}
        }
    }
}

pub fn websocket_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind");

    println!("WebSocket server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {:?}", stream.peer_addr());
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
