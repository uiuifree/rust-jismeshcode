use crate::types::coordinate::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    south_west: Coordinate,
    north_east: Coordinate,
}

impl BoundingBox {
    pub fn new(south_west: Coordinate, north_east: Coordinate) -> Self {
        BoundingBox {
            south_west,
            north_east,
        }
    }

    pub fn south_west(&self) -> Coordinate {
        self.south_west
    }

    pub fn north_east(&self) -> Coordinate {
        self.north_east
    }

    pub fn min_lat(&self) -> f64 {
        self.south_west.lat()
    }

    pub fn max_lat(&self) -> f64 {
        self.north_east.lat()
    }

    pub fn min_lon(&self) -> f64 {
        self.south_west.lon()
    }

    pub fn max_lon(&self) -> f64 {
        self.north_east.lon()
    }

    pub fn contains(&self, coord: Coordinate) -> bool {
        coord.lat() >= self.min_lat()
            && coord.lat() <= self.max_lat()
            && coord.lon() >= self.min_lon()
            && coord.lon() <= self.max_lon()
    }

    pub fn center(&self) -> Coordinate {
        let lat = (self.min_lat() + self.max_lat()) / 2.0;
        let lon = (self.min_lon() + self.max_lon()) / 2.0;
        Coordinate::new_unchecked(lat, lon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box() {
        let sw = Coordinate::new_unchecked(35.0, 139.0);
        let ne = Coordinate::new_unchecked(36.0, 140.0);
        let bbox = BoundingBox::new(sw, ne);

        assert_eq!(bbox.min_lat(), 35.0);
        assert_eq!(bbox.max_lat(), 36.0);
        assert_eq!(bbox.min_lon(), 139.0);
        assert_eq!(bbox.max_lon(), 140.0);
    }

    #[test]
    fn test_contains() {
        let sw = Coordinate::new_unchecked(35.0, 139.0);
        let ne = Coordinate::new_unchecked(36.0, 140.0);
        let bbox = BoundingBox::new(sw, ne);

        let inside = Coordinate::new_unchecked(35.5, 139.5);
        let outside = Coordinate::new_unchecked(37.0, 139.5);

        assert!(bbox.contains(inside));
        assert!(!bbox.contains(outside));
    }

    #[test]
    fn test_center() {
        let sw = Coordinate::new_unchecked(35.0, 139.0);
        let ne = Coordinate::new_unchecked(36.0, 140.0);
        let bbox = BoundingBox::new(sw, ne);

        let center = bbox.center();
        assert_eq!(center.lat(), 35.5);
        assert_eq!(center.lon(), 139.5);
    }
}
