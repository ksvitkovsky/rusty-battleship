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

    pub fn has_available_ships(&self, player: &Player) -> bool {
        return player.submarines < self.submarine_limit
            || player.destroyers < self.destroyer_limit
            || player.cruisers < self.cruiser_limit
            || player.battleships < self.battleship_limit;
    }
}

#[cfg(test)]
mod test_game_rules {
    use crate::game_rules::GameRules;
    use crate::player::Player;
    use crate::point::Orientation;
    use crate::ship::Ship;

    #[test]
    pub fn test_can_place_ship() {
        let rules = GameRules::new();

        let mut player = Player::new();
        player.submarines = 3;
        player.destroyers = 3;
        player.cruisers = 2;
        player.battleships = 1;

        assert_eq!(rules.can_place_ship(&player, Ship::Submarine), true);

        let cruiser = Ship::Cruiser(Orientation::Horizontal);
        assert_eq!(rules.can_place_ship(&player, cruiser), false);
    }

    #[test]
    pub fn test_has_available_ships(){
        let rules = GameRules::new();

        let mut player = Player::new();
        player.submarines = 3;
        player.destroyers = 3;
        player.cruisers = 2;
        player.battleships = 1;

        assert_eq!(rules.has_available_ships(&player), true);

        player.submarines = 4;
        assert_eq!(rules.has_available_ships(&player), false);
    }
}
