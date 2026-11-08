# rust-code-best-ci

Rust Workspace向けのGitHub Actions品質保証テンプレートです。
`rust-quality-template.zip`と`RUST_CI_ARCHITECTURE.md`をもとに、
再利用可能なWorkflow、実装に対するKani証明、依存監査、週次Miriを用意しています。

| 検査 | PR・main push・merge queue | 毎週月曜 02:17 UTC | 手動 |
| --- | --- | --- | --- |
| rustfmt・Clippy・cargo-deny | 実行 | 実行 | 実行 |
| ビルド・Nextest・doctest（default / all features） | 実行 | 実行 | 実行 |
| Kani・証明ハーネス最低件数 | 実行 | 実行 | 実行 |
| Miri（ライブラリテスト） | 省略 | 実行 | `run_deep`で選択、初期値true |
| Quality gate | 実行 | 実行 | 実行 |

検査ジョブは並列実行します。`Quality gate`は必要な検査がすべて成功した場合だけ成功します。
失敗・キャンセルに加え、有効な検査の予期しないスキップも失敗として扱います。
Miri・Kaniを入力で無効化した場合だけ、そのジョブのスキップを許可します。

## ファイル

```text
.github/workflows/quality.yml                イベント・実行ポリシー
.github/workflows/rust-quality-reusable.yml  workflow_call対応の検査本体
.github/workflows/blaze.yml                  既存の手動ビルド実験（独立）
.github/dependabot.yml                       Action・Cargo依存の週次更新
deny.toml                                   依存監査ポリシー
src/lib.rs                                  実装・テスト・Kani証明の動作例
```

## 導入

1. `quality.yml`、`rust-quality-reusable.yml`、`deny.toml`をコピーします。
   必要なら`.github/dependabot.yml`もコピーしてください。
   `main`以外を使う場合は呼び出し側の`push.branches`を変更します。
2. `Cargo.lock`をコミットします。`deny.toml`のライセンス許可一覧と依存元の制限を、
   プロジェクトの方針に合わせてレビューしてください。既知の脆弱性は例外なしで検査し、
   未許可のライセンス・Git依存元・レジストリ・ワイルドカード依存指定を拒否します。
   同一crateの複数バージョンは警告です。
3. 実際のcrate内へ`#[cfg(kani)]`で証明を組み込みます。
   [src/lib.rs](src/lib.rs)は本番の`increment`関数そのものを呼び出す例です。
   `Cargo.toml`に以下を追加し、通常コンパイルでも独自cfgを認識させます。

   ```toml
   [lints.rust]
   unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)'] }
   ```

   Workspaceの共有lint設定を使う場合は`[workspace.lints.rust]`へ設定し、
   各メンバーで`[lints] workspace = true`を有効にしてください。
   Kaniを利用しないプロジェクトは呼び出し側で`run_formal: false`を明示します。
4. 初回のCIでチェック名を確認し、**`verify / Quality gate`**をブランチ保護・
   Rulesetsの必須チェックに指定します。GitHub側の表示名が異なる場合は実際の名前を選択します。
   このテンプレートを配置するだけではブランチ保護は有効になりません。
5. 定期実行失敗を追跡できるよう、GitHub Actionsの通知を有効にしてください。

## 再利用時の入力

| 入力 | 初期値 | 用途 |
| --- | --- | --- |
| `rust_toolchain` | `1.99.1` | fmt・Clippy・テスト・ツール導入に使うRust |
| `miri_toolchain` | `nightly-2026-10-25` | Miri用の日付固定nightly |
| `run_formal` | `true` | Kani検証を実行 |
| `run_deep` | `false` | Miri検査を実行 |
| `minimum_harnesses` | `1` | コンパイルで検出される証明ハーネスの最低件数。1以上 |

別リポジトリから直接利用する場合は、`uses`にこのリポジトリのフルコミットSHAを指定します。
checkout対象は呼び出し元リポジトリなので、そこに`Cargo.lock`・`deny.toml`・証明を配置してください。
同一runで複数回呼び出す場合は、Artifact名が衝突しないよう調整が必要です。

## 検証範囲と判断

- **全体を検査:** 旧CIのドキュメント差分スキップと差分fmtは採用していません。
  `include_str!`、build script、共有設定、Workspaceの逆依存による検査漏れを防ぎます。
  差分最適化は全体検査の実行時間を計測してから導入してください。
- **Feature:** defaultとall-featuresを別々にビルド・テストし、Clippyも両方で実行します。
  全組合せ・no-default-featuresは網羅しません。相互排他的なFeatureを持つ場合は、
  matrix、Clippy、Kaniのlist/検証、Miri、denyのgraph設定を対応する組合せに変更してください。
- **テスト:** Nextestで全ターゲットを実行し、doctestはCargoで別途実行します。
  テストが0件のcrateではNextestが失敗します。テストを追加するか運用方針を明示してください。
  DB・ポートを共有するテストは、プロジェクト固有のNextest設定で並列度を調整してください。
- **Kani:** `kani-verifier 0.68.0`を固定し、専用の互換toolchainは`kani setup`で導入します。
  `cargo kani list --format json`の実際のハーネス件数を検証してから、同じWorkspace・Feature条件で
  全証明を実行します。最低件数を増やすと、証明削除の検出にも利用できます。
  ファイル欠落・形式変更・0件・最低件数未満では失敗します。
  通常のCargo処理は`--locked`を使用します。Kani 0.68.0には同等のオプションがないため、
  事前の`cargo metadata --locked`と検証後の`Cargo.lock`差分確認を行います。
- **保証:** Kaniが示す性質は、ハーネスの仕様・仮定・探索範囲に限定されます。
  旧[Verusの証明例](proofs/increment.rs)は参考として残しますが、通常CIの対象はKaniです。
- **Miri:** 日付固定nightlyでライブラリテストを検査します。
  CLIのみのcrateは対象変更が必要です。FFI・ネットワークなどの非対応テストは分離してください。
  Miriを無効にした通常PRではMiri相当の保証は得られません。
- **プラットフォーム:** Linux x86_64が標準対象です。
  Windows/macOSの対応確認やMSRV保証が必要な製品では、別途テストmatrixを追加してください。

Kani一覧の仕様は[公式RFC](https://model-checking.github.io/kani/rfc/rfcs/0013-list.html)、
この固定版の実際のJSON形式は[0.68.0の実装](https://github.com/model-checking/kani/blob/kani-0.68.0/kani-driver/src/list/output.rs)で確認できます。
依存監査の設定は[cargo-deny公式ドキュメント](https://embarkstudios.github.io/cargo-deny/checks/index.html)を参照してください。

## 速度・安全性・保守

- Nextest `0.9.151`・cargo-deny `0.20.2`は固定バイナリを導入し、毎回のソースビルドを避けます。
  KaniのセットアップとMiriは専用ジョブへ分離しています。
- Cargoキャッシュはジョブ・toolchain・Featureで分離し、デフォルトブランチからだけ保存します。
  `CARGO_INCREMENTAL=0`とし、ジョブ間の`target/`転送は行いません。
- ビルド時間のHTMLレポートとKaniの証明一覧を14日間Artifactに保存します。
  `always()`で失敗時も保存を試みますが、ファイル生成前の失敗ではArtifactはありません。
- `contents: read`のみを付与し、checkoutの認証情報を残しません。
  外部ActionはフルSHA、Runnerは`ubuntu-24.04`、toolchainはバージョンで固定しています。
  `pull_request_target`や外部サービスの秘密情報は使いません。
- 新しいPR更新では前のPR実行をキャンセルし、push・週次・手動実行はキャンセルしません。
- DependabotはActionとCargo依存を更新します。YAML内のツールバージョン・Rust・nightlyは
  手動更新が必要です。Kani更新時はJSONスキーマも再確認してください。
- カバレッジ、mutation testing、sccache、mold、crate差分実行は標準構成に含めません。
  プロジェクトで必要な保証と計測結果に応じて追加してください。
- 既存の`blaze.yml`は手動のビルド速度評価用として保持しています。必須チェックには指定しません。

## ローカル確認

`cargo`がrustup proxyでない環境では、`cargo +VERSION`の代わりに
`rustup run VERSION cargo`を使用してください。

```sh
cargo +1.99.1 fmt --all -- --check
cargo +1.99.1 clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.99.1 build --workspace --all-targets --all-features --locked --timings
# Nextest・cargo-denyを上記の固定バージョンで導入してから実行
cargo +1.99.1 nextest run --workspace --all-targets --all-features --locked
cargo +1.99.1 test --workspace --doc --all-features --locked
cargo +1.99.1 deny --workspace --locked check
# default featuresでもClippy・ビルド・Nextest・doctestを実行

# Kani対応環境で実行
cargo +1.99.1 install kani-verifier --version 0.68.0 --locked
cargo +1.99.1 kani setup
cargo +1.99.1 kani list --workspace --all-features --format json
cargo +1.99.1 kani --workspace --all-features

# Miriコンポーネント導入後
cargo +nightly-2026-10-25 miri test --workspace --lib --all-features --locked
actionlint
```

本テンプレートの配置・ローカル検証と、GitHub-hosted Linux上の全ジョブ実行は別です。
移植先で初回のActions結果を確認してから必須チェックを設定してください。
