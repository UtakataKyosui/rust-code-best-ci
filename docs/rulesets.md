# Rulesetsの設定手順

AGENTS.mdなどの指示だけでは、テスト削除やCI無効化を防げません。
サーバ側の保護として、[.github/rulesets/main.json](../.github/rulesets/main.json)を適用します。

| ルール | 目的 |
| --- | --- |
| `verify / Quality gate`を必須チェックに指定 | 全検査の成功を要求する。有効な検査のスキップも失敗扱い |
| Code ownerレビュー必須（[CODEOWNERS](../.github/CODEOWNERS)） | CI・証明・検証設定に加え、テストとKani証明を含む`src/`・`tests/`の変更を人間が確認する |
| レビュー後のpushで承認を取り消す | 承認後の検証弱体化を防ぐ |
| 会話の解決を必須にする | レビュー指摘の見落としを防ぐ |
| 削除・強制pushの禁止 | 履歴の改ざんを防ぐ |

## 適用方法

このファイルを置くだけでは有効になりません。リポジトリ管理者が適用します。

1. GitHubの **Settings → Rules → Rulesets → New ruleset → Import a ruleset** で`main.json`を読み込みます。
2. 必須チェック名が実際の表示名と一致するか確認します。異なる場合は選択し直します。
3. 管理者のバイパスが不要な場合は`bypass_actors`を空のままにします。
4. 検証: テストを削除するだけのPRが、CODEOWNERSの承認なしにマージできないことを確認し、
   実証計画（`docs/validation-plan.md`）に結果を記録します。

CODEOWNERSのオーナーはこのリポジトリの所有者に設定しています。`src/`全体を対象にするため、通常の実装変更もオーナーの承認が必要です。
テストを別ディレクトリへ分離している場合は、対象パスを調整してください。移植先では自分のチームへ置き換えてください。
