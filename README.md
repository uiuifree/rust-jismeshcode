# jismeshcode

[![Crates.io](https://img.shields.io/crates/v/jismeshcode.svg)](https://crates.io/crates/jismeshcode)
[![Documentation](https://docs.rs/jismeshcode/badge.svg)](https://docs.rs/jismeshcode)
[![CI](https://github.com/uiuifree/rust-jismeshcode/workflows/CI/badge.svg)](https://github.com/uiuifree/rust-jismeshcode/actions)
[![License](https://img.shields.io/crates/l/jismeshcode.svg)](https://github.com/uiuifree/rust-jismeshcode#license)

日本標準地域メッシュコード（JIS X 0410）を扱う包括的なRustライブラリです。

## 概要

`jismeshcode`は、地理座標と日本標準地域メッシュコード（JIS X 0410）を相互変換するライブラリです。1次メッシュ（約80km）から5次メッシュ（約100m）まで、すべてのメッシュレベルに対応しています。

## ユースケース

このライブラリは以下のような用途で活用できます：

- **統計データの集計・分析**: 政府統計データ（国勢調査、経済センサスなど）のメッシュ単位での集計
- **位置情報サービス**: GPS座標をメッシュコードに変換して空間インデックスを構築
- **GISアプリケーション**: 地図上での地域分析やヒートマップ生成
- **エリアマーケティング**: 商圏分析、人口動態分析、店舗配置最適化
- **環境データ分析**: 気象データや地質データのメッシュ単位での管理
- **都市計画**: 土地利用計画や交通量分析

## 特徴

### 🚀 高性能
- **ゼロコスト抽象化**: `Copy`トレイトによる効率的なメモリ管理
- **遅延評価**: イテレータを活用した大規模データの効率的処理
- **最適化されたアルゴリズム**: JIS X 0410に準拠した高速変換

### 📦 完全なJIS X 0410対応
- **全メッシュレベルサポート**: 1次（80km）〜5次（100m）まで完全対応
- **仕様準拠**: 総務省統計局の地域メッシュ仕様に完全準拠
- **実証済み**: 実際の統計データでの検証済み

### 🛠️ 使いやすいAPI
- **型安全**: Rustの型システムを活用した安全な設計
- **豊富なドキュメント**: 日本語と英語の両方で完全なドキュメント
- **実用的なサンプル**: 4つの詳細なサンプルコード付属

### 🌐 柔軟な環境対応
- **`no_std`対応**: 組み込み環境でも使用可能
- **オプショナルな依存関係**: 必要な機能だけを選択可能
- **Serdeサポート**: JSON/YAML等へのシリアライズに対応

### 📊 実用的な機能
- 座標とメッシュコードの相互変換
- 階層操作（親子メッシュの取得）
- 隣接メッシュの計算（8方向）
- 空間範囲検索（境界ボックス）
- **半径検索（Haversine距離計算）**
- メッシュの境界・中心座標計算

## メッシュレベル

- **1次メッシュ**: 約80km（4桁）
- **2次メッシュ**: 約10km（6桁）
- **3次メッシュ**: 約1km（8桁）
- **4次メッシュ（2分の1）**: 約500m（9桁）
- **4次メッシュ（4分の1）**: 約250m（10桁）
- **4次メッシュ（8分の1）**: 約125m（11桁）
- **5次メッシュ**: 約100m（10桁）

## 主要な型

### `MeshCode`

メッシュコードを表す型です。内部的にu64で表現され、`Copy`トレイトを実装しているため効率的に扱えます。

```rust,ignore
// 文字列からメッシュコードを作成
let mesh = MeshCode::from_str("53394611").unwrap();

// メッシュコードのレベルを取得
let level = mesh.level(); // MeshLevel::Third

// 文字列表現を取得
let code_str = mesh.as_string(); // "53394611"
```

### `Coordinate`

緯度経度座標を表す型です。日本の範囲内（北緯20-46度、東経122-154度）の座標のみを受け付けます。

```rust,ignore
// 座標を作成（範囲チェックあり）
let coord = Coordinate::new(35.6812, 139.7671).unwrap();

// 緯度・経度を取得
let lat = coord.lat(); // 35.6812
let lon = coord.lon(); // 139.7671
```

### `MeshLevel`

メッシュのレベル（次数）を表す列挙型です。

```rust,ignore
// 各メッシュレベル
MeshLevel::First        // 1次メッシュ（約80km）
MeshLevel::Second       // 2次メッシュ（約10km）
MeshLevel::Third        // 3次メッシュ（約1km）
MeshLevel::FourthHalf   // 4次メッシュ（2分の1、約500m）
MeshLevel::FourthQuarter // 4次メッシュ（4分の1、約250m）
MeshLevel::FourthEighth // 4次メッシュ（8分の1、約125m）
MeshLevel::Fifth        // 5次メッシュ（約100m）

// サイズ情報を取得
let lat_size = MeshLevel::Third.lat_size_degrees(); // 緯度方向のサイズ（度）
let lon_size = MeshLevel::Third.lon_size_degrees(); // 経度方向のサイズ（度）
let approx_size = MeshLevel::Third.approximate_size_meters(); // おおよそのサイズ（m）
```

### `BoundingBox`

矩形範囲（境界ボックス）を表す型です。空間範囲検索で使用します。

```rust,ignore
// 南西端と北東端の座標から境界ボックスを作成
let sw = Coordinate::new(35.6, 139.7).unwrap();
let ne = Coordinate::new(35.7, 139.8).unwrap();
let bbox = BoundingBox::new(sw, ne);

// 境界を取得
let min_lat = bbox.min_lat();
let max_lat = bbox.max_lat();
let min_lon = bbox.min_lon();
let max_lon = bbox.max_lon();

// 座標が範囲内にあるかチェック
let is_inside = bbox.contains(coord);

// 中心座標を取得
let center = bbox.center();
```

### `Direction`

隣接メッシュの方向を表す列挙型です。8方向に対応しています。

```rust,ignore
Direction::North      // 北
Direction::NorthEast  // 北東
Direction::East       // 東
Direction::SouthEast  // 南東
Direction::South      // 南
Direction::SouthWest  // 南西
Direction::West       // 西
Direction::NorthWest  // 北西

// すべての方向を取得
for dir in Direction::ALL {
    println!("{}", dir);
}
```

## 主要な関数

### 変換関数

```rust,ignore
// 座標からメッシュコードへ変換
let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();

// メッシュコードから境界ボックスへ変換
let bounds = mesh_to_bounds(mesh);

// メッシュコードから中心座標へ変換
let center = mesh_to_center(mesh);
```

### 階層操作

```rust,ignore
// 親メッシュを取得（3次 → 2次 → 1次）
let parent_mesh = parent(mesh).unwrap();

// 子メッシュをすべて取得（1次 → 64個の2次、2次 → 100個の3次）
let children_list = children(mesh);

// 指定レベルへ変換（細かいメッシュから粗いメッシュへのみ可能）
let first_mesh = to_level(mesh, MeshLevel::First).unwrap();
```

### 隣接メッシュ

```rust,ignore
// 指定方向の隣接メッシュを取得
let north_mesh = neighbor(mesh, Direction::North);

// すべての方向の隣接メッシュを取得（最大8個）
let all_neighbors = neighbors(mesh);
```

### 空間検索

```rust,ignore
// 境界ボックス内のメッシュコードをイテレータで取得
let bbox = BoundingBox::new(sw, ne);
for mesh in mesh_codes_in_bbox(bbox, MeshLevel::Third) {
    println!("{}", mesh);
}

// 座標から半径内のメッシュコードをイテレータで取得
let center = Coordinate::new(35.6812, 139.7671).unwrap();
for mesh in mesh_codes_in_radius(center, 1000.0, MeshLevel::Third) {
    println!("{}", mesh);
}

// メッシュコードから半径内のメッシュコードをイテレータで取得
let center_mesh = MeshCode::from_str("53394611").unwrap();
for mesh in mesh_codes_in_radius_from_mesh(center_mesh, 1000.0) {
    println!("{}", mesh);
}

// 2点間の距離を計算（Haversine公式）
let coord1 = Coordinate::new(35.6812, 139.7671).unwrap();
let coord2 = Coordinate::new(35.6895, 139.6917).unwrap();
let distance = haversine_distance(coord1, coord2);
println!("距離: {}m", distance);
```

### 境界・包含判定

```rust,ignore
// メッシュの境界を取得
let bounds = bounds(mesh);

// メッシュの中心座標を取得
let center = center(mesh);

// 座標がメッシュ内に含まれるか判定
let is_contained = contains(mesh, coord);
```

## 使用例

```rust,no_run
use jismeshcode::prelude::*;

// 座標からメッシュコードへ変換
let coord = Coordinate::new(35.6812, 139.7671).unwrap();
let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
println!("東京駅のメッシュコード: {}", mesh);

// メッシュの境界を取得
let bounds = mesh_to_bounds(mesh);
println!("南西端: {:?}, 北東端: {:?}", bounds.south_west(), bounds.north_east());

// メッシュの中心座標を取得
let center = mesh_to_center(mesh);
println!("中心座標: ({}, {})", center.lat(), center.lon());

// 隣接メッシュを取得
let all_neighbors = neighbors(mesh);
for neighbor in all_neighbors {
    println!("隣接メッシュ: {}", neighbor);
}

// 親メッシュと子メッシュ
let parent_mesh = parent(mesh).unwrap();
let children_list = children(parent_mesh);

// 半径検索（例：1km以内のメッシュを取得）
for nearby_mesh in mesh_codes_in_radius(coord, 1000.0, MeshLevel::Third) {
    let center = mesh_to_center(nearby_mesh);
    let distance = haversine_distance(coord, center);
    println!("メッシュ: {}, 距離: {:.0}m", nearby_mesh, distance);
}
```

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
jismeshcode = "0.2"
```

`no_std`環境の場合：

```toml
[dependencies]
jismeshcode = { version = "0.2", default-features = false }
```

`serde`対応が必要な場合：

```toml
[dependencies]
jismeshcode = { version = "0.2", features = ["serde"] }
```

## サンプルコード

より詳細な使用例は`examples/`ディレクトリを参照してください。

## ライセンス

以下のいずれかのライセンスを選択できます：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))
