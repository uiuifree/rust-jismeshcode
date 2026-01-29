//! 特定座標のテスト
//!
//! 座標: 35.6437877, 139.6716674

use jismeshcode::prelude::*;

#[test]
fn test_specific_location_basic() {
    // 指定された座標
    let lat = 35.6437877;
    let lon = 139.6716674;

    println!("\n========================================");
    println!("座標: ({}, {})", lat, lon);
    println!("========================================");

    let coord = Coordinate::new(lat, lon).unwrap();

    // 1次メッシュ
    let mesh1 = coord_to_mesh(coord, MeshLevel::First).unwrap();
    println!("\n1次メッシュ: {}", mesh1.as_string());

    // 2次メッシュ
    let mesh2 = coord_to_mesh(coord, MeshLevel::Second).unwrap();
    println!("2次メッシュ: {}", mesh2.as_string());

    // 3次メッシュ
    let mesh3 = coord_to_mesh(coord, MeshLevel::Third).unwrap();
    println!("3次メッシュ: {}", mesh3.as_string());

    // 検証
    assert_eq!(mesh3.level(), MeshLevel::Third);
    assert_eq!(mesh3.as_string().len(), 8);
}

#[test]
fn test_specific_location_bounds() {
    let coord = Coordinate::new(35.6437877, 139.6716674).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();

    println!("\n========================================");
    println!("3次メッシュ: {}", mesh.as_string());
    println!("========================================");

    // 境界を取得
    let bounds = mesh_to_bounds(mesh);

    println!("\n【境界情報】");
    println!(
        "南西端: ({:.10}, {:.10})",
        bounds.min_lat(),
        bounds.min_lon()
    );
    println!(
        "北東端: ({:.10}, {:.10})",
        bounds.max_lat(),
        bounds.max_lon()
    );

    // 中心座標
    let center = mesh_to_center(mesh);
    println!("\n【中心座標】");
    println!("中心: ({:.10}, {:.10})", center.lat(), center.lon());

    // サイズ
    let lat_size = bounds.max_lat() - bounds.min_lat();
    let lon_size = bounds.max_lon() - bounds.min_lon();
    println!("\n【メッシュサイズ】");
    println!("緯度幅: {:.10}度 ({:.10}秒)", lat_size, lat_size * 3600.0);
    println!("経度幅: {:.10}度 ({:.10}秒)", lon_size, lon_size * 3600.0);

    // 元の座標との距離
    let lat_diff = (coord.lat() - center.lat()).abs();
    let lon_diff = (coord.lon() - center.lon()).abs();
    println!("\n【中心からの距離】");
    println!("緯度差: {:.10}度 ({:.2}m)", lat_diff, lat_diff * 111000.0);
    println!("経度差: {:.10}度 ({:.2}m)", lon_diff, lon_diff * 91000.0);

    // 検証: 元の座標がメッシュ内に含まれることを確認
    assert!(
        bounds.contains(coord),
        "元の座標がメッシュ内に含まれていません"
    );

    // 検証: メッシュサイズが3次メッシュの仕様通りであることを確認
    assert!(
        (lat_size - 30.0 / 3600.0).abs() < 1e-10,
        "緯度サイズが仕様と異なります"
    );
    assert!(
        (lon_size - 45.0 / 3600.0).abs() < 1e-10,
        "経度サイズが仕様と異なります"
    );
}

#[test]
fn test_specific_location_hierarchy() {
    let coord = Coordinate::new(35.6437877, 139.6716674).unwrap();
    let mesh3 = coord_to_mesh(coord, MeshLevel::Third).unwrap();

    println!("\n========================================");
    println!("階層構造");
    println!("========================================");

    // 親メッシュ
    let mesh2 = parent(mesh3).unwrap();
    let mesh1 = parent(mesh2).unwrap();

    println!("\n3次メッシュ: {}", mesh3.as_string());
    println!("↓");
    println!("2次メッシュ: {}", mesh2.as_string());
    println!("↓");
    println!("1次メッシュ: {}", mesh1.as_string());

    // 検証: 階層関係が正しいことを確認
    assert_eq!(mesh2.as_string().len(), 6);
    assert_eq!(mesh1.as_string().len(), 4);
    assert!(mesh3.as_string().starts_with(&mesh2.as_string()));
    assert!(mesh2.as_string().starts_with(&mesh1.as_string()));

    // 子メッシュに自分が含まれることを確認
    let children_of_mesh2 = children(mesh2);
    assert!(
        children_of_mesh2.contains(&mesh3),
        "親の子メッシュに自分が含まれていません"
    );
}

#[test]
fn test_specific_location_neighbors() {
    let coord = Coordinate::new(35.6437877, 139.6716674).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();

    println!("\n========================================");
    println!("隣接メッシュ（3次メッシュ: {}）", mesh.as_string());
    println!("========================================\n");

    // 各方向の隣接メッシュ
    for dir in Direction::ALL {
        if let Some(neighbor_mesh) = neighbor(mesh, dir) {
            let neighbor_center = mesh_to_center(neighbor_mesh);
            println!(
                "{:10}: {} (中心: {:.6}, {:.6})",
                format!("{:?}", dir),
                neighbor_mesh.as_string(),
                neighbor_center.lat(),
                neighbor_center.lon()
            );
        } else {
            println!("{:10}: (範囲外)", format!("{:?}", dir));
        }
    }

    // すべての隣接メッシュを取得
    let all_neighbors = neighbors(mesh);
    println!("\n隣接メッシュ数: {}", all_neighbors.len());

    // 検証: 隣接メッシュが最大8個であることを確認
    assert!(all_neighbors.len() <= 8);
    assert!(
        all_neighbors.len() > 0,
        "隣接メッシュが1つも見つかりませんでした"
    );
}

#[test]
fn test_specific_location_roundtrip() {
    let original_lat = 35.6437877;
    let original_lon = 139.6716674;

    println!("\n========================================");
    println!("ラウンドトリップテスト");
    println!("========================================");

    let coord = Coordinate::new(original_lat, original_lon).unwrap();

    // 各レベルでラウンドトリップテスト
    for level in [
        MeshLevel::First,
        MeshLevel::Second,
        MeshLevel::Third,
        MeshLevel::FourthHalf,
        MeshLevel::FourthQuarter,
        MeshLevel::Fifth,
    ] {
        let mesh = coord_to_mesh(coord, level).unwrap();
        let bounds = mesh_to_bounds(mesh);
        let center = mesh_to_center(mesh);

        println!("\n{:?}メッシュ: {}", level, mesh.as_string());
        println!(
            "  中心との距離: 緯度{:.6}度, 経度{:.6}度",
            (center.lat() - original_lat).abs(),
            (center.lon() - original_lon).abs()
        );

        // 検証: 元の座標がメッシュ内に含まれることを確認
        assert!(
            bounds.contains(coord),
            "{:?}メッシュのラウンドトリップに失敗しました",
            level
        );
    }
}

#[test]
fn test_specific_location_all_levels() {
    let coord = Coordinate::new(35.6437877, 139.6716674).unwrap();

    println!("\n========================================");
    println!("全メッシュレベルの変換結果");
    println!("========================================\n");

    println!("元の座標: ({}, {})", coord.lat(), coord.lon());
    println!();

    // すべてのメッシュレベルを試す
    let levels = [
        ("1次メッシュ (約80km)", MeshLevel::First),
        ("2次メッシュ (約10km)", MeshLevel::Second),
        ("3次メッシュ (約1km)", MeshLevel::Third),
        ("4次メッシュ 2分の1 (約500m)", MeshLevel::FourthHalf),
        ("4次メッシュ 4分の1 (約250m)", MeshLevel::FourthQuarter),
        ("5次メッシュ (約100m)", MeshLevel::Fifth),
    ];

    for (name, level) in levels {
        let mesh = coord_to_mesh(coord, level).unwrap();
        let bounds = mesh_to_bounds(mesh);
        let center = mesh_to_center(mesh);

        println!("{}: {}", name, mesh.as_string());
        println!(
            "  範囲: ({:.6}, {:.6}) 〜 ({:.6}, {:.6})",
            bounds.min_lat(),
            bounds.min_lon(),
            bounds.max_lat(),
            bounds.max_lon()
        );
        println!("  中心: ({:.6}, {:.6})", center.lat(), center.lon());
        println!();
    }
}

#[test]
fn test_specific_location_surrounding_area() {
    let coord = Coordinate::new(35.6437877, 139.6716674).unwrap();
    let center_mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();

    println!("\n========================================");
    println!("周辺エリア（3×3グリッド）");
    println!("========================================\n");

    println!("中心メッシュ: {}\n", center_mesh.as_string());

    // 3×3のグリッドを作成
    let mut grid = Vec::new();

    // 北西から順に
    for row in [-1, 0, 1] {
        let mut row_meshes = Vec::new();
        for col in [-1, 0, 1] {
            let mut current = center_mesh;

            // 南北方向の移動
            if row > 0 {
                for _ in 0..row {
                    if let Some(n) = neighbor(current, Direction::North) {
                        current = n;
                    }
                }
            } else if row < 0 {
                for _ in 0..(-row) {
                    if let Some(s) = neighbor(current, Direction::South) {
                        current = s;
                    }
                }
            }

            // 東西方向の移動
            if col > 0 {
                for _ in 0..col {
                    if let Some(e) = neighbor(current, Direction::East) {
                        current = e;
                    }
                }
            } else if col < 0 {
                for _ in 0..(-col) {
                    if let Some(w) = neighbor(current, Direction::West) {
                        current = w;
                    }
                }
            }

            row_meshes.push(current);
        }
        grid.push(row_meshes);
    }

    // グリッドを表示（北が上）
    println!("北");
    println!("  ↑");
    for (i, row) in grid.iter().enumerate() {
        if i == 0 {
            print!("  ");
        } else {
            print!("  ");
        }
        for (j, mesh) in row.iter().enumerate() {
            if i == 1 && j == 1 {
                print!("[{}] ", mesh.as_string());
            } else {
                print!(" {}  ", mesh.as_string());
            }
        }
        println!();
    }
    println!("西 ←   → 東");
    println!("  ↓");
    println!("南");
}
