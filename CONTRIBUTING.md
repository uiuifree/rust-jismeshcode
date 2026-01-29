# Contributing to jismeshcode

jismeshcodeへの貢献に興味を持っていただきありがとうございます！

このドキュメントでは、プロジェクトへの貢献方法について説明します。

## 行動規範

このプロジェクトは、すべての貢献者に対して敬意を持って接することを期待しています。
建設的で友好的なコミュニティを維持するため、以下の点を心がけてください：

- 他者を尊重する
- 建設的なフィードバックを提供する
- 異なる視点や経験を歓迎する

## 貢献の方法

### バグ報告

バグを見つけた場合は、[Issues](https://github.com/uiuifree/jismeshcode/issues)で報告してください。

以下の情報を含めると、より早く問題を解決できます：

- 問題の簡潔な説明
- 再現手順
- 期待される動作
- 実際の動作
- 使用している環境（OS、Rustバージョンなど）
- 可能であればコードサンプル

### 機能リクエスト

新機能のアイデアがある場合は、[Issues](https://github.com/uiuifree/jismeshcode/issues)で提案してください。

以下の情報を含めると有用です：

- 機能の説明
- ユースケース（なぜこの機能が必要か）
- 可能であれば実装案

### プルリクエスト

コードの貢献は大歓迎です！以下の手順に従ってください：

1. **フォークとクローン**
   ```bash
   git clone https://github.com/yourusername/jismeshcode.git
   cd jismeshcode
   ```

2. **ブランチの作成**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **変更の実装**
   - コードスタイルガイドに従う（下記参照）
   - テストを追加する
   - ドキュメントを更新する

4. **テストの実行**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --all -- --check
   ```

5. **コミット**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

   コミットメッセージの形式：
   - `feat:` 新機能
   - `fix:` バグ修正
   - `docs:` ドキュメントのみの変更
   - `style:` コードフォーマット
   - `refactor:` リファクタリング
   - `test:` テストの追加・修正
   - `chore:` ビルドプロセスやツールの変更

6. **プッシュ**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **プルリクエストの作成**
   - GitHubでプルリクエストを作成
   - 変更内容を明確に説明
   - 関連するIssueがあれば参照

## コードスタイルガイド

### Rust コーディング規約

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)に従う
- `cargo fmt`でフォーマット
- `cargo clippy`の警告をすべて解決
- 公開APIには必ずドキュメントコメントを付ける

### ドキュメント

- すべての公開API（`pub`な関数、型、モジュール）にドキュメントコメントを付ける
- 日本語でドキュメントを記述（英語も歓迎）
- 可能な限りコード例（doctest）を含める
- 引数、戻り値、エラーについて説明する

例：
```rust
/// メッシュコードの親メッシュを取得する
///
/// 例えば、3次メッシュの親は2次メッシュになります。
/// 1次メッシュの場合は親が存在しないためNoneを返します。
///
/// # 引数
/// * `mesh` - 対象のメッシュコード
///
/// # 戻り値
/// 親メッシュコード、または親が存在しない場合はNone
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let mesh = MeshCode::from_str("53394611").unwrap();
/// let parent_mesh = parent(mesh).unwrap();
/// assert_eq!(parent_mesh.as_string(), "533946");
/// ```
pub fn parent(mesh: MeshCode) -> Option<MeshCode> {
    // ...
}
```

### テスト

- 新機能には必ずテストを追加
- エッジケースや境界条件をテスト
- 可能な限り高いカバレッジを目指す（目標: 90%以上）

テストの種類：
- **単体テスト**: 各関数の個別テスト（`#[cfg(test)]`モジュール内）
- **統合テスト**: `tests/`ディレクトリ内
- **Doctest**: ドキュメントコメント内のコード例

## 開発環境のセットアップ

### 必要なツール

- Rust 1.81以上
- cargo
- clippy
- rustfmt

### セットアップ手順

```bash
# リポジトリのクローン
git clone https://github.com/uiuifree/jismeshcode.git
cd jismeshcode

# 依存関係のインストール
cargo build

# テストの実行
cargo test

# ドキュメントの生成
cargo doc --open
```

### 便利なコマンド

```bash
# すべてのチェックを実行
cargo test && cargo clippy -- -D warnings && cargo fmt --all -- --check

# ベンチマークの実行
cargo bench

# サンプルの実行
cargo run --example basic_usage

# リリースビルド
cargo build --release
```

## リリースプロセス

（メンテナー向け）

1. `CHANGELOG.md`を更新
2. `Cargo.toml`のバージョンを更新
3. コミット: `git commit -am "chore: release v0.x.0"`
4. タグ作成: `git tag v0.x.0`
5. プッシュ: `git push && git push --tags`
6. crates.ioに公開: `cargo publish`

## 質問や議論

- バグや機能リクエストは[Issues](https://github.com/uiuifree/jismeshcode/issues)で
- 一般的な質問やアイデアは[Discussions](https://github.com/uiuifree/jismeshcode/discussions)で

## ライセンス

貢献したコードは、このプロジェクトのライセンス（MIT OR Apache-2.0）の下で公開されることに同意したものとみなされます。
