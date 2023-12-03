use crate::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Playmap {
    pub value: u128,
}

impl Playmap {
    pub fn new() -> Self {
        return Playmap { value: 0 };
    }

    pub fn is_marked_field(&self, point: Point) -> bool {
        let point_index = point.y * 10 + point.x + 1;
        let bit_index = 128 - point_index;

        return self.value >> bit_index & 0b1 == 1;
    }

    pub fn mark_field(&mut self, point: Point) {
        let point_index = point.y * 10 + point.x + 1;
        let bit_index = 128 - point_index;

        self.value |= 1 << bit_index;
    }

    pub fn demark_field(&mut self, point: Point) {
        let point_index = point.y * 10 + point.x + 1;
        let bit_index = 128 - point_index;

        self.value &= !(1 << bit_index);
    }
}

impl From<u128> for Playmap {
    fn from(value: u128) -> Self {
        return Playmap { value };
    }
}

#[cfg(test)]
mod playmap_test {
    use crate::{playmap::Playmap, point::Point};

    #[test]
    pub fn test_is_marked_field() {
        let map = Playmap::from(0b0010 << 124);

        assert_eq!(map.is_marked_field(Point { x: 2, y: 0 }), true);
        assert_eq!(map.is_marked_field(Point { x: 1, y: 0 }), false);
    }

    #[test]
    pub fn test_mark_field() {
        let mut map = Playmap::new();

        map.mark_field(Point { x: 0, y: 0 });
        assert_eq!(map, Playmap::from(0b1000 << 124));

        map.mark_field(Point { x: 2, y: 0 });
        assert_eq!(map, Playmap::from(0b1010 << 124));
    }

    #[test]
    pub fn test_demark_field() {
        let mut map = Playmap::from(0b1010 << 124);

        map.demark_field(Point { x: 0, y: 0 });
        assert_eq!(map, Playmap::from(0b0010 << 124));

        map.demark_field(Point { x: 2, y: 0 });
        assert_eq!(map, Playmap::from(0b0000 << 124));
    }
}
