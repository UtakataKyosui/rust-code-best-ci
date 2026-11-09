# ローカル検証の窓口。CIと同じコマンドを実行します。
toolchain := env_var_or_default("RUST_TOOLCHAIN", "1.99.1")
cargo := "cargo +" + toolchain

default:
    @just --list

# fmt・Clippy・ビルド・テスト・doctest（default / all features）
check: fmt lint test

fmt:
    {{cargo}} fmt --all -- --check

lint:
    {{cargo}} clippy --workspace --all-targets --locked -- -D warnings
    {{cargo}} clippy --workspace --all-targets --all-features --locked -- -D warnings

test:
    {{cargo}} nextest run --workspace --all-targets --locked
    {{cargo}} nextest run --workspace --all-targets --all-features --locked
    {{cargo}} test --workspace --doc --locked
    {{cargo}} test --workspace --doc --all-features --locked

deny:
    {{cargo}} deny --workspace --locked check

# Kani（要: kani-verifier 0.68.0 と kani setup）
kani:
    {{cargo}} kani --workspace --all-features list --format json
    {{cargo}} kani --workspace --all-features
