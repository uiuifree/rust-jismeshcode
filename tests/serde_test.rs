#![cfg(feature = "serde")]

use jismeshcode::prelude::*;

#[test]
fn test_mesh_code_serialize_as_string() {
    let mesh = MeshCode::from_str("53394611").unwrap();
    let json = serde_json::to_string(&mesh).unwrap();
    assert_eq!(json, "\"53394611\"");
}

#[test]
fn test_mesh_code_roundtrip() {
    for code in ["5339", "533946", "53394611", "533946111", "53394611234"] {
        let mesh = MeshCode::from_str(code).unwrap();
        let json = serde_json::to_string(&mesh).unwrap();
        let restored: MeshCode = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, mesh, "roundtrip failed for {code}");
    }
}

#[test]
fn test_mesh_code_preserves_leading_zeros() {
    let mesh = MeshCode::from_str("0001").unwrap();
    let json = serde_json::to_string(&mesh).unwrap();
    assert_eq!(json, "\"0001\"");
    let restored: MeshCode = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, mesh);
}

#[test]
fn test_mesh_code_deserialize_invalid() {
    // 不正な形式は拒否される
    assert!(serde_json::from_str::<MeshCode>("\"abc\"").is_err());
    assert!(serde_json::from_str::<MeshCode>("\"12345\"").is_err());
    assert!(serde_json::from_str::<MeshCode>("\"533988\"").is_err());
}

#[test]
fn test_coordinate_roundtrip() {
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let json = serde_json::to_string(&coord).unwrap();
    let restored: Coordinate = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, coord);
}

#[test]
fn test_coordinate_deserialize_validates_range() {
    // 基本範囲（緯度±90度、経度±180度）を超える値は拒否される
    assert!(serde_json::from_str::<Coordinate>(r#"{"lat": 91.0, "lon": 139.0}"#).is_err());
    assert!(serde_json::from_str::<Coordinate>(r#"{"lat": 35.0, "lon": 181.0}"#).is_err());
    // 日本範囲外でも基本範囲内なら受け付ける（メッシュ境界の座標が日本範囲を超え得るため）
    assert!(serde_json::from_str::<Coordinate>(r#"{"lat": 0.0, "lon": 0.0}"#).is_ok());
}

#[test]
fn test_bounding_box_roundtrip() {
    let mesh = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(mesh);
    let json = serde_json::to_string(&bounds).unwrap();
    let restored: BoundingBox = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, bounds);
}

#[test]
fn test_mesh_level_roundtrip() {
    for level in [
        MeshLevel::First,
        MeshLevel::Second,
        MeshLevel::Third,
        MeshLevel::FourthHalf,
        MeshLevel::FourthQuarter,
        MeshLevel::FourthEighth,
        MeshLevel::Fifth,
    ] {
        let json = serde_json::to_string(&level).unwrap();
        let restored: MeshLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, level);
    }
}

#[test]
fn test_direction_roundtrip() {
    for dir in Direction::ALL {
        let json = serde_json::to_string(&dir).unwrap();
        let restored: Direction = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, dir);
    }
}
