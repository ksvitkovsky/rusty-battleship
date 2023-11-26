use anyhow::{anyhow, Result};

use crate::{
    playmap::Playmap,
    point::{Orientation, Point},
    ship::{Ship, BATTLESHIP_SIZE, CRUISER_SIZE, DESTROYER_SIZE, SUBMARINE_SIZE},
};

pub struct Player {
    pub ships: Playmap,
    pub shots: Playmap,

    submarines: u8,
    destroyers: u8,
    cruisers: u8,
    battleships: u8,
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

    pub fn get_hits(&self) -> Playmap {
        let value = self.ships.value & self.shots.value;
        return Playmap { value };
    }

    pub fn place_figure(&mut self, ship: Ship, point: Point) -> Result<()> {
        let count = match ship {
            Ship::Submarine => &mut self.submarines,
            Ship::Destroyer(_) => &mut self.destroyers,
            Ship::Cruiser(_) => &mut self.cruisers,
            Ship::Battleship(_) => &mut self.battleships,
        };

        if *count == ship.limit() {
            return Err(anyhow!("cant place ship above limit"));
        }

        for point in ship.get_points(point)? {
            self.ships.mark_field(point);
        }

        *count += 1;

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
    pub fn test_get_hits() {
        let mut player = Player::new();
        player.ships = Playmap::from_u128(0b1001 << 124);
        player.shots = Playmap::from_u128(0b0101 << 124);

        assert_eq!(player.get_hits().value, 0b0001 << 124);
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
        player.ships = Playmap::from_u128(0b0110 << 124);

        player.remove_figure(Point { x: 1, y: 0 })?;
        assert_eq!(player.ships.value, 0b0000 << 124);

        return Ok(());
    }

    #[test]
    pub fn test_register_shot() {
        todo!()
    }
}
