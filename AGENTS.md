# AGENTS.md

AI Coding Agent向けの規約です。人間の開発者にも適用されます。

## 検証

変更後は、PRを出す前に次を実行し、結果を報告します（詳細は[justfile](justfile)）。

```sh
just check   # fmt・Clippy・テスト・doctest
just deny    # 依存監査
just kani    # 形式検証（Kani導入済みの環境）
```

実行できなかった検査は「未実行（NOT_RUN）」と明記し、成功（PASS）と区別します。

## 禁止事項

- テストの削除・無効化（`#[ignore]`）、アサーションの弱体化で検査を通すこと。
- 証明ハーネス（`#[kani::proof]`）の削除、`kani::assume`による前提の不当な拡大、
  探索範囲（`unwind`など）の縮小。
- CI設定・`deny.toml`・`rustfmt.toml`・`actions.lock`の変更を、PRの説明なしに含めること。
- CIの失敗を隠す変更（`continue-on-error`、必須チェックの除外、`|| true`）。
- `pull_request_target`の導入や、PRのコードにSecretsを渡すこと。

CI・証明・検証設定の変更は[CODEOWNERS](.github/CODEOWNERS)のレビュー対象です。
指示だけでは強制できないため、サーバ側の保護は[Rulesets](docs/rulesets.md)で設定します。

## 設計規約

- 最初は単一crate。長いだけのファイルはmoduleで分ける。crate分割は再利用・独立ビルド・依存隔離が必要な場合のみ。
- 新しいcrateを作るPRには、理由・公開API・依存方向を記載する。
- 証明対象を移動・変更したときは、proof harness・検証コマンド・[証明台帳](docs/proofs.md)を同じPRで更新する。
- 単体テストは`src`内の`cfg(test)`、統合テストは各packageの`tests/`に置く。
- 判断の背景は[docs/decisions.md](docs/decisions.md)を参照する。採用方針と実測済みを混同しない。

## Actionsの変更

`uses:`を追加・変更したら`gh actions-lock`で`actions.lock`を再生成し、差分をレビューする。
