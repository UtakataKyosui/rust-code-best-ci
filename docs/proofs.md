# 証明台帳

Kani・Creusot・Verusで検証する性質を記録します。証明対象や前提を変えた場合は、このファイルと
proof harness、検証コマンドを同じPRで更新してください。テスト成功のために検証範囲を狭めないでください。

| 対象 | 性質 | ツール | 場所 | 前提・探索範囲 | 状態 |
| --- | --- | --- | --- | --- | --- |
| `increment` | 結果が数学的な後続値であり、`u64::MAX`のときだけ`None` | Kani 0.68.0 | [src/lib.rs](../src/lib.rs) | 全`u64`入力（上限なし） | CIで検証 |
| `increment` | 同上 | Verus | [proofs/increment.rs](../proofs/increment.rs) | 参考用。CIの対象外 | 未実行 |

実行コマンド:

```sh
cargo kani --workspace --all-features
```
