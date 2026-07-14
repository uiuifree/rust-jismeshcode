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

// ========================================
// 分割地域メッシュ（JIS X 0410）のテスト
// ========================================

/// 分割番号は南西=1、南東=2、北西=3、北東=4（JIS X 0410）
#[test]
fn test_half_mesh_jis_numbering() {
    let third = MeshCode::from_str("53394611").unwrap();
    let children_list = children(third);
    assert_eq!(children_list.len(), 4);

    // 番号1のメッシュは3次メッシュと同じ南西端を持つ
    let third_bounds = mesh_to_bounds(third);
    let half1 = MeshCode::from_str("533946111").unwrap();
    let half1_bounds = mesh_to_bounds(half1);
    assert!((half1_bounds.min_lat() - third_bounds.min_lat()).abs() < 1e-10);
    assert!((half1_bounds.min_lon() - third_bounds.min_lon()).abs() < 1e-10);

    // 番号4のメッシュは3次メッシュと同じ北東端を持つ
    let half4 = MeshCode::from_str("533946114").unwrap();
    let half4_bounds = mesh_to_bounds(half4);
    assert!((half4_bounds.max_lat() - third_bounds.max_lat()).abs() < 1e-10);
    assert!((half4_bounds.max_lon() - third_bounds.max_lon()).abs() < 1e-10);
}

/// 各分割レベルで座標→メッシュ→境界のラウンドトリップが成立する
#[test]
fn test_subdivision_roundtrip_consistency() {
    let third = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(third);

    for level in [
        MeshLevel::FourthHalf,
        MeshLevel::FourthQuarter,
        MeshLevel::FourthEighth,
        MeshLevel::Fifth,
    ] {
        // 3次メッシュ内を細かくサンプリング
        for i in 0..10 {
            for j in 0..10 {
                let lat = bounds.min_lat()
                    + (i as f64 + 0.5) / 10.0 * (bounds.max_lat() - bounds.min_lat());
                let lon = bounds.min_lon()
                    + (j as f64 + 0.5) / 10.0 * (bounds.max_lon() - bounds.min_lon());
                let coord = Coordinate::new_unchecked(lat, lon);

                let mesh = coord_to_mesh(coord, level).unwrap();
                assert_eq!(mesh.level(), level, "level mismatch at ({lat}, {lon})");

                // 座標がメッシュ境界内に含まれる
                let mesh_bounds = mesh_to_bounds(mesh);
                assert!(
                    mesh_bounds.contains(coord),
                    "{level:?} mesh {mesh} does not contain ({lat}, {lon})"
                );

                // 文字列経由の再パースで同じメッシュに戻る（10桁の曖昧なケースを除く）
                if level != MeshLevel::Fifth {
                    let reparsed = MeshCode::from_str(&mesh.as_string()).unwrap();
                    assert_eq!(reparsed, mesh, "reparse mismatch for {mesh}");
                }
            }
        }
    }
}

/// 5次メッシュ（100m）は緯度番号0〜9・経度番号0〜9で、北東端でも桁があふれない
#[test]
fn test_fifth_mesh_jis_numbering() {
    let third = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(third);

    // 3次メッシュの南西端 → "…00"
    let sw = Coordinate::new_unchecked(bounds.min_lat() + 1e-9, bounds.min_lon() + 1e-9);
    let mesh = coord_to_mesh(sw, MeshLevel::Fifth).unwrap();
    assert_eq!(mesh.as_string(), "5339461100");

    // 3次メッシュの北東端直前 → "…99"（以前は桁あふれで隣の3次メッシュに化けていた）
    let ne = Coordinate::new_unchecked(bounds.max_lat() - 1e-9, bounds.max_lon() - 1e-9);
    let mesh = coord_to_mesh(ne, MeshLevel::Fifth).unwrap();
    assert_eq!(mesh.as_string(), "5339461199");
}

/// 8分の1メッシュは11桁で各分割番号が1〜4
#[test]
fn test_eighth_mesh_jis_format() {
    let third = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(third);
    let center = bounds.center();

    let mesh = coord_to_mesh(center, MeshLevel::FourthEighth).unwrap();
    let code_str = mesh.as_string();
    assert_eq!(code_str.len(), 11);
    for digit in code_str[8..11].chars() {
        assert!(('1'..='4').contains(&digit), "invalid digit in {code_str}");
    }
}

/// 分割メッシュの階層: 3次 → 1/2 → 1/4 → 1/8
#[test]
fn test_subdivision_hierarchy() {
    let eighth = MeshCode::from_str("53394611234").unwrap();
    assert_eq!(eighth.level(), MeshLevel::FourthEighth);

    let quarter = parent(eighth).unwrap();
    assert_eq!(quarter.level(), MeshLevel::FourthQuarter);
    assert_eq!(quarter.as_string(), "5339461123");

    let half = parent(quarter).unwrap();
    assert_eq!(half.level(), MeshLevel::FourthHalf);
    assert_eq!(half.as_string(), "533946112");

    let third = parent(half).unwrap();
    assert_eq!(third.level(), MeshLevel::Third);
    assert_eq!(third.as_string(), "53394611");

    // childrenも階層に沿って4個ずつ
    assert_eq!(children(half).len(), 4);
    assert_eq!(children(quarter).len(), 4);
    assert!(children(eighth).is_empty());
}

/// 無効なコード値はMeshCode::newで拒否される
#[test]
fn test_mesh_code_validation() {
    // 2次メッシュの緯度・経度番号は0〜7
    assert!(MeshCode::from_str("533988").is_err());
    // 分割番号は1〜4
    assert!(MeshCode::from_str("533946110").is_err());
    assert!(MeshCode::from_str("533946115").is_err());
    // 桁あふれ
    assert!(MeshCode::new(MeshLevel::First, 10000).is_err());
}
