use crate::{player::Player, ship::Ship};

#[derive(Clone, Copy)]
pub struct GameRules {
    pub submarine_limit: u8,
    pub destroyer_limit: u8,
    pub cruiser_limit: u8,
    pub battleship_limit: u8,
}

impl GameRules {
    pub fn new() -> Self {
        return GameRules {
            submarine_limit: 4,
            destroyer_limit: 3,
            cruiser_limit: 2,
            battleship_limit: 1,
        };
    }

    pub fn can_place_ship(&self, player: &Player, ship: Ship) -> bool {
        return match ship {
            Ship::Submarine => player.submarines < self.submarine_limit,
            Ship::Destroyer(_) => player.destroyers < self.destroyer_limit,
            Ship::Cruiser(_) => player.cruisers < self.cruiser_limit,
            Ship::Battleship(_) => player.battleships < self.battleship_limit,
        };
    }

    pub fn get_limit(&self, ship: Ship) -> u8 {
        return match ship {
            Ship::Submarine => self.submarine_limit,
            Ship::Destroyer(_) => self.destroyer_limit,
            Ship::Cruiser(_) => self.cruiser_limit,
            Ship::Battleship(_) => self.destroyer_limit,
        };
    }

    pub fn has_available_ships(&self, player: &Player) -> bool {
        return player.submarines < self.submarine_limit
            && player.destroyers < self.destroyer_limit
            && player.cruisers < self.cruiser_limit
            && player.battleships < self.battleship_limit;
    }
}
