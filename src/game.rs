use std::sync::mpsc::{channel, Receiver, Sender};

use anyhow::{anyhow, Result};

use crate::{
    game_rules::GameRules, game_stage::GameStage, player::Player, playmap::Playmap, point::Point,
    ship::Ship,
};

type PlayerId = u8;

pub struct Game {
    pub rules: GameRules,
    pub stage: GameStage,

    pub player_a: Player,
    pub player_b: Player,

    connection_count: PlayerId,
    connection_a: Option<PlayerId>,
    connection_b: Option<PlayerId>,

    sender_a: Option<Sender<()>>,
    sender_b: Option<Sender<()>>,
}

pub struct StateSnapshot {
    pub my_ships: Playmap,
    pub my_marks: Playmap,
    pub enemy_marks: Playmap,
    pub enemy_losses: Playmap,
}

struct Players<'a> {
    my_id: PlayerId,
    me: &'a mut Player,
    enemy_id: PlayerId,
    enemy: &'a mut Player,
}

impl Game {
    pub fn new(rules: GameRules) -> Self {
        return Game {
            rules: rules,
            stage: GameStage::Waiting,

            player_a: Player::new(),
            player_b: Player::new(),

            connection_count: 0,
            connection_a: None,
            connection_b: None,

            sender_a: None,
            sender_b: None,
        };
    }

    pub fn connect(&mut self) -> Result<(PlayerId, Receiver<()>)> {
        if self.connection_a.is_some() && self.connection_b.is_some() {
            return Err(anyhow!("both seats taken"));
        }

        self.connection_count += 1;

        let (sender, receiver) = channel();

        if self.connection_a.is_none() {
            self.connection_a = Some(self.connection_count);
            self.sender_a = Some(sender);
        } else {
            self.connection_b = Some(self.connection_count);
            self.sender_b = Some(sender);
        }

        if self.connection_a.is_some() && self.connection_b.is_some() {
            self.stage = GameStage::PlayerShips(None);
        }

        return Ok((self.connection_count, receiver));
    }

    pub fn disconnect(&mut self, my_id: PlayerId) -> Result<()> {
        if self.connection_a == Some(my_id) {
            self.connection_a = self.connection_b.take();
            self.sender_a = self.sender_b.take();
        } else if self.connection_b == Some(my_id) {
            self.connection_b = None;
            self.sender_b = None;
        } else {
            return Err(anyhow!("player not found"));
        }

        self.player_a = Player::new();
        self.player_b = Player::new();

        self.stage = GameStage::Waiting;

        return Ok(());
    }

    pub fn place_figure(&mut self, my_id: PlayerId, ship: Ship, point: Point) -> Result<()> {
        let stage = self.stage.clone();
        let rules = self.rules.clone();

        if stage != GameStage::PlayerShips(None) && stage != GameStage::PlayerShips(Some(my_id)) {
            return Err(anyhow!("cant place ships, wrong stage"));
        }

        let players = self.get_players(my_id)?;
        if rules.can_place_ship(players.me, ship) {
            players.me.place_figure(ship, point)?;
        } else {
            return Err(anyhow!("cant place ships above limit"));
        }

        if !rules.has_available_ships(players.me) {
            if stage == GameStage::PlayerShips(None) {
                self.stage = GameStage::PlayerShips(Some(players.enemy_id));
            } else {
                self.stage = GameStage::PlayerShoots(self.connection_a.unwrap());
            }
        }

        return Ok(());
    }

    pub fn remove_figure(&mut self, my_id: PlayerId, point: Point) -> Result<()> {
        if self.stage != GameStage::PlayerShips(None)
            && self.stage != GameStage::PlayerShips(Some(my_id))
        {
            return Err(anyhow!("cant remove ships, wrong stage"));
        }

        let players = self.get_players(my_id)?;
        players.me.remove_figure(point)?;

        return Ok(());
    }

    pub fn shoot(&mut self, my_id: PlayerId, point: Point) -> Result<()> {
        if self.stage != GameStage::PlayerShoots(my_id) {
            return Err(anyhow!("cant shoot, wrong turn or stage"));
        }

        let players = self.get_players(my_id)?;
        players.enemy.register_shot(point);

        if players.enemy.has_ship_at(point) {
            if !players.enemy.has_intact_ships() {
                self.stage = GameStage::PlayerWins(players.my_id);
            } else {
                self.stage = GameStage::PlayerShoots(players.my_id);
            }
        } else {
            self.stage = GameStage::PlayerShoots(players.enemy_id);
        }

        return Ok(());
    }

    pub fn get_state(&mut self, my_id: PlayerId) -> Result<StateSnapshot> {
        let players = self.get_players(my_id)?;

        return Ok(StateSnapshot {
            my_ships: players.me.ships,
            my_marks: players.me.shots,
            enemy_marks: players.enemy.shots,
            enemy_losses: players.enemy.get_hits(),
        });
    }

    pub fn trigger_sync(&self) -> Result<()> {
        if let Some(sender_a) = &self.sender_a {
            sender_a.send(())?;
        }
        if let Some(sender_b) = &self.sender_b {
            sender_b.send(())?;
        }

        return Ok(());
    }

    fn get_players(&mut self, my_id: PlayerId) -> Result<Players> {
        if self.connection_a == Some(my_id) && self.connection_b.is_some() {
            return Ok(Players {
                my_id: self.connection_a.unwrap(),
                me: &mut self.player_a,
                enemy_id: self.connection_b.unwrap(),
                enemy: &mut self.player_b,
            });
        } else if self.connection_b == Some(my_id) && self.connection_a.is_some() {
            return Ok(Players {
                my_id: self.connection_b.unwrap(),
                me: &mut self.player_b,
                enemy_id: self.connection_a.unwrap(),
                enemy: &mut self.player_a,
            });
        } else {
            return Err(anyhow!("player not found"));
        }
    }
}

#[cfg(test)]
mod test_game {
    use anyhow::Result;

    use crate::game_rules::GameRules;

    use super::Game;

    #[test]
    pub fn test_connect() -> Result<()> {
        let rules = GameRules::new();
        let mut game = Game::new(rules);

        let (connection_id, _) = game.connect()?;
        assert_eq!(connection_id, 1);

        let (connection_id, _) = game.connect()?;
        assert_eq!(connection_id, 2);

        assert!(game.connect().is_err());

        return Ok(());
    }

    #[test]
    pub fn test_disconnect() -> Result<()> {
        let rules = GameRules::new();
        let mut game = Game::new(rules);

        let (connection_a, _) = game.connect()?;
        let (connection_b, _) = game.connect()?;

        game.disconnect(connection_a)?;

        assert_eq!(game.connection_a, Some(connection_b));
        assert_eq!(game.connection_b, None);

        return Ok(());
    }

    #[test]
    pub fn test_trigger_sync() -> Result<()> {
        let mut game = Game::new(GameRules::new());

        let (_, receiver_a) = game.connect()?;
        let (_, receiver_b) = game.connect()?;

        game.trigger_sync()?;

        assert_eq!(receiver_a.try_iter().count(), 1);
        assert_eq!(receiver_b.try_iter().count(), 1);

        return Ok(());
    }
}
