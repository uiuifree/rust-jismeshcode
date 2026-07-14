# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-07-14

### Fixed（破壊的変更）
- **JIS X 0410準拠の修正**: 分割地域メッシュ・5次メッシュのエンコーディングを標準に準拠させた
  - 4次メッシュ（2分の1）: 分割番号を「南西=1、南東=2、北西=3、北東=4」に修正（従来は1と4が逆）
  - 4次メッシュ（4分の1）: 3次コード+2分の1番号(1〜4)+4分の1番号(1〜4)の階層形式に修正（従来は非標準の通し番号01〜16）
  - 4次メッシュ（8分の1）: 3次コード+分割番号(1〜4)×3桁の階層形式に修正（従来は非標準の通し番号001〜064）
  - 5次メッシュ（100m）: 3次コード+緯度方向番号(0〜9)+経度方向番号(0〜9)に修正（従来は+1オフセットにより北東端で桁あふれし、隣の3次メッシュのコードに化けるバグがあった）
- `no_std`ビルドの修復: `default-features = false`がコンパイル不能だった問題を修正（`libm`フィーチャーを追加）
- 10桁メッシュコードのレベル判定を修正: 9〜10桁目がともに1〜4の場合のみ4分の1メッシュと判定

### Added
- `serde`フィーチャーの実装（従来はフィーチャーフラグのみで実装が存在しなかった）
  - `MeshCode`はメッシュコード文字列としてシリアライズ
  - `Coordinate`/`BoundingBox`/`MeshLevel`/`Direction`に`Serialize`/`Deserialize`実装
- `MeshCode::new`/`from_str`のコード値検証（2次メッシュ番号0〜7、分割番号1〜4、桁あふれを拒否）
- `MeshCode`に`core::str::FromStr`/`From<MeshCode> for String`/`TryFrom<String>`実装
- `children`の階層拡張: 2分の1→4分の1→8分の1の子メッシュ取得に対応
- CIに`no_std`ターゲット（thumbv7em-none-eabihf）のビルドチェックを追加

### Changed
- `MeshLevel::parent`の階層を修正: 4分の1の親は2分の1、8分の1の親は4分の1（従来はいずれも3次）
- `to_level`は祖先レベルへの変換のみ許可（別系統レベルへの変換はエラー）
- 未使用の`thiserror`依存を削除

## [0.2.0] - 2026-01-30

### Added
- メッシュコードからの半径検索機能（`MeshCodeRadiusIterator`）
- Haversine公式による正確な距離計算（`haversine_distance`）
- 座標から半径指定でBoundingBoxを計算する機能（`calculate_bbox_offsets`）
- 半径検索の包括的なテストケース

## [0.1.0] - 2026-01-29

### Added
- 初回リリース
- 座標とメッシュコードの相互変換機能
- 全メッシュレベル（1次〜5次）対応
- 階層操作（親子メッシュ取得、レベル変換）
- 隣接メッシュ計算（8方向）
- 空間範囲検索（境界ボックス）
- `no_std`環境対応
- オプショナルなSerde対応
- 包括的なドキュメント（日本語・英語）
- 4つの詳細なサンプルコード
- 54個のテスト（単体テスト、統合テスト、JIS仕様準拠テスト）

### Features
- `std` (default): 標準ライブラリを使用
- `serde`: Serdeによるシリアライゼーション対応

[Unreleased]: https://github.com/uiuifree/rust-jismeshcode/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/uiuifree/rust-jismeshcode/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/uiuifree/rust-jismeshcode/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/uiuifree/rust-jismeshcode/releases/tag/v0.1.0
