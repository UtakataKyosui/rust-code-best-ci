# rust-code-best-ci
Rustのコード品質を確実かつ高速にチェックするためのCIテンプレートを作成

## 構成

| ジョブ | 役割 |
| --- | --- |
| `changes` | PRのmerge baseから変更を検出。mainへのpush・手動実行は全体検証 |
| `fmt` | PRでは変更されたRustファイルのみ、mainでは`cargo fmt --all` |
| `clippy` | ワークスペースの全ターゲット・全featureを`-D warnings`で検査 |
| `test` | 全ターゲットのビルド、cargo-nextest、Cargoのdoctest |
| `verus` | 仕様を記述した`proofs/*.rs`の全エントリーポイントを数学的に検証 |
| `Quality gate` | 上記の失敗・キャンセルを検出する、ブランチ保護用の固定チェック |

fmt・Clippy・テスト・証明は変更検出後に並列実行します。ビルドとnextestは
同じジョブ内で成果物を再利用し、大きな`target`のartifact転送を避けています。
nextest自身もテストを並列実行します。

通常のCIは[quality.yml](.github/workflows/quality.yml)、Blazeの評価は
[blaze.yml](.github/workflows/blaze.yml)です。最小ライブラリと境界値テストを同梱しているため、
このリポジトリでもそのままCIを実行できます。

## PR差分だけをチェックする範囲

- `pull_request`の作成・更新・再オープンで実行します。`git merge-base`と
  `git diff -z`を使い、空白を含むパス、追加・変更・コピー・リネームを扱います。
  削除ファイルはfmt対象外です。
- 差分fmtは**変更行ではなく変更ファイル全体**に対する`rustfmt --check`です。
  `skip_children=true`により、未変更の子モジュールへチェックが広がるのを防ぎます。
  Cargo/rustfmt設定変更時には全体fmtへ戻します。`proofs/`のVerus構文は対象外です。
- Clippy・ビルド・テストは、変更行や変更crateだけには限定しません。
  マクロ、build script、依存crate、feature間の影響を取りこぼさないためです。
- README・LICENSE・`docs/*.md`のみのPRは重い検査を省略します。
  `proofs/`のみの変更は証明を実行します。それ以外の不明なファイルや設定変更は
  Cargo検査と証明の両方を実行します。
  `include_str!`などでドキュメントをビルド入力にしている場合は、
  `changes`の`documentation()`から該当パスを除外してください。
- mainと手動実行は全体検証です。ワークフロー自体に`paths`フィルターを置かず、
  ドキュメントのみのPRでも`Quality gate`を完了させます。

## キャッシュと安全性

[Swatinem/rust-cache](https://github.com/Swatinem/rust-cache)を採用しています。
Rustのインストール後にCargoの依存ダウンロード・依存ビルド成果物を復元し、
mainからのみ保存します。Clippyとテストのキャッシュはジョブ別に分かれます。
ワークスペース自身のビルド成果物や、ジョブ間で生きているプロセスを保存するものではありません。
`CARGO_INCREMENTAL=0`でCIのディスク使用量を抑えます。

sccacheは必須ではありません。多数のジョブ・ターゲット間でコンパイル結果を共有したい場合は
[mozilla/sccache](https://github.com/mozilla/sccache)と`RUSTC_WRAPPER=sccache`を検討してください。
外部キャッシュの認証情報をfork PRに渡さないようにしてください。

ワークフローは`contents: read`のみで、checkout後に認証情報を残しません。
`pull_request_target`や信頼できないPRを実行する永続self-hosted runnerは使いません。
ActionsはコミットSHA、nextest・Verus・Blazeはリリースを固定しています。
VerusのZIPは固定SHA-256、Blazeは固定コミットのインストーラーによるリリースチェックサムで検証します。

## Verus

[Verus](https://github.com/verus-lang/verus)は任意のRustコードを自動的に証明するツールではなく、
`verus!`内で事前条件・事後条件・不変条件などを記述して検証します。
[proofs/increment.rs](proofs/increment.rs)は、オーバーフローしない後続値と
`u64::MAX`での`None`を証明する独立した例です。
Cargo版の実装と同じ計算ですが、Cargoの関数との等価性まで自動で証明しているわけではありません。
実際のアプリケーションでは検証対象の実装をVerusで管理するか、
[cargo-verus](https://verus-lang.github.io/verus/guide/cargo_verus.html)で統合してください。

Verusは専用のRust toolchainと同梱Z3を必要とするため、Cargoジョブから分離しています。
固定リリース`0.2026.11.02.8bf68ee`にはRust `1.99.1`を使用します。
更新時はバージョン・toolchain・ZIPチェックサムを一緒に更新してください。
エントリーポイントは`proofs/`直下、共有モジュールはサブディレクトリに置きます。
ソース変更でも全証明を再実行し、証明ファイルがゼロの場合は失敗させます。

## Blazeによる高速化の評価

[indexable-inc/blaze](https://github.com/indexable-inc/blaze)は、常駐エンジンを使って
変更後の再ビルドを高速化します。インストールだけで`cargo build`が高速化するわけではありません。
専用toolchainを使用するため、通常のCargoによる検証を必須とします。

Actions画面から**Blaze experiment → Run workflow**で実行できます。
固定リリース`0.20261027.0`で`blaze build`を2回実行し、cold/warmの時間を表示します。
2回目は変更なしのwarmビルドであり、
編集後の増分ビルドやCargoとの速度比較を測るベンチマークではありません。
この独立ワークフローはブランチ保護の必須チェックにはしません。

GitHub-hosted runnerはジョブごとに新規作成されるため、Blazeのdaemonのメモリは
次のCIへ引き継げません。cold buildの高速化や既存Cargo/nextestとの互換性は保証しません。
このリリースの`blaze test`は同梱サンプルでも標準ライブラリの未対応機能で失敗したため、
ワークフローはビルド評価のみとし、テストは通常CIのCargo/nextestで実行します。
Blaze自身のwarm VM CI runnerはまだ設計段階です。
大幅な高速化を求める場合も、まず実プロジェクトで互換性・メモリ量・増分ビルド時間を計測してください。
参考: [リリース時点のインストール方法](https://github.com/indexable-inc/blaze/blob/v0.20261027.0/docs/install.md)、
[CI runnerの状況](https://github.com/indexable-inc/blaze/blob/main/docs/ci-runner.md)。

## 導入・ローカル実行

1. `.github/workflows/`を自分のRustリポジトリへコピーします。
   デフォルトブランチがmainでなければ`branches`と`save-if`を変更してください。
2. `Cargo.lock`をコミットし、`rustfmt.toml`のeditionをプロジェクトに合わせます。
   混在editionのワークスペースは差分fmtを`cargo fmt --all -- --check`に置き換えてください。
   `--all-features`で両立しないfeatureがある場合は、対応するfeatureのmatrixに変更します。
   現在はstableを明示指定しているため、MSRVや固定toolchainを検査する場合は
   インストールと`+stable`を併せて変更してください。
3. `proofs/`へ検証対象を用意します。Verus不要の場合は`verus`ジョブと
   `Quality gate`の`needs`からその項目を除外してください。
4. ブランチ保護の必須チェックに**Quality gate**を設定します。

```sh
cargo +stable fmt --all -- --check
cargo +stable clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +stable build --workspace --all-targets --all-features --locked
cargo +stable install cargo-nextest --version 0.9.151 --locked
cargo +stable nextest run --workspace --all-targets --all-features --locked
cargo +stable test --workspace --doc --all-features --locked
# 公式リリースのZIPを検証・展開し、対応toolchainをインストールした後:
/path/to/verus-x86-linux/verus proofs/increment.rs
```

CIではnextestの固定バイナリを`taiki-e/install-action`で導入し、
毎回の`cargo install`によるツールのソースビルドを避けています。
