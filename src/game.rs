use anyhow::{anyhow, Result};

use crate::{player::Player, playmap::Playmap, point::Point, ship::Ship};

type PlayerId = u8;

#[derive(PartialEq)]
#[repr(u8)]
pub enum GameStage {
    Waiting = 1,
    PlayerShips(Option<PlayerId>) = 2,
    PlayerShoots(PlayerId) = 3,
    PlayerWins(PlayerId) = 4,
}

pub struct Game {
    pub stage: GameStage,

    pub player_a: Player,
    pub player_b: Player,

    connection_count: u8,
    connection_a: Option<u8>,
    connection_b: Option<u8>,
}

pub struct StateSnapshot {
    pub my_ships: Playmap,
    pub my_marks: Playmap,
    pub enemy_marks: Playmap,
    pub enemy_losses: Playmap,
}

struct Players<'a> {
    me: &'a mut Player,
    enemy: &'a mut Player,
}

impl Game {
    pub fn new() -> Self {
        return Game {
            stage: GameStage::Waiting,

            player_a: Player::new(),
            player_b: Player::new(),

            connection_count: 0,
            connection_a: None,
            connection_b: None,
        };
    }

    pub fn connect(&mut self) -> Result<u8> {
        if self.connection_a.is_some() && self.connection_b.is_some() {
            return Err(anyhow!("both seats taken"));
        }

        self.connection_count += 1;

        if self.connection_a.is_none() {
            self.connection_a = Some(self.connection_count);
        } else {
            self.connection_b = Some(self.connection_count);
        }

        return Ok(self.connection_count);
    }

    pub fn disconnect(&mut self, my_connection_id: u8) -> Result<()> {
        if self.connection_a == Some(my_connection_id) {
            self.connection_a = self.connection_b;
            self.connection_b = None;
        } else if self.connection_b == Some(my_connection_id) {
            self.connection_b = None;
        } else {
            return Err(anyhow!("player not found"));
        }

        self.player_a = Player::new();
        self.player_b = Player::new();

        self.stage = GameStage::Waiting;

        return Ok(());
    }

    pub fn place_figure(&mut self, my_connection_id: u8, ship: Ship, point: Point) -> Result<()> {
        if self.stage != GameStage::PlayerShips(None)
            && self.stage != GameStage::PlayerShips(Some(my_connection_id))
        {
            return Err(anyhow!("cant place ships, wrong stage"));
        }

        let players = self.get_players(my_connection_id)?;
        players.me.place_figure(ship, point)?;

        return Ok(());
    }

    pub fn remove_figure(&mut self, my_connection_id: u8, point: Point) -> Result<()> {
        if self.stage != GameStage::PlayerShips(None)
            && self.stage != GameStage::PlayerShips(Some(my_connection_id))
        {
            return Err(anyhow!("cant remove ships, wrong stage"));
        }

        let players = self.get_players(my_connection_id)?;
        players.me.remove_figure(point)?;

        return Ok(());
    }

    pub fn shoot(&mut self, my_connection_id: u8, point: Point) -> Result<()> {
        if self.stage != GameStage::PlayerShoots(my_connection_id) {
            return Err(anyhow!("cant shoot, wrong turn or stage"));
        }

        let players = self.get_players(my_connection_id)?;
        players.me.register_shot(point);

        return Ok(());
    }

    pub fn get_state(&mut self, my_connection_id: u8) -> Result<StateSnapshot> {
        let Players { me, enemy } = self.get_players(my_connection_id)?;

        return Ok(StateSnapshot {
            my_ships: me.ships,
            my_marks: me.shots,
            enemy_marks: enemy.shots,
            enemy_losses: enemy.get_hits(),
        });
    }

    fn get_players(&mut self, my_connection_id: u8) -> Result<Players> {
        if self.connection_a == Some(my_connection_id) {
            return Ok(Players {
                me: &mut self.player_a,
                enemy: &mut self.player_b,
            });
        } else if self.connection_b == Some(my_connection_id) {
            return Ok(Players {
                me: &mut self.player_b,
                enemy: &mut self.player_a,
            });
        } else {
            return Err(anyhow!("player not found"));
        }
    }
}
