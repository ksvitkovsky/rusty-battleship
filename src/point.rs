use anyhow::{anyhow, Error, Result};

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal = 1,
    Vertical = 2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Result<Self> {
        if x > 9 || y > 9 {
            return Err(anyhow!("coordinate is bound to 0..9 range"));
        }

        return Ok(Point { x, y });
    }

    pub fn get_next(&self, orientation: &Orientation) -> Result<Self> {
        match orientation {
            Orientation::Horizontal => {
                if self.x == 9 {
                    return Err(anyhow!("next point is out of range"));
                }

                return Ok(Point {
                    x: self.x + 1,
                    y: self.y,
                });
            }
            Orientation::Vertical => {
                if self.y == 9 {
                    return Err(anyhow!("next point is out of range"));
                }

                return Ok(Point {
                    x: self.x,
                    y: self.y + 1,
                });
            }
        }
    }

    pub fn get_prev(&self, orientation: &Orientation) -> Result<Self> {
        match orientation {
            Orientation::Horizontal => {
                if self.x == 0 {
                    return Err(anyhow!("prev point is out of range"));
                }

                return Ok(Point {
                    x: self.x - 1,
                    y: self.y,
                });
            }
            Orientation::Vertical => {
                if self.y == 0 {
                    return Err(anyhow!("prev point is out of range"));
                }

                return Ok(Point {
                    x: self.x,
                    y: self.y - 1,
                });
            }
        }
    }
}

impl TryFrom<u8> for Point {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let x = value >> 4;
        let y = value & 0b0000_1111;

        return Point::new(x, y);
    }
}

#[cfg(test)]
mod point_test {
    use anyhow::Result;

    use crate::point::{Orientation, Point};

    #[test]
    pub fn test_from_u8() -> Result<()> {
        assert_eq!(Point::try_from(0b0000_0000)?, Point { x: 0, y: 0 });
        assert_eq!(Point::try_from(0b1001_1000)?, Point { x: 9, y: 8 });

        return Ok(());
    }

    #[test]
    pub fn test_get_next() -> Result<()> {
        let next = Point { x: 0, y: 0 }.get_next(&Orientation::Horizontal)?;
        assert_eq!(next, Point { x: 1, y: 0 });

        let next = next.get_next(&Orientation::Vertical)?;
        assert_eq!(next, Point { x: 1, y: 1 });

        let next = Point { x: 9, y: 0 }.get_next(&Orientation::Horizontal);
        assert!(next.is_err());

        return Ok(());
    }

    #[test]
    pub fn test_get_prev() -> Result<()> {
        let prev = Point { x: 1, y: 1 }.get_prev(&Orientation::Horizontal)?;
        assert_eq!(prev, Point { x: 0, y: 1 });

        let prev = prev.get_prev(&Orientation::Vertical)?;
        assert_eq!(prev, Point { x: 0, y: 0 });

        let prev = Point { x: 0, y: 0 }.get_prev(&Orientation::Horizontal);
        assert!(prev.is_err());

        return Ok(());
    }
}
