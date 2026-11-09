# 技術選定と未決事項の決定記録

Notionの「Rust CI 品質保証・技術選定ナレッジ」の机上調査を、このリポジトリの運用方針として記録します。
**採用方針は実測済みを意味しません。** 実測が必要な項目は[実証計画](validation-plan.md)で追跡します。

## 品質保証のレイヤー

| 層 | ツール | 実行タイミング | 現在の実装 |
| --- | --- | --- | --- |
| L1 | rustfmt・Clippy | PR・main | 実装済み |
| L2 | nextest・doctest・proptest | PR・main | nextest・doctestは実装済み、proptestは条件付き |
| L3 | Miri | 週次・手動 | 実装済み |
| L4 | Kani（Creusot・Verusは条件付き） | PR・main | Kaniは実装済み |
| L5 | cargo-llvm-cov・cargo-mutants | 週次・手動 | 手動・週次ジョブとして追加（`run_coverage`・`run_mutation`） |
| L6 | cargo-deny | PR・main | 実装済み |

## 決定一覧

| 項目 | 決定 | 理由・再評価の条件 |
| --- | --- | --- |
| 形式検証 | Kaniを初期採用 | 既存コードへ証明ハーネスを段階的に追加できる。保証は有界探索・仮定の範囲に限る |
| Creusot・Verus | 条件付き（Kaniで必要な性質を表せない場合のみ） | 注釈と保守の負担が大きい。採用時は対象関数と性質を[証明台帳](proofs.md)に記録する |
| Prusti | 保留 | 互換性調査が済むまで採用しない |
| テスト実行 | nextest。doctestは`cargo test --doc` | nextestはdoctestを実行しない |
| 依存監査 | cargo-deny | 脆弱性に加えてライセンス・依存元も検査できる |
| Property test | proptest（条件付き） | 入力空間が広い純粋関数を追加するときに導入する。依存は`dev-dependencies`に限定 |
| 未使用依存 | cargo-shear（条件付き） | 依存が増えた時点で`cargo shear`をQuality jobに追加する。現状は依存0件のため見送り |
| UB検出 | 週次Miri。sanitizersは必要時のみ | FFI・unsafeを含む場合にsanitizers専用ジョブを追加 |
| カバレッジ・変異 | cargo-llvm-cov・cargo-mutantsを週次・手動で実行 | 実行時間が長くPRの必須検査には向かない。閾値は実測後に決める |
| Cargoキャッシュ | Swatinem/rust-cache | `main`だけが保存する |
| sccache | **見送り** | ジョブ別に`cargo --timings`とキャッシュ有無を実測し、コンパイルが支配的と分かった場合に再評価する |
| リンカー（mold・LLD・wild） | **標準構成に含めない** | ターゲットごとの互換性が異なる。リンク時間が支配的と分かったプロジェクトで個別に導入する |
| cargo-chef | Docker案件のみ | このテンプレートはDockerを使わない |
| 差分ジョブ（paths-filter） | **採用しない** | 共有設定・`include_str!`・build scriptによる検査漏れを防ぐため、PRでも全体を検査する |
| crate逆依存の解析 | 見送り（全体検査を維持） | Workspaceが大きくなり、全体検査の所要時間が許容範囲を超えたら`cargo metadata`かguppyで設計する |
| ビルド時間の測定 | `cargo --timings`を採用 | テストジョブでArtifactとして保存済み |
| Blaze | 手動の評価用のまま | 必須検査には指定しない |

## 設計ルール

### CIファイルの分割

- Workflowは、トリガー・実行頻度・権限が異なる場合に分ける。行数だけを理由に分割しない。
- Jobは独立した品質ゲート、ランナー・toolchainが異なる単位で分ける。
- 複数ジョブで同じセットアップを繰り返すようになった場合だけComposite Actionへ切り出す。
- 差分検知が不確かな場合は全体検証へ戻す。
- PRの任意コードにSecretsを渡さない。必須チェックとCODEOWNERSを維持する。

### Rustの分割

- 最初は単一crate。長いだけのファイルはmoduleで分ける。
- crateは再利用・独立ビルド・依存隔離が必要な場合だけ分ける。
- `xtask`は、CIの差分解析や検証結果処理が複雑になった場合に導入する。それまでは`justfile`を窓口にする。
- 新しいcrateを作るときは、理由・公開API・依存方向をPRに記録する。

### AI Agentの運用

- 規約は[AGENTS.md](../AGENTS.md)に置く。指示だけでは強制できないため、Rulesetsと必須チェックを併用する。
- 設定は[Rulesetsの設定手順](rulesets.md)を参照する。
