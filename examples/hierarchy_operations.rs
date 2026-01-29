//! # メッシュの階層操作
//!
//! このサンプルでは、メッシュコードの階層構造を操作する方法を紹介します。
//! メッシュは階層構造になっており、粗い（広い）メッシュから細かい（狭い）メッシュへと
//! 細分化されています。
//!
//! ## メッシュの階層関係
//! - 1次メッシュ（約80km）は 64個の 2次メッシュに分割される
//! - 2次メッシュ（約10km）は 100個の 3次メッシュに分割される
//! - 3次メッシュ（約1km）は 4個の 4次メッシュ（2分の1）に分割される
//!
//! ## ユースケース
//! - 詳細なメッシュから広域のメッシュへ集約する
//! - 広域のメッシュから詳細なメッシュへ展開する
//! - 異なる精度のメッシュデータを統合する

use jismeshcode::prelude::*;

fn main() {
    println!("=== メッシュの階層操作 ===\n");

    // ========================================
    // 開始メッシュの設定
    // ========================================
    // 東京駅付近の3次メッシュ
    let third_mesh = MeshCode::from_str("53394611").unwrap();
    println!(
        "開始メッシュ: {} (レベル: {})",
        third_mesh,
        third_mesh.level() as u8
    );

    // ========================================
    // 1. 階層を上る（細かいメッシュ → 粗いメッシュ）
    // ========================================
    println!("\n1. 階層を上る（子 → 親へ遡る）:");

    // 3次メッシュから順に親メッシュを取得していく
    // 3次 → 2次 → 1次の順で遡る
    let mut current = third_mesh;
    let mut level = 3;

    while let Some(parent_mesh) = parent(current) {
        level -= 1;
        println!(
            "   {}次 → {}次: {} → {}",
            level + 1,
            level,
            current,
            parent_mesh
        );
        current = parent_mesh;
    }
    println!("   1次メッシュには親がありません");

    // ========================================
    // 2. 階層を下る（粗いメッシュ → 細かいメッシュ）
    // ========================================
    println!("\n2. 階層を下る（親 → 子へ展開）:");

    // 1次メッシュから開始
    let first = MeshCode::from_str("5339").unwrap();
    println!("   1次メッシュ: {}", first);

    // 1次メッシュの子（2次メッシュ）を取得
    // 1次メッシュは64個（8×8）の2次メッシュに分割される
    let second_children = children(first);
    println!("   → 2次メッシュの子供: {} 個", second_children.len());
    println!(
        "      最初の3個: {:?}",
        &second_children[0..3.min(second_children.len())]
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
    );

    // 2次メッシュの子（3次メッシュ）を取得
    // 2次メッシュは100個（10×10）の3次メッシュに分割される
    let second = MeshCode::from_str("533946").unwrap();
    let third_children = children(second);
    println!("\n   2次メッシュ {} の子供:", second);
    println!("   → 3次メッシュの子供: {} 個", third_children.len());
    println!(
        "      最初の5個: {:?}",
        &third_children[0..5.min(third_children.len())]
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
    );

    // ========================================
    // 3. レベル変換（直接親レベルへ変換）
    // ========================================
    println!("\n3. レベル変換（一気に親レベルへ変換）:");

    let mesh = MeshCode::from_str("53394611").unwrap();
    println!("   元のメッシュ: {} (レベル {})", mesh, mesh.level() as u8);

    // 3次メッシュから2次メッシュへ直接変換
    if let Ok(second) = to_level(mesh, MeshLevel::Second) {
        println!("   → 2次メッシュへ変換: {}", second);
    }

    // 3次メッシュから1次メッシュへ直接変換
    if let Ok(first) = to_level(mesh, MeshLevel::First) {
        println!("   → 1次メッシュへ変換: {}", first);
    }

    // 注意: 親から子へは変換できない（情報が足りないため）
    println!("\n   注意: 粗いメッシュから細かいメッシュへの変換はできません");
    println!("   （どの子メッシュか特定できないため）");

    // ========================================
    // 4. 各レベルのサイズ比較
    // ========================================
    println!("\n4. 各メッシュレベルのサイズ比較:");

    for level in [
        MeshLevel::First,
        MeshLevel::Second,
        MeshLevel::Third,
        MeshLevel::FourthHalf,
        MeshLevel::Fifth,
    ] {
        println!(
            "   {:?}メッシュ: 緯度{:.6}度 × 経度{:.6}度 (約{}m四方)",
            level,
            level.lat_size_degrees(),
            level.lon_size_degrees(),
            level.approximate_size_meters() as i32
        );
    }

    // ========================================
    // 5. 実用例: 子メッシュの数を確認
    // ========================================
    println!("\n5. 実用例: 各レベルの子メッシュ数");

    let first_mesh = MeshCode::from_str("5339").unwrap();
    let first_children = children(first_mesh);
    println!("   1次メッシュの子供: {} 個（8×8）", first_children.len());

    let second_mesh = MeshCode::from_str("533946").unwrap();
    let second_children = children(second_mesh);
    println!(
        "   2次メッシュの子供: {} 個（10×10）",
        second_children.len()
    );

    let third_mesh = MeshCode::from_str("53394611").unwrap();
    let third_children = children(third_mesh);
    println!("   3次メッシュの子供: {} 個（4分割）", third_children.len());
}
