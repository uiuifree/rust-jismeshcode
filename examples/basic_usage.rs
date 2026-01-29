//! # 基本的な使い方
//!
//! このサンプルでは、jismeshcodeライブラリの基本的な機能を紹介します。
//! - 座標からメッシュコードへの変換
//! - メッシュの境界と中心座標の取得
//! - 親子関係の確認
//! - メッシュコード文字列のパース

use jismeshcode::prelude::*;

fn main() {
    println!("=== 基本的な使い方のサンプル ===\n");

    // ========================================
    // 1. 座標からメッシュコードへの変換
    // ========================================
    println!("1. 座標からメッシュコードへの変換:");

    // 東京駅の座標（緯度35.6812度、経度139.7671度）
    let coord = Coordinate::new(35.6812, 139.7671).unwrap();
    println!("   座標: ({}, {})", coord.lat(), coord.lon());

    // 1次メッシュ（約80km四方）に変換
    let mesh_first = coord_to_mesh(coord, MeshLevel::First).unwrap();
    println!("   1次メッシュ: {}", mesh_first);

    // 2次メッシュ（約10km四方）に変換
    let mesh_second = coord_to_mesh(coord, MeshLevel::Second).unwrap();
    println!("   2次メッシュ: {}", mesh_second);

    // 3次メッシュ（約1km四方）に変換
    let mesh_third = coord_to_mesh(coord, MeshLevel::Third).unwrap();
    println!("   3次メッシュ: {}", mesh_third);

    // ========================================
    // 2. メッシュの境界を取得
    // ========================================
    println!("\n2. メッシュの境界を取得:");
    let bounds = mesh_to_bounds(mesh_third);
    println!("   南西端: ({:.6}, {:.6})", bounds.min_lat(), bounds.min_lon());
    println!("   北東端: ({:.6}, {:.6})", bounds.max_lat(), bounds.max_lon());

    // ========================================
    // 3. メッシュの中心座標を取得
    // ========================================
    println!("\n3. メッシュの中心座標を取得:");
    let center = mesh_to_center(mesh_third);
    println!("   中心座標: ({:.6}, {:.6})", center.lat(), center.lon());

    // ========================================
    // 4. 座標がメッシュ内に含まれるか確認
    // ========================================
    println!("\n4. 座標がメッシュ内に含まれるか確認:");
    let in_mesh = contains(mesh_third, coord);
    println!("   座標はメッシュ内にある: {}", in_mesh);

    // ========================================
    // 5. 親メッシュを取得
    // ========================================
    println!("\n5. 親メッシュを取得:");
    // 3次メッシュの親は2次メッシュになります
    if let Some(parent_mesh) = parent(mesh_third) {
        println!("   {} の親メッシュ: {}", mesh_third, parent_mesh);
    }

    // ========================================
    // 6. 子メッシュの数を取得
    // ========================================
    println!("\n6. 子メッシュの数を取得:");
    let mesh_second = MeshCode::from_str("533946").unwrap();
    let children_list = children(mesh_second);
    // 2次メッシュは100個の3次メッシュに分割されます
    println!("   {} の子メッシュ数: {} 個", mesh_second, children_list.len());

    // ========================================
    // 7. 文字列からメッシュコードをパース
    // ========================================
    println!("\n7. 文字列からメッシュコードをパース:");
    let mesh = MeshCode::from_str("5339").unwrap();
    println!("   パース結果: {} (レベル: {:?})", mesh, mesh.level());
}
