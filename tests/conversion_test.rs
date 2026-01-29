//! 変換ロジックの包括的なテスト
//!
//! このテストファイルでは、座標とメッシュコードの変換ロジックを
//! 境界値、エッジケース、全メッシュレベルで徹底的にテストします。

use jismeshcode::prelude::*;

// ========================================
// 1次メッシュの変換テスト
// ========================================

#[test]
fn test_first_mesh_boundaries() {
    // 1次メッシュの境界値テスト
    // メッシュ "5339" の範囲: 北緯35.333...度〜36.000度, 東経139度〜140度

    // 南西角（含まれる）
    let sw = Coordinate::new(35.333334, 139.0).unwrap();
    let mesh = coord_to_mesh(sw, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5339");

    // 北東角に近い点（含まれる）
    let ne_inside = Coordinate::new(35.999, 139.999).unwrap();
    let mesh = coord_to_mesh(ne_inside, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5339");

    // 北東角の外（含まれない）
    let ne_outside = Coordinate::new(36.001, 140.001).unwrap();
    let mesh = coord_to_mesh(ne_outside, MeshLevel::First).unwrap();
    assert_ne!(mesh.as_string(), "5339");
}

#[test]
fn test_first_mesh_all_digits() {
    // 各桁の値が正しく計算されることを確認

    // p=5, q=3, r=3, s=9 -> 5339
    let coord = Coordinate::new(35.5, 139.5).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5339");

    // p=5, q=2, r=3, s=8 -> 5238
    let coord = Coordinate::new(35.0, 138.5).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "5238");

    // 北海道: p=6, q=4, r=4, s=1 -> 6441
    let coord = Coordinate::new(43.0, 141.5).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "6441");

    // 沖縄: p=3, q=9, r=2, s=8 -> 3928
    let coord = Coordinate::new(26.5, 128.0).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::First).unwrap();
    assert_eq!(mesh.as_string(), "3928");
}

// ========================================
// 2次メッシュの変換テスト
// ========================================

#[test]
fn test_second_mesh_conversion() {
    // 2次メッシュは1次メッシュを8×8に分割
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::Second).unwrap();
    assert_eq!(mesh.as_string(), "533946");

    // 2次メッシュのコードから1次メッシュを確認
    let first = to_level(mesh, MeshLevel::First).unwrap();
    assert_eq!(first.as_string(), "5339");
}

#[test]
fn test_second_mesh_all_subdivisions() {
    // 1次メッシュ "5339" の64個の2次メッシュを確認
    let first_mesh = MeshCode::from_str("5339").unwrap();
    let children_list = children(first_mesh);

    assert_eq!(children_list.len(), 64);

    // t=0〜7, u=0〜7の全組み合わせが存在することを確認
    for t in 0..8 {
        for u in 0..8 {
            let expected = format!("5339{}{}", t, u);
            assert!(
                children_list.iter().any(|m| m.as_string() == expected),
                "Missing second mesh: {}",
                expected
            );
        }
    }
}

#[test]
fn test_second_mesh_boundaries() {
    // 2次メッシュの境界テスト
    let base_lat = 35.666667; // 1次メッシュの南端 + 4 * (40/60/8)
    let base_lon = 139.75; // 1次メッシュの西端 + 6 * 1.0/8

    // 境界内の座標
    let inside = Coordinate::new(base_lat + 0.01, base_lon + 0.01).unwrap();
    let mesh = coord_to_mesh(inside, MeshLevel::Second).unwrap();
    assert_eq!(mesh.as_string(), "533946");
}

// ========================================
// 3次メッシュの変換テスト
// ========================================

#[test]
fn test_third_mesh_conversion() {
    // 東京駅の3次メッシュ
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
    assert_eq!(mesh.as_string(), "53394611");
}

#[test]
fn test_third_mesh_all_subdivisions() {
    // 2次メッシュ "533946" の100個の3次メッシュを確認
    let second_mesh = MeshCode::from_str("533946").unwrap();
    let children_list = children(second_mesh);

    assert_eq!(children_list.len(), 100);

    // v=0〜9, w=0〜9の全組み合わせが存在することを確認
    for v in 0..10 {
        for w in 0..10 {
            let expected = format!("533946{}{}", v, w);
            assert!(
                children_list.iter().any(|m| m.as_string() == expected),
                "Missing third mesh: {}",
                expected
            );
        }
    }
}

#[test]
fn test_third_mesh_size() {
    // 3次メッシュのサイズが約1km（30秒 × 45秒）であることを確認
    let mesh = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(mesh);

    let lat_diff = bounds.max_lat() - bounds.min_lat();
    let lon_diff = bounds.max_lon() - bounds.min_lon();

    // 緯度: 30秒 = 30/3600度
    assert!((lat_diff - 30.0 / 3600.0).abs() < 1e-10);
    // 経度: 45秒 = 45/3600度
    assert!((lon_diff - 45.0 / 3600.0).abs() < 1e-10);
}

// ========================================
// 4次メッシュ（2分の1）の変換テスト
// ========================================

#[test]
fn test_fourth_half_mesh_conversion() {
    // 3次メッシュを2×2に分割（1,2,3,4）
    let third = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(third);
    let center = bounds.center();

    // 中心座標から4次メッシュを取得
    let mesh = coord_to_mesh(center, MeshLevel::FourthHalf).unwrap();

    // 親メッシュが正しいことを確認
    let parent_mesh = parent(mesh).unwrap();
    assert_eq!(parent_mesh.as_string(), "53394611");
}

#[test]
fn test_fourth_half_mesh_all_subdivisions() {
    // 3次メッシュの4個の4次メッシュ（2分の1）
    let third = MeshCode::from_str("53394611").unwrap();
    let children_list = children(third);

    assert_eq!(children_list.len(), 4);

    // 1,2,3,4のすべてが存在することを確認
    for i in 1..=4 {
        let expected = format!("53394611{}", i);
        assert!(
            children_list.iter().any(|m| m.as_string() == expected),
            "Missing fourth half mesh: {}",
            expected
        );
    }
}

#[test]
fn test_fourth_half_mesh_quadrants() {
    // 4次メッシュ（2分の1）の4象限を確認
    let third_str = "53394611";
    let third_bounds = mesh_to_bounds(MeshCode::from_str(third_str).unwrap());

    let lat_mid = (third_bounds.min_lat() + third_bounds.max_lat()) / 2.0;
    let lon_mid = (third_bounds.min_lon() + third_bounds.max_lon()) / 2.0;

    // 北東象限（1）
    let ne = Coordinate::new_unchecked(lat_mid + 0.001, lon_mid + 0.001);
    let mesh = coord_to_mesh(ne, MeshLevel::FourthHalf).unwrap();
    assert_eq!(mesh.as_string(), "533946111");

    // 南東象限（2）
    let se = Coordinate::new_unchecked(lat_mid - 0.001, lon_mid + 0.001);
    let mesh = coord_to_mesh(se, MeshLevel::FourthHalf).unwrap();
    assert_eq!(mesh.as_string(), "533946112");

    // 北西象限（3）
    let nw = Coordinate::new_unchecked(lat_mid + 0.001, lon_mid - 0.001);
    let mesh = coord_to_mesh(nw, MeshLevel::FourthHalf).unwrap();
    assert_eq!(mesh.as_string(), "533946113");

    // 南西象限（4）
    let sw = Coordinate::new_unchecked(lat_mid - 0.001, lon_mid - 0.001);
    let mesh = coord_to_mesh(sw, MeshLevel::FourthHalf).unwrap();
    assert_eq!(mesh.as_string(), "533946114");
}

// ========================================
// 4次メッシュ（4分の1）の変換テスト
// ========================================

#[test]
fn test_fourth_quarter_mesh_conversion() {
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::FourthQuarter).unwrap();

    // 親メッシュが3次メッシュであることを確認
    let parent_mesh = parent(mesh).unwrap();
    assert_eq!(parent_mesh.level(), MeshLevel::Third);
}

#[test]
fn test_fourth_quarter_mesh_range() {
    // 4次メッシュ（4分の1）は01〜16の範囲
    let third = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(third);

    // 各位置で4次メッシュ（4分の1）を生成
    let lat_step = (bounds.max_lat() - bounds.min_lat()) / 4.0;
    let lon_step = (bounds.max_lon() - bounds.min_lon()) / 4.0;

    for i in 0..4 {
        for j in 0..4 {
            let lat = bounds.min_lat() + (i as f64 + 0.5) * lat_step;
            let lon = bounds.min_lon() + (j as f64 + 0.5) * lon_step;
            let coord = Coordinate::new_unchecked(lat, lon);
            let mesh = coord_to_mesh(coord, MeshLevel::FourthQuarter).unwrap();

            // コードの末尾が01〜16の範囲内であることを確認
            let code_str = mesh.as_string();
            let last_two = &code_str[code_str.len() - 2..];
            let num: u32 = last_two.parse().unwrap();
            assert!(
                num >= 1 && num <= 16,
                "Invalid fourth quarter code: {}",
                num
            );
        }
    }
}

// ========================================
// 5次メッシュの変換テスト
// ========================================

#[test]
fn test_fifth_mesh_conversion() {
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let mesh = coord_to_mesh(coord, MeshLevel::Fifth).unwrap();

    // 親メッシュが3次メッシュであることを確認
    let parent_mesh = parent(mesh).unwrap();
    assert_eq!(parent_mesh.level(), MeshLevel::Third);

    // 5次メッシュのサイズが約100m（3秒 × 4.5秒）であることを確認
    let bounds = mesh_to_bounds(mesh);
    let lat_diff = bounds.max_lat() - bounds.min_lat();
    let lon_diff = bounds.max_lon() - bounds.min_lon();

    assert!((lat_diff - 3.0 / 3600.0).abs() < 1e-10);
    assert!((lon_diff - 4.5 / 3600.0).abs() < 1e-10);
}

// ========================================
// ラウンドトリップテスト
// ========================================

#[test]
fn test_roundtrip_all_levels() {
    // すべてのメッシュレベルでラウンドトリップテスト
    let original_coord = Coordinate::new(35.6812, 139.7671).unwrap();

    for level in [
        MeshLevel::First,
        MeshLevel::Second,
        MeshLevel::Third,
        MeshLevel::FourthHalf,
        MeshLevel::FourthQuarter,
        MeshLevel::Fifth,
    ] {
        let mesh = coord_to_mesh(original_coord, level).unwrap();
        let bounds = mesh_to_bounds(mesh);

        // 元の座標がメッシュの境界内に含まれることを確認
        assert!(
            bounds.contains(original_coord),
            "Roundtrip failed for level {:?}",
            level
        );
    }
}

#[test]
fn test_roundtrip_precision() {
    // ラウンドトリップの精度テスト
    let test_coords = vec![
        (35.6812, 139.7671), // 東京駅
        (35.6586, 139.7454), // 東京タワー
        (43.0642, 141.3469), // 札幌駅
        (26.2124, 127.6809), // 那覇
        (35.0116, 135.7681), // 京都駅
    ];

    for (lat, lon) in test_coords {
        let coord = Coordinate::new(lat, lon).unwrap();
        let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
        let center = mesh_to_center(mesh);

        // 中心座標との距離が妥当な範囲内（3次メッシュの半分以内）
        let lat_diff = (center.lat() - lat).abs();
        let lon_diff = (center.lon() - lon).abs();

        assert!(
            lat_diff < 15.0 / 3600.0,
            "Latitude difference too large: {}",
            lat_diff
        );
        assert!(
            lon_diff < 22.5 / 3600.0,
            "Longitude difference too large: {}",
            lon_diff
        );
    }
}

// ========================================
// エッジケーステスト
// ========================================

#[test]
fn test_japan_boundary_coordinates() {
    // 日本の範囲境界付近の座標
    let boundary_coords = vec![
        (20.5, 136.0), // 南端付近
        (45.5, 141.0), // 北端付近
        (35.0, 122.5), // 西端付近
        (35.0, 153.5), // 東端付近
    ];

    for (lat, lon) in boundary_coords {
        let coord = Coordinate::new(lat, lon).unwrap();
        for level in [MeshLevel::First, MeshLevel::Second, MeshLevel::Third] {
            let mesh = coord_to_mesh(coord, level).unwrap();
            let bounds = mesh_to_bounds(mesh);
            assert!(bounds.contains(coord));
        }
    }
}

#[test]
fn test_mesh_code_boundaries() {
    // メッシュコードの各桁が境界値を取ることを確認

    // pの最大値: 9（北緯60度）※日本の範囲外も含む
    // qの範囲: 0-9
    // rの範囲: 2-5（東経122度〜154度に対応）
    // sの範囲: 0-9

    // 日本の範囲内での最小・最大メッシュコード
    let min_coord = Coordinate::new(20.1, 122.1).unwrap();
    let max_coord = Coordinate::new(45.9, 153.9).unwrap();

    let min_mesh = coord_to_mesh(min_coord, MeshLevel::First).unwrap();
    let max_mesh = coord_to_mesh(max_coord, MeshLevel::First).unwrap();

    // メッシュコードが4桁であることを確認
    assert_eq!(min_mesh.as_string().len(), 4);
    assert_eq!(max_mesh.as_string().len(), 4);
}

// ========================================
// 精度テスト
// ========================================

#[test]
fn test_conversion_consistency() {
    // 同じメッシュ内の複数の座標が同じメッシュコードに変換されることを確認
    let mesh = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(mesh);

    // メッシュ内の9点をテスト
    for i in 0..3 {
        for j in 0..3 {
            let lat =
                bounds.min_lat() + (i as f64 + 0.5) / 3.0 * (bounds.max_lat() - bounds.min_lat());
            let lon =
                bounds.min_lon() + (j as f64 + 0.5) / 3.0 * (bounds.max_lon() - bounds.min_lon());
            let coord = Coordinate::new_unchecked(lat, lon);
            let converted = coord_to_mesh(coord, MeshLevel::Third).unwrap();

            assert_eq!(
                converted.as_string(),
                mesh.as_string(),
                "Inconsistent conversion at ({}, {})",
                lat,
                lon
            );
        }
    }
}

#[test]
fn test_adjacent_mesh_boundaries() {
    // 隣接するメッシュの境界が正しく接していることを確認
    let mesh1 = MeshCode::from_str("53394611").unwrap();
    let mesh2 = MeshCode::from_str("53394612").unwrap(); // 東隣

    let bounds1 = mesh_to_bounds(mesh1);
    let bounds2 = mesh_to_bounds(mesh2);

    // mesh1の東端とmesh2の西端が一致することを確認
    assert!(
        (bounds1.max_lon() - bounds2.min_lon()).abs() < 1e-10,
        "Adjacent mesh boundaries don't match"
    );
}
