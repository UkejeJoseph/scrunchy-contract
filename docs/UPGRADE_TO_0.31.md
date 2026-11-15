# Upgrading to Anchor 0.31.0

## The Problem

Anchor 0.31.0 has a dependency conflict:
- `anchor-lang 0.31.0` needs `solana-program 2.3.0`
- `anchor-spl 0.31.0` with `associated_token` needs `solana-program 2.1.0`
- Cargo can't resolve both versions

## Solution: Remove `associated_token` Feature

We can work around this by **not using** the `associated_token` feature and handling ATAs manually. However, this requires code changes.

Let me try a different approach first - using only the `token` feature.

