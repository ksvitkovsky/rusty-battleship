use anyhow::{anyhow, Result};

use crate::{
    playmap::Playmap,
    point::{Orientation, Point},
    ship::{Ship, BATTLESHIP_SIZE, CRUISER_SIZE, DESTROYER_SIZE, SUBMARINE_SIZE},
};

pub struct Player {
    pub ships: Playmap,
    pub shots: Playmap,

    pub submarines: u8,
    pub destroyers: u8,
    pub cruisers: u8,
    pub battleships: u8,
}

impl Player {
    pub fn new() -> Self {
        return Player {
            ships: Playmap::new(),
            shots: Playmap::new(),

            submarines: 0,
            destroyers: 0,
            cruisers: 0,
            battleships: 0,
        };
    }

    pub fn has_ship_at(&self, point: Point) -> bool {
        return self.ships.is_marked_field(point);
    }

    pub fn get_hits(&self) -> Playmap {
        let value = self.ships.value & self.shots.value;
        return Playmap { value };
    }

    pub fn has_intact_ships(&self) -> bool {
        return self.get_hits().value.count_ones() < self.ships.value.count_ones();
    }

    pub fn place_figure(&mut self, ship: Ship, point: Point) -> Result<()> {
        for point in ship.get_points(point)? {
            self.ships.mark_field(point);
        }

        match ship {
            Ship::Submarine => self.submarines += 1,
            Ship::Destroyer(_) => self.destroyers += 1,
            Ship::Cruiser(_) => self.cruisers += 1,
            Ship::Battleship(_) => self.battleships += 1,
        };

        return Ok(());
    }

    pub fn remove_figure(&mut self, point: Point) -> Result<()> {
        if !self.ships.is_marked_field(point) {
            return Err(anyhow!("no ship at this point"));
        } else {
            self.ships.demark_field(point);
        }

        let mut length = 1;

        let next = point;
        while let Ok(next) = next.get_prev(&Orientation::Vertical) {
            if !self.ships.is_marked_field(next) {
                break;
            } else {
                self.ships.demark_field(next);
                length += 1;
            }
        }

        let next = point;
        while let Ok(next) = next.get_next(&Orientation::Vertical) {
            if !self.ships.is_marked_field(next) {
                break;
            } else {
                self.ships.demark_field(next);
                length += 1;
            }
        }

        let next = point;
        while let Ok(next) = next.get_prev(&Orientation::Horizontal) {
            if !self.ships.is_marked_field(next) {
                break;
            } else {
                self.ships.demark_field(next);
                length += 1;
            }
        }

        let next = point;
        while let Ok(next) = next.get_next(&Orientation::Horizontal) {
            if !self.ships.is_marked_field(next) {
                break;
            } else {
                self.ships.demark_field(next);
                length += 1;
            }
        }

        let count = match length {
            SUBMARINE_SIZE => &mut self.submarines,
            DESTROYER_SIZE => &mut self.destroyers,
            CRUISER_SIZE => &mut self.cruisers,
            BATTLESHIP_SIZE => &mut self.battleships,
            _ => panic!("size does not match any known ship type"),
        };

        *count -= 1;

        return Ok(());
    }

    pub fn register_shot(&mut self, point: Point) {
        self.shots.mark_field(point);
    }
}

#[cfg(test)]
mod test_player {
    use anyhow::Result;

    use crate::playmap::Playmap;
    use crate::point::{Orientation, Point};
    use crate::ship::Ship;

    use super::Player;

    #[test]
    pub fn has_ship_at() {
        let mut player = Player::new();
        player.ships = Playmap::from(0b0100 << 124);

        assert_eq!(player.has_ship_at(Point::new(0, 0).unwrap()), false);
        assert_eq!(player.has_ship_at(Point::new(1, 0).unwrap()), true);
    }

    #[test]
    pub fn test_get_hits() {
        let mut player = Player::new();
        player.ships = Playmap::from(0b1001 << 124);
        player.shots = Playmap::from(0b0101 << 124);

        assert_eq!(player.get_hits().value, 0b0001 << 124);
    }

    #[test]
    pub fn has_intact_ships() {
        let mut player = Player::new();
        player.ships = Playmap::from(0b1100 << 124);
        player.shots = Playmap::from(0b1010 << 124);

        assert_eq!(player.has_intact_ships(), true);
    }

    #[test]
    pub fn test_place_figure() -> Result<()> {
        let mut player = Player::new();
        let ship = Ship::Destroyer(Orientation::Horizontal);

        player.place_figure(ship, Point { x: 1, y: 0 })?;
        assert_eq!(player.ships.value, 0b0110 << 124);

        return Ok(());
    }

    #[test]
    pub fn test_remove_figure() -> Result<()> {
        let mut player = Player::new();
        player.ships = Playmap::from(0b0110 << 124);
        player.destroyers = 1;

        player.remove_figure(Point { x: 1, y: 0 })?;
        assert_eq!(player.ships.value, 0b0000 << 124);

        return Ok(());
    }

    #[test]
    pub fn test_register_shot() {
        let mut player = Player::new();

        player.register_shot(Point { x: 1, y: 0 });
        assert_eq!(player.shots.value, 0b0100 << 124);
    }
}
