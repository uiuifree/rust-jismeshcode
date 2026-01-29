//! # 隣接メッシュの分析
//!
//! このサンプルでは、指定したメッシュの隣接メッシュを取得する方法を紹介します。
//! メッシュは8方向（北、北東、東、南東、南、南西、西、北西）に隣接メッシュを持ちます。
//!
//! ## ユースケース
//! - ある地点の周辺エリアの情報を取得する
//! - メッシュ単位で範囲を広げながら検索する
//! - 隣接するメッシュを順番に処理する

use jismeshcode::prelude::*;

fn main() {
    println!("=== 隣接メッシュの分析 ===\n");

    // ========================================
    // 対象メッシュの設定
    // ========================================
    // 東京駅付近の3次メッシュ
    let mesh = MeshCode::from_str("53394611").unwrap();
    println!("対象メッシュ: {}", mesh);

    // メッシュの中心座標を取得
    let center = mesh_to_center(mesh);
    println!("中心座標: ({:.6}, {:.6})\n", center.lat(), center.lon());

    // ========================================
    // 各方向の隣接メッシュを個別に取得
    // ========================================
    println!("各方向の隣接メッシュ:");

    // Direction::ALLは8方向すべてを含む配列
    // 北、北東、東、南東、南、南西、西、北西の順番
    for dir in Direction::ALL.iter() {
        if let Some(neighbor_mesh) = neighbor(mesh, *dir) {
            // 隣接メッシュが存在する場合（日本の範囲内）
            let neighbor_center = mesh_to_center(neighbor_mesh);
            println!(
                "  {:10}: {} (中心: {:.6}, {:.6})",
                format!("{}", dir),
                neighbor_mesh,
                neighbor_center.lat(),
                neighbor_center.lon()
            );
        } else {
            // 隣接メッシュが範囲外の場合（海上など）
            println!("  {:10}: (範囲外)", format!("{}", dir));
        }
    }

    // ========================================
    // すべての隣接メッシュを一度に取得
    // ========================================
    println!("\nすべての隣接メッシュを一括取得:");

    // neighbors()関数は範囲内の隣接メッシュのみを返す
    // 最大8個、境界付近では少なくなる
    let all_neighbors = neighbors(mesh);
    println!("  見つかった隣接メッシュ数: {} 個", all_neighbors.len());

    // 各隣接メッシュを表示
    for (i, n) in all_neighbors.iter().enumerate() {
        println!("  {}: {}", i + 1, n);
    }

    // ========================================
    // 実用例: 隣接メッシュの連鎖的な取得
    // ========================================
    println!("\n実用例: 東方向への連鎖的な移動");
    let mut current = mesh;

    // 東方向に3回移動してみる
    for i in 1..=3 {
        if let Some(next) = neighbor(current, Direction::East) {
            println!("  {}回目の東隣: {}", i, next);
            current = next;
        } else {
            println!("  {}回目: これ以上東に進めません", i);
            break;
        }
    }
}
