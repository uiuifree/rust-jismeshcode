use jismeshcode::prelude::*;

#[test]
fn test_roundtrip_conversion() {
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
    let bounds = mesh_to_bounds(mesh);

    assert!(bounds.contains(coord));
}

#[test]
fn test_parent_child_consistency() {
    let mesh = MeshCode::from_str("53394611").unwrap();

    let parent_mesh = parent(mesh).unwrap();
    assert_eq!(parent_mesh.as_string(), "533946");

    let children_list = children(parent_mesh);
    assert!(children_list.contains(&mesh));
}

#[test]
fn test_level_conversion() {
    let mesh = MeshCode::from_str("53394611").unwrap();

    let second = to_level(mesh, MeshLevel::Second).unwrap();
    assert_eq!(second.as_string(), "533946");

    let first = to_level(mesh, MeshLevel::First).unwrap();
    assert_eq!(first.as_string(), "5339");

    assert!(to_level(first, MeshLevel::Third).is_err());
}

#[test]
fn test_neighbor_consistency() {
    let mesh = MeshCode::from_str("53394611").unwrap();
    let east = neighbor(mesh, Direction::East);
    assert!(east.is_some());

    if let Some(east_mesh) = east {
        let west_of_east = neighbor(east_mesh, Direction::West);
        assert_eq!(west_of_east, Some(mesh));
    }
}

#[test]
fn test_bbox_iteration() {
    let sw = Coordinate::new(35.6, 139.7).unwrap();
    let ne = Coordinate::new(35.7, 139.8).unwrap();
    let bbox = BoundingBox::new(sw, ne);

    let meshes: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Third).collect();

    assert!(meshes.len() > 0);
    for mesh in meshes {
        let center = mesh_to_center(mesh);
        assert!(
            center.lat() >= sw.lat() - 0.1 && center.lat() <= ne.lat() + 0.1,
            "Mesh center latitude out of range"
        );
        assert!(
            center.lon() >= sw.lon() - 0.1 && center.lon() <= ne.lon() + 0.1,
            "Mesh center longitude out of range"
        );
    }
}
