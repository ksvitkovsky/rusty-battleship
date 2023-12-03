pub mod game;
pub mod game_rules;
pub mod game_stage;
pub mod player;
pub mod playmap;
pub mod point;
pub mod ship;

use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread::spawn,
};

use anyhow::Result;
use game::Game;
use game_rules::GameRules;
use point::Point;
use ship::Ship;
use tungstenite::{accept, Message};

pub fn main() {
    let server = TcpListener::bind("localhost:9001").unwrap();
    let game = Arc::new(Mutex::new(Game::new(GameRules::new())));

    for stream in server.incoming() {
        let player_game = game.clone();

        spawn(move || -> Result<()> {
            let mut websocket = accept(stream.unwrap()).unwrap();

            let mut game_lock = player_game.lock().unwrap();
            let (my_id, receiver) = game_lock.connect()?;

            drop(game_lock);

            loop {
                if let Ok(_) = receiver.try_recv() {
                    let mut game_lock = player_game.lock().unwrap();

                    let stage = game_lock.stage.clone();
                    let state = game_lock.get_state(my_id)?;

                    let mut payload: Vec<u8> = Vec::new();
                    payload.push(stage.into_u8(my_id)?);
                    payload.extend(state.my_ships.value.to_be_bytes());
                    payload.extend(state.my_marks.value.to_be_bytes());
                    payload.extend(state.enemy_marks.value.to_be_bytes());
                    payload.extend(state.enemy_losses.value.to_be_bytes());

                    websocket.send(Message::Binary(payload))?;

                    drop(game_lock);
                }

                let msg = websocket.read().unwrap();

                match msg {
                    Message::Binary(bin) => {
                        match bin[..] {
                            // player places a ship
                            [1, ship_u8, point_u8] => {
                                let mut game_lock = player_game.lock().unwrap();

                                game_lock.place_figure(
                                    my_id,
                                    Ship::try_from(ship_u8)?,
                                    Point::from_u8(point_u8)?,
                                )?;

                                game_lock.trigger_sync()?;
                                drop(game_lock);
                            }
                            // player removes a ship
                            [2, point_u8] => {
                                let mut game_lock = player_game.lock().unwrap();

                                game_lock.remove_figure(my_id, Point::from_u8(point_u8)?)?;

                                game_lock.trigger_sync()?;
                                drop(game_lock);
                            }
                            // player shoots at point
                            [3, point_u8] => {
                                let mut game_lock = player_game.lock().unwrap();

                                game_lock.shoot(my_id, Point::from_u8(point_u8)?)?;

                                game_lock.trigger_sync()?;
                                drop(game_lock);
                            }
                            _ => {}
                        }
                    }
                    Message::Close(_) => {
                        let mut game_lock = player_game.lock().unwrap();
                        game_lock.disconnect(my_id)?;

                        drop(game_lock);
                    }
                    _ => {}
                }
            }
        });
    }
}
