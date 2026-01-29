use jismeshcode::prelude::*;

#[test]
fn test_basic_radius_search_1000m() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
    let meshes: Vec<_> = mesh_codes_in_radius(tokyo, 1000.0, MeshLevel::Third).collect();

    // 1000m以内には複数のメッシュがある
    assert!(meshes.len() > 1);
    assert!(meshes.len() < 100, "1000m以内のメッシュ数は妥当な範囲");

    // 中心座標を含むメッシュが結果に含まれる
    let center_mesh = coord_to_mesh(tokyo, MeshLevel::Third).unwrap();
    assert!(meshes.contains(&center_mesh));

    // すべてのメッシュが実際に1000m以内か検証
    for mesh in &meshes {
        let mesh_center = mesh_to_center(*mesh);
        let distance = haversine_distance(tokyo, mesh_center);
        assert!(
            distance <= 1000.0,
            "メッシュ {} の距離 {:.2}m は1000m以内であるべき",
            mesh,
            distance
        );
    }
}

#[test]
fn test_mesh_based_radius_search() {
    let mesh = MeshCode::from_str("53394611").unwrap();
    let nearby: Vec<_> = mesh_codes_in_radius_from_mesh(mesh, 1000.0).collect();

    // 中心メッシュ自身が含まれる
    assert!(nearby.contains(&mesh));

    // 複数のメッシュが見つかる
    assert!(nearby.len() > 1);

    // すべて同じレベルのメッシュ
    assert!(nearby.iter().all(|m| m.level() == mesh.level()));

    // 距離の検証
    let center = mesh_to_center(mesh);
    for m in &nearby {
        let m_center = mesh_to_center(*m);
        let distance = haversine_distance(center, m_center);
        assert!(
            distance <= 1000.0,
            "距離 {:.2}m は1000m以内であるべき",
            distance
        );
    }
}

#[test]
fn test_radius_zero() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
    let meshes: Vec<_> = mesh_codes_in_radius(tokyo, 0.0, MeshLevel::Third).collect();

    // 半径0では中心座標を含むメッシュのみ
    assert_eq!(meshes.len(), 1);

    let expected = coord_to_mesh(tokyo, MeshLevel::Third).unwrap();
    assert_eq!(meshes[0], expected);
}

#[test]
fn test_radius_negative() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
    let meshes: Vec<_> = mesh_codes_in_radius(tokyo, -100.0, MeshLevel::Third).collect();

    // 負の半径では空
    assert_eq!(meshes.len(), 0);
}

#[test]
fn test_increasing_radius() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();

    let meshes_500: Vec<_> = mesh_codes_in_radius(tokyo, 500.0, MeshLevel::Third).collect();
    let meshes_1000: Vec<_> = mesh_codes_in_radius(tokyo, 1000.0, MeshLevel::Third).collect();
    let meshes_2000: Vec<_> = mesh_codes_in_radius(tokyo, 2000.0, MeshLevel::Third).collect();
    let meshes_5000: Vec<_> = mesh_codes_in_radius(tokyo, 5000.0, MeshLevel::Third).collect();

    // 半径が大きいほどメッシュ数が増える
    assert!(meshes_500.len() < meshes_1000.len());
    assert!(meshes_1000.len() < meshes_2000.len());
    assert!(meshes_2000.len() < meshes_5000.len());

    println!("500m: {} meshes", meshes_500.len());
    println!("1000m: {} meshes", meshes_1000.len());
    println!("2000m: {} meshes", meshes_2000.len());
    println!("5000m: {} meshes", meshes_5000.len());
}

#[test]
fn test_different_mesh_levels() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
    let radius = 10000.0; // 10km

    let first_level: Vec<_> = mesh_codes_in_radius(tokyo, radius, MeshLevel::First).collect();
    let second_level: Vec<_> = mesh_codes_in_radius(tokyo, radius, MeshLevel::Second).collect();
    let third_level: Vec<_> = mesh_codes_in_radius(tokyo, radius, MeshLevel::Third).collect();

    // 細かいレベルほどメッシュ数が多い
    assert!(first_level.len() < second_level.len());
    assert!(second_level.len() < third_level.len());

    // 正しいレベルのメッシュのみ
    assert!(first_level.iter().all(|m| m.level() == MeshLevel::First));
    assert!(second_level.iter().all(|m| m.level() == MeshLevel::Second));
    assert!(third_level.iter().all(|m| m.level() == MeshLevel::Third));

    println!("First level: {} meshes", first_level.len());
    println!("Second level: {} meshes", second_level.len());
    println!("Third level: {} meshes", third_level.len());
}

#[test]
fn test_distance_accuracy() {
    // 東京-横浜間の距離検証
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
    let yokohama = Coordinate::new(35.4437, 139.6380).unwrap();
    let distance = haversine_distance(tokyo, yokohama);

    // 東京-横浜間は約28km
    assert!(distance > 27000.0 && distance < 29000.0);
    println!("東京-横浜間: {:.2}km", distance / 1000.0);
}

#[test]
fn test_iterator_pattern() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();

    // イテレータの遅延評価を確認
    let mut iter = mesh_codes_in_radius(tokyo, 2000.0, MeshLevel::Third);

    let first = iter.next();
    assert!(first.is_some());

    let second = iter.next();
    assert!(second.is_some());

    // countで残りを消費
    let _remaining = iter.count();

    // 全体では複数のメッシュがあることを確認
    let total: Vec<_> = mesh_codes_in_radius(tokyo, 2000.0, MeshLevel::Third).collect();
    assert!(total.len() > 2, "2000m以内には3つ以上のメッシュがある");
}

#[test]
fn test_large_radius() {
    let tokyo = Coordinate::new(35.6812, 139.7671).unwrap();
    let meshes: Vec<_> = mesh_codes_in_radius(tokyo, 50000.0, MeshLevel::Third).collect();

    // 50km以内には多数のメッシュがある
    assert!(meshes.len() > 100);

    // すべてのメッシュが50km以内
    for mesh in &meshes {
        let mesh_center = mesh_to_center(*mesh);
        let distance = haversine_distance(tokyo, mesh_center);
        assert!(
            distance <= 50000.0,
            "距離 {:.2}m は50km以内であるべき",
            distance
        );
    }

    println!("50km以内のメッシュ数: {}", meshes.len());
}

#[test]
fn test_edge_case_japan_boundary() {
    // 日本の北端付近
    let hokkaido = Coordinate::new(45.0, 141.0).unwrap();
    let meshes: Vec<_> = mesh_codes_in_radius(hokkaido, 5000.0, MeshLevel::Third).collect();

    assert!(meshes.len() > 0);
    assert!(meshes.iter().all(|m| m.level() == MeshLevel::Third));
}

#[test]
fn test_mesh_from_string() {
    // 文字列からメッシュコードを作成して検索
    let mesh = MeshCode::from_str("53394611").unwrap();
    let nearby: Vec<_> = mesh_codes_in_radius_from_mesh(mesh, 2000.0).collect();

    assert!(nearby.contains(&mesh));
    assert!(nearby.len() > 1);

    println!("メッシュ {} から2000m以内: {} 個", mesh, nearby.len());
}
