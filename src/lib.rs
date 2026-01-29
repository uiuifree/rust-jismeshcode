#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

//! # jismeshcode
//!
//! 日本標準地域メッシュコード（JIS X 0410）を扱うRustライブラリ
//!
//! ## 主な機能
//!
//! - 座標とメッシュコードの相互変換
//! - メッシュの階層操作（親子関係）
//! - 隣接メッシュの計算
//! - 空間範囲検索
//!
//! ## クイックスタート
//!
//! ```rust
//! use jismeshcode::prelude::*;
//!
//! // 座標からメッシュコードへ変換
//! let coord = Coordinate::new(35.6812, 139.7671).unwrap();
//! let mesh = coord_to_mesh(coord, MeshLevel::Third).unwrap();
//! println!("メッシュコード: {}", mesh);
//!
//! // メッシュの中心座標を取得
//! let center = mesh_to_center(mesh);
//! println!("中心座標: ({}, {})", center.lat(), center.lon());
//! ```

/// 座標とメッシュコードの変換機能
pub mod convert;
/// エラー型の定義
pub mod error;
/// メッシュの操作機能（階層、隣接など）
pub mod operations;
/// 空間検索機能
pub mod spatial;
/// 基本的な型定義
pub mod types;
/// ユーティリティ機能
pub mod utils;

/// よく使う型と関数を一括でインポートするためのprelude
pub mod prelude;

pub use convert::{coord_to_mesh, mesh_to_bounds, mesh_to_center};
pub use error::{CoordinateError, MeshCodeError, Result};
pub use operations::{bounds, center, children, contains, neighbor, neighbors, parent, to_level};
pub use spatial::{mesh_codes_in_bbox, MeshCodeIterator};
pub use types::{BoundingBox, Coordinate, Direction, MeshCode, MeshLevel};
