//! # 空間範囲検索
//!
//! このサンプルでは、指定した矩形範囲内に含まれるメッシュコードを検索する方法を紹介します。
//! 境界ボックス（BoundingBox）を使って、効率的に範囲内のメッシュを列挙できます。
//!
//! ## ユースケース
//! - 特定エリア内の統計データを集計する
//! - 地図上で選択された範囲のメッシュを取得する
//! - エリアマーケティング分析で対象メッシュを特定する

use jismeshcode::prelude::*;

fn main() {
    println!("=== 空間範囲検索 ===\n");

    // ========================================
    // 検索範囲の設定
    // ========================================
    // 東京駅周辺の矩形範囲を設定
    // 南西端: 北緯35.6度、東経139.7度
    // 北東端: 北緯35.7度、東経139.8度
    let sw = Coordinate::new(35.6, 139.7).unwrap();
    let ne = Coordinate::new(35.7, 139.8).unwrap();

    // 境界ボックスを作成
    let bbox = BoundingBox::new(sw, ne);

    println!("検索範囲:");
    println!("  南西端: 北緯{}, 東経{}", sw.lat(), sw.lon());
    println!("  北東端: 北緯{}, 東経{}", ne.lat(), ne.lon());
    println!(
        "  範囲: 緯度{:.2}度 × 経度{:.2}度",
        ne.lat() - sw.lat(),
        ne.lon() - sw.lon()
    );

    // ========================================
    // 3次メッシュ（約1km四方）で検索
    // ========================================
    println!("\n3次メッシュ（約1km四方）で検索:");

    // mesh_codes_in_bbox()はイテレータを返すため、
    // collect()で一度にすべて取得するか、
    // for文で1つずつ処理することができる
    let meshes: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Third).collect();
    println!("  見つかったメッシュ数: {} 個", meshes.len());

    // メッシュ数が少ない場合は全て表示、多い場合は一部のみ表示
    if meshes.len() <= 20 {
        println!("\n  全メッシュコード:");
        for mesh in &meshes {
            let center = mesh_to_center(*mesh);
            println!(
                "    {} - 中心座標: ({:.6}, {:.6})",
                mesh,
                center.lat(),
                center.lon()
            );
        }
    } else {
        println!("\n  最初の10個のメッシュコード:");
        for mesh in meshes.iter().take(10) {
            let center = mesh_to_center(*mesh);
            println!(
                "    {} - 中心座標: ({:.6}, {:.6})",
                mesh,
                center.lat(),
                center.lon()
            );
        }
        println!("    ... 他 {} 個", meshes.len() - 10);
    }

    // ========================================
    // 2次メッシュ（約10km四方）で検索
    // ========================================
    println!("\n2次メッシュ（約10km四方）で検索:");
    let meshes_2nd: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::Second).collect();
    println!("  見つかったメッシュ数: {} 個", meshes_2nd.len());

    // 2次メッシュは範囲が広いため、数が少なくなる
    if !meshes_2nd.is_empty() {
        println!("\n  2次メッシュコード一覧:");
        for mesh in &meshes_2nd {
            println!("    {}", mesh);
        }
    }

    // ========================================
    // 1次メッシュ（約80km四方）で検索
    // ========================================
    println!("\n1次メッシュ（約80km四方）で検索:");
    let meshes_1st: Vec<_> = mesh_codes_in_bbox(bbox, MeshLevel::First).collect();
    println!("  見つかったメッシュ数: {} 個", meshes_1st.len());

    // 1次メッシュは非常に広いため、通常は1〜2個程度
    for mesh in &meshes_1st {
        println!("    {}", mesh);
    }

    // ========================================
    // メモリ効率的な処理例
    // ========================================
    println!("\nメモリ効率的な処理例（collect()せずにイテレータで処理）:");

    // イテレータのままでループすると、
    // 大量のメッシュがある場合でもメモリを節約できる
    let mut count = 0;
    for _mesh in mesh_codes_in_bbox(bbox, MeshLevel::Third) {
        count += 1;
        // ここで各メッシュに対する処理を行う
        // 例: データベースから該当メッシュのデータを取得、集計など
        // let data = fetch_data_from_db(_mesh);
    }
    println!("  処理したメッシュ数: {} 個", count);
}
