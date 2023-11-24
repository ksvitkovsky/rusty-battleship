use anyhow::{Ok, Result};

use crate::point::{Orientation, Point};

#[repr(u8)]
pub enum Ship {
    Submarine = 1,
    Destroyer(Orientation) = 2,
    Cruiser(Orientation) = 3,
    Battleship(Orientation) = 4,
}

impl Ship {
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