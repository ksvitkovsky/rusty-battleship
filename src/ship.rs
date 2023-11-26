use anyhow::{Ok, Result};

use crate::point::{Orientation, Point};

#[repr(u8)]
pub enum Ship {
    Submarine = 1,
    Destroyer(Orientation) = 2,
    Cruiser(Orientation) = 3,
    Battleship(Orientation) = 4,
}

pub const SUBMARINE_SIZE: u8 = 1;
pub const DESTROYER_SIZE: u8 = 2;
pub const CRUISER_SIZE: u8 = 3;
pub const BATTLESHIP_SIZE: u8 = 4;

impl Ship {
    pub fn limit(&self) -> u8 {
        return match self {
            Self::Submarine => 4,
            Self::Destroyer(_) => 3,
            Self::Cruiser(_) => 2,
            Self::Battleship(_) => 1,
        };
    }

    pub fn get_points(&self, point: Point) -> Result<Vec<Point>> {
        let mut points = Vec::new();

        match self {
            Self::Submarine => {
                points.push(point);
            }
            Self::Destroyer(ort) => {
                points.push(point);
                points.push(points[0].get_next(ort)?);
            }
            Self::Cruiser(ort) => {
                points.push(point);
                points.push(points[0].get_next(ort)?);
                points.push(points[1].get_next(ort)?);
            }
            Self::Battleship(ort) => {
                points.push(point);
                points.push(points[0].get_next(ort)?);
                points.push(points[1].get_next(ort)?);
                points.push(points[2].get_next(ort)?);
            }
        };

        return Ok(points);
    }
}

#[cfg(test)]
mod ship_test {
    use anyhow::Result;

    use crate::point::Point;
    use crate::ship::Orientation;

    use super::Ship;

    #[test]
    pub fn test_get_points() -> Result<()> {
        let point = Point { x: 0, y: 0 };

        let submarine = Ship::Submarine;
        assert_eq!(submarine.get_points(point)?.len(), 1);

        let destroyer = Ship::Destroyer(Orientation::Horizontal);
        assert_eq!(destroyer.get_points(point)?.len(), 2);

        let cruiser = Ship::Cruiser(Orientation::Vertical);
        assert_eq!(cruiser.get_points(point)?.len(), 3);

        let invalid = cruiser.get_points(Point { x: 8, y: 8 });
        assert!(invalid.is_err());

        return Ok(());
    }
}
