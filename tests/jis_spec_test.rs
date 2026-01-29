use jismeshcode::prelude::*;

#[test]
fn test_tokyo_station() {
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5339");
}

#[test]
fn test_tokyo_tower() {
    let coord = Coordinate::new(35.6586, 139.7454).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5339");
}

#[test]
fn test_mount_fuji() {
    let coord = Coordinate::new(35.3606, 138.7274).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5338");
}

#[test]
fn test_first_mesh_size() {
    let mesh = MeshCode::from_str("5339").unwrap();
    let bounds = mesh_to_bounds(mesh);

    let lat_diff = bounds.max_lat() - bounds.min_lat();
    let lon_diff = bounds.max_lon() - bounds.min_lon();

    assert!((lat_diff - 40.0 / 60.0).abs() < 1e-10);
    assert!((lon_diff - 1.0).abs() < 1e-10);
}

#[test]
fn test_second_mesh_count() {
    let first_mesh = MeshCode::from_str("5339").unwrap();
    let children_list = children(first_mesh);

    assert_eq!(children_list.len(), 64);
}

#[test]
fn test_third_mesh_count() {
    let second_mesh = MeshCode::from_str("533946").unwrap();
    let children_list = children(second_mesh);

    assert_eq!(children_list.len(), 100);
}

#[test]
fn test_mesh_hierarchy() {
    let third = MeshCode::from_str("53394611").unwrap();
    let second = parent(third).unwrap();
    let first = parent(second).unwrap();

    assert_eq!(first.as_string(), "5339");
    assert_eq!(second.as_string(), "533946");
    assert!(parent(first).is_none());
}
