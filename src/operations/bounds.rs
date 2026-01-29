use crate::convert::{mesh_to_bounds, mesh_to_center};
use crate::types::{BoundingBox, Coordinate, MeshCode};

pub fn bounds(mesh: MeshCode) -> BoundingBox {
    mesh_to_bounds(mesh)
}

pub fn center(mesh: MeshCode) -> Coordinate {
    mesh_to_center(mesh)
}

pub fn contains(mesh: MeshCode, coord: Coordinate) -> bool {
    let bbox = bounds(mesh);
    bbox.contains(coord)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds() {
        let mesh = MeshCode::from_str("5339").unwrap();
        let bbox = bounds(mesh);
        assert!(bbox.min_lat() > 0.0);
        assert!(bbox.max_lat() > bbox.min_lat());
    }

    #[test]
    fn test_center() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let c = center(mesh);
        assert!(c.lat() > 35.0 && c.lat() < 36.0);
        assert!(c.lon() > 139.0 && c.lon() < 140.0);
    }

    #[test]
    fn test_contains() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let c = center(mesh);
        assert!(contains(mesh, c));
    }
}
