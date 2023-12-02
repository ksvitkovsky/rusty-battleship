pub mod game;
pub mod game_rules;
pub mod game_stage;
pub mod player;
pub mod playmap;
pub mod point;
pub mod ship;

use std::{net::TcpListener, thread::spawn};

use tungstenite::{accept, Message};

pub fn main() {
    let server = TcpListener::bind("localhost:9001").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                match msg {
                    Message::Binary(bin) => {
                        println!("incoming msg {:} bytes", bin.len());

                        websocket.send(Message::Binary(bin)).unwrap()
                    }
                    Message::Close(_) => {}
                    _ => {}
                }
            }
        });
    }
}
