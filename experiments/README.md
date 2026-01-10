# Experiments

This folder contains experiments for testing Links Theory dependencies.

## LiNo API Test (`src/main.rs`)

Demonstrates how to use the `links-notation` crate to parse Links Notation (LiNo) format:

- Simple doublet format: `(1 2)`
- Triplet format: `(A connects B)`
- Named links: `(link1: 1 2)`
- Multiple links: `(1 2) (2 3) (3 1)`

### Running

```bash
cd experiments
cargo run
```

## Notes on `doublets` crate

The `doublets` crate (from [linksplatform/doublets-rs](https://github.com/linksplatform/doublets-rs)) provides persistent storage for doublet-links but requires nightly Rust due to unstable features like `step_trait` and `trait_alias`.

To experiment with it, use nightly Rust:

```bash
rustup run nightly cargo build
```

The main `network-isomorphism-solver` library uses stable Rust and in-memory storage for broader compatibility.
