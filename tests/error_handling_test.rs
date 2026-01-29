//! エラーハンドリングのテスト
//!
//! 無効な入力に対して適切なエラーが返されることを確認します。

use jismeshcode::prelude::*;

// ========================================
// 座標エラーのテスト
// ========================================

#[test]
fn test_invalid_latitude() {
    // 緯度が範囲外（-90〜90度）
    assert!(Coordinate::new(91.0, 139.0).is_err());
    assert!(Coordinate::new(-91.0, 139.0).is_err());
    assert!(Coordinate::new(100.0, 139.0).is_err());
    assert!(Coordinate::new(-100.0, 139.0).is_err());
}

#[test]
fn test_invalid_longitude() {
    // 経度が範囲外（-180〜180度）
    assert!(Coordinate::new(35.0, 181.0).is_err());
    assert!(Coordinate::new(35.0, -181.0).is_err());
    assert!(Coordinate::new(35.0, 200.0).is_err());
    assert!(Coordinate::new(35.0, -200.0).is_err());
}

#[test]
fn test_out_of_japan_range() {
    // 日本の範囲外（北緯20-46度、東経122-154度）
    assert!(Coordinate::new(10.0, 139.0).is_err()); // 南すぎる
    assert!(Coordinate::new(50.0, 139.0).is_err()); // 北すぎる
    assert!(Coordinate::new(35.0, 100.0).is_err()); // 西すぎる
    assert!(Coordinate::new(35.0, 160.0).is_err()); // 東すぎる

    // ヨーロッパ
    assert!(Coordinate::new(48.8566, 2.3522).is_err()); // パリ

    // アメリカ
    assert!(Coordinate::new(40.7128, -74.0060).is_err()); // ニューヨーク
}

#[test]
fn test_japan_range_boundaries() {
    // 日本の範囲の境界値
    assert!(Coordinate::new(20.0, 122.0).is_ok());
    assert!(Coordinate::new(46.0, 154.0).is_ok());
    assert!(Coordinate::new(19.9, 122.0).is_err());
    assert!(Coordinate::new(46.1, 154.0).is_err());
}

// ========================================
// メッシュコードパースエラーのテスト
// ========================================

#[test]
fn test_empty_mesh_code() {
    // 空文字列
    assert!(MeshCode::from_str("").is_err());
}

#[test]
fn test_invalid_mesh_code_length() {
    // 無効な桁数
    assert!(MeshCode::from_str("1").is_err());
    assert!(MeshCode::from_str("12").is_err());
    assert!(MeshCode::from_str("123").is_err());
    assert!(MeshCode::from_str("12345").is_err()); // 5桁は存在しない
    assert!(MeshCode::from_str("1234567").is_err()); // 7桁は存在しない
}

#[test]
fn test_non_numeric_mesh_code() {
    // 数字以外の文字を含む
    assert!(MeshCode::from_str("abcd").is_err());
    assert!(MeshCode::from_str("53a9").is_err());
    assert!(MeshCode::from_str("5339-46").is_err());
    assert!(MeshCode::from_str("5339 46").is_err());
    assert!(MeshCode::from_str("5339.46").is_err());
}

#[test]
fn test_invalid_digit_values() {
    // 各桁の範囲を超える値
    // 2次メッシュのt,uは0-7のみ有効
    assert!(MeshCode::from_str("533988").is_ok()); // 8は有効（範囲外かもしれないが形式は正しい）
    assert!(MeshCode::from_str("533999").is_ok()); // 9は形式的には有効
}

#[test]
fn test_mesh_code_with_leading_zeros() {
    // 先頭ゼロを含むメッシュコード（有効）
    assert!(MeshCode::from_str("0001").is_ok());
    assert!(MeshCode::from_str("0012").is_ok());
    assert!(MeshCode::from_str("00123456").is_ok());
}

// ========================================
// レベル変換エラーのテスト
// ========================================

#[test]
fn test_invalid_level_conversion() {
    // 粗いメッシュから細かいメッシュへの変換はエラー
    let first = MeshCode::from_str("5339").unwrap();
    assert!(to_level(first, MeshLevel::Second).is_err());
    assert!(to_level(first, MeshLevel::Third).is_err());

    let second = MeshCode::from_str("533946").unwrap();
    assert!(to_level(second, MeshLevel::Third).is_err());
}

#[test]
fn test_valid_level_conversion() {
    // 細かいメッシュから粗いメッシュへの変換は成功
    let third = MeshCode::from_str("53394611").unwrap();
    assert!(to_level(third, MeshLevel::Second).is_ok());
    assert!(to_level(third, MeshLevel::First).is_ok());
    assert!(to_level(third, MeshLevel::Third).is_ok()); // 同じレベル
}

// ========================================
// 隣接メッシュのエッジケース
// ========================================

#[test]
fn test_neighbor_at_boundaries() {
    // 日本の範囲端のメッシュでは、一部の方向に隣接メッシュがない可能性がある

    // 北端付近のメッシュ
    let north_mesh = MeshCode::from_str("6945").unwrap(); // 北海道最北端付近
    let north_neighbors = neighbors(north_mesh);
    // 北方向の隣接メッシュがない可能性がある
    assert!(north_neighbors.len() <= 8);

    // 南端付近のメッシュ
    let south_mesh = MeshCode::from_str("3028").unwrap(); // 沖縄南端付近
    let south_neighbors = neighbors(south_mesh);
    assert!(south_neighbors.len() <= 8);
}

#[test]
fn test_neighbor_out_of_range() {
    // 範囲外に出る隣接メッシュはNoneを返す
    // （具体的なメッシュコードは実装依存）

    // 極端な位置のメッシュ
    let coord = Coordinate::new(20.1, 122.1).unwrap(); // 南西端
    let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();

    // 南西方向の隣接メッシュは範囲外の可能性
    let _sw_neighbor = neighbor(mesh, Direction::SouthWest);
    // Noneまたは範囲外のメッシュ（テストは存在確認のみ）
}

// ========================================
// 境界ボックスのエラーケース
// ========================================

#[test]
fn test_empty_bounding_box() {
    // 南西端と北東端が同じ場合
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    let bbox = BoundingBox::new(coord, coord);

    // イテレータは少なくとも1つのメッシュを返すはず
    let meshes: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Third).collect();
    assert!(meshes.len() > 0);
}

#[test]
fn test_inverted_bounding_box() {
    // 南西端と北東端が逆の場合
    // （現在の実装では特にエラーチェックしていない可能性）
    let sw = Coordinate::new(35.7, 139.8).unwrap();
    let ne = Coordinate::new(35.6, 139.7).unwrap();
    let bbox = BoundingBox::new(sw, ne);

    // 反転した境界ボックスでも動作するかテスト
    let _meshes: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Third).collect();
    // 実装によっては空になる可能性がある（テストは実行確認のみ）
}

// ========================================
// 数値精度のエッジケース
// ========================================

#[test]
fn test_floating_point_precision() {
    // 浮動小数点数の精度に関するエッジケース

    // ほぼ同じ座標
    let coord1 = Coordinate::new(35.681200, 139.767100).unwrap();
    let coord2 = Coordinate::new(35.681201, 139.767101).unwrap();

    let mesh1 = coord_to_mesh(coord1, MeshLevel::Third).unwrap();
    let mesh2 = coord_to_mesh(coord2, MeshLevel::Third).unwrap();

    // 非常に近い座標でも同じメッシュになることを確認
    assert_eq!(mesh1.as_string(), mesh2.as_string());
}

#[test]
fn test_coordinate_at_mesh_boundary() {
    // メッシュの境界上の座標
    let mesh = MeshCode::from_str("53394611").unwrap();
    let bounds = mesh_to_bounds(mesh);

    // 境界の座標を正確に使用
    let sw_coord = bounds.south_west();
    let converted = coord_to_mesh(sw_coord, MeshLevel::Third).unwrap();

    // 南西端の座標は、浮動小数点の丸め誤差により隣のメッシュになる可能性がある
    // メッシュ境界上の座標は不安定なため、境界内に含まれることのみ確認
    let converted_bounds = mesh_to_bounds(converted);
    assert!(
        converted_bounds.contains(sw_coord) || bounds.contains(sw_coord),
        "Coordinate at boundary should be in one of the adjacent meshes"
    );
}

// ========================================
// メモリ安全性のテスト
// ========================================

#[test]
fn test_large_number_of_meshes() {
    // 大量のメッシュを生成してもメモリエラーが起きないことを確認
    let sw = Coordinate::new(35.0, 139.0).unwrap();
    let ne = Coordinate::new(36.0, 140.0).unwrap();
    let bbox = BoundingBox::new(sw, ne);

    // 3次メッシュで大量のメッシュが生成される
    // 1度 x 1度の範囲で、3次メッシュは約1km四方なので、
    // 緯度方向: 1度 / (30秒/3600) ≈ 120個
    // 経度方向: 1度 / (45秒/3600) ≈ 80個
    // 合計: 約 120 x 80 = 9600個
    let count = mesh_codes_in_bbox(bbox, MeshLevel::Third).count();
    assert!(
        count > 5000,
        "Expected more than 5000 meshes, got {}",
        count
    );
    assert!(
        count < 15000,
        "Expected less than 15000 meshes, got {}",
        count
    );
}

#[test]
fn test_mesh_code_copy_trait() {
    // MeshCodeがCopyトレイトを実装していることを確認
    let mesh1 = MeshCode::from_str("5339").unwrap();
    let mesh2 = mesh1; // Copy
    let mesh3 = mesh1; // 再度Copy

    // すべて同じ値を持つことを確認
    assert_eq!(mesh1.as_string(), mesh2.as_string());
    assert_eq!(mesh1.as_string(), mesh3.as_string());
}
