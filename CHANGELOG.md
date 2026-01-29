# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/uiuifree/rust-jismeshcode/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/uiuifree/rust-jismeshcode/releases/tag/v0.1.0
