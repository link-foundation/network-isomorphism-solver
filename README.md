# Network Isomorphism Solver

A Rust library for determining network isomorphism using Links Theory concepts. This solver can check if two networks are structurally identical (isomorphic), find all automorphisms (symmetries), and detect subnetwork patterns.

[![CI/CD Pipeline](https://github.com/link-foundation/network-isomorphism-solver/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/link-foundation/network-isomorphism-solver/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

## Overview

Network isomorphism is a fundamental problem in computer science with applications across many domains:

- **Drug Discovery**: Comparing molecular structures to find similar compounds
- **Circuit Verification**: Checking if two circuit designs are functionally equivalent
- **Computer Vision**: Pattern matching in image recognition
- **Social Network Analysis**: Detecting community structures and fraud patterns
- **Cryptography**: Zero-knowledge proofs and graph-based cryptographic systems

This library uses **Links Theory** concepts where networks are represented as collections of doublet-links (ordered pairs of source and target nodes), following the L → L² formula where "a link connects links".

## Features

- **Isomorphism Detection**: Check if two networks have identical structure
- **Mapping Discovery**: Find the exact node-to-node correspondence between isomorphic networks
- **Automorphism Analysis**: Discover all symmetries within a network
- **Subnetwork Matching**: Find if one network contains another as a pattern
- **Links Notation Support**: Parse human-readable network descriptions
- **Weisfeiler-Lehman Algorithm**: Efficient canonical labeling for pruning the search space

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
network-isomorphism-solver = "0.2.0"
```

## Quick Start

```rust
use network_isomorphism_solver::{LinkNetwork, check_isomorphism};

// Create two networks using Links Notation
let benzene1 = LinkNetwork::from_notation("
    C1 connects C2
    C2 connects C3
    C3 connects C4
    C4 connects C5
    C5 connects C6
    C6 connects C1
");

let benzene2 = LinkNetwork::from_notation("
    A connects B
    B connects C
    C connects D
    D connects E
    E connects F
    F connects A
");

// Check if they are isomorphic
let result = check_isomorphism(&benzene1, &benzene2);

if result.is_isomorphic {
    println!("Networks are isomorphic!");
    if let Some(mapping) = result.mapping {
        println!("Node mapping: {:?}", mapping);
    }
}
```

## API Reference

### Core Types

#### `DoubletLink`
A link connecting two nodes (source → target).

```rust
pub struct DoubletLink {
    pub source: LinkId,
    pub target: LinkId,
}
```

#### `LinkNetwork`
A network represented as a collection of doublet-links.

```rust
impl LinkNetwork {
    /// Create an empty network
    pub fn new() -> Self;

    /// Parse from Links Notation
    pub fn from_notation(notation: &str) -> Self;

    /// Add a link between two nodes
    pub fn add_link(&mut self, source: LinkId, target: LinkId);

    /// Get node and link counts
    pub fn node_count(&self) -> usize;
    pub fn link_count(&self) -> usize;

    /// Get degree of a node
    pub fn degree(&self, node: LinkId) -> usize;

    /// Get sorted degree sequence
    pub fn degree_sequence(&self) -> Vec<usize>;
}
```

#### `IsomorphismResult`
Result of an isomorphism check.

```rust
pub struct IsomorphismResult {
    pub is_isomorphic: bool,
    pub mapping: Option<HashMap<LinkId, LinkId>>,
}
```

### Functions

#### `is_isomorphic(network1, network2) -> bool`
Quick check if two networks are isomorphic.

```rust
use network_isomorphism_solver::{LinkNetwork, is_isomorphic};

let net1 = LinkNetwork::from_notation("A connects B, B connects C");
let net2 = LinkNetwork::from_notation("X connects Y, Y connects Z");

assert!(is_isomorphic(&net1, &net2));
```

#### `check_isomorphism(network1, network2) -> IsomorphismResult`
Check isomorphism and return the node mapping if found.

```rust
use network_isomorphism_solver::{LinkNetwork, check_isomorphism};

let result = check_isomorphism(&net1, &net2);
if let Some(mapping) = result.mapping {
    // mapping: {A: X, B: Y, C: Z}
}
```

#### `find_automorphisms(network) -> Vec<HashMap<LinkId, LinkId>>`
Find all automorphisms (symmetries) of a network.

```rust
use network_isomorphism_solver::{LinkNetwork, find_automorphisms};

// Square has 8 automorphisms (rotations + reflections)
let square = LinkNetwork::from_notation("
    A connects B, B connects C, C connects D, D connects A
");
let autos = find_automorphisms(&square);
assert_eq!(autos.len(), 8);
```

#### `contains_subnetwork(haystack, needle) -> bool`
Check if a network contains another as a subnetwork pattern.

```rust
use network_isomorphism_solver::{LinkNetwork, contains_subnetwork};

let molecule = LinkNetwork::from_notation("
    C1 connects C2, C2 connects C3, C3 connects C4,
    C4 connects C5, C5 connects C6, C6 connects C1,
    C1 connects O
");

let ring = LinkNetwork::from_notation("
    A connects B, B connects C, C connects D,
    D connects E, E connects F, F connects A
");

assert!(contains_subnetwork(&molecule, &ring));
```

## Real-World Use Cases

### Drug Discovery
Compare molecular structures to find similar compounds:

```rust
let aspirin = LinkNetwork::from_notation("
    benzene connects carboxyl
    carboxyl connects acetyl
");

let candidate = LinkNetwork::from_notation("
    ring connects acid
    acid connects ester
");

if is_isomorphic(&aspirin, &candidate) {
    println!("Potential drug candidate found!");
}
```

### Circuit Verification
Verify that optimized circuits maintain functional equivalence:

```rust
let original = LinkNetwork::from_notation("
    input1 connects gate
    input2 connects gate
    gate connects output
");

let optimized = LinkNetwork::from_notation("
    A connects X
    B connects X
    X connects Y
");

assert!(is_isomorphic(&original, &optimized));
```

### Social Network Analysis
Detect fraud patterns in transaction networks:

```rust
let fraud_pattern = LinkNetwork::from_notation("
    hub connects node1
    hub connects node2
    hub connects node3
");

if contains_subnetwork(&transaction_graph, &fraud_pattern) {
    println!("Potential fraud detected!");
}
```

## Algorithm

The solver uses a combination of techniques:

1. **Quick Rejection**: Compare node counts, link counts, and degree sequences
2. **Weisfeiler-Lehman Refinement**: Compute canonical node signatures for pruning
3. **Backtracking Search**: Systematically explore valid node mappings
4. **Constraint Propagation**: Use degree and adjacency constraints to prune search

The Weisfeiler-Lehman algorithm iteratively refines node labels based on neighborhood structure, creating signatures that help identify potentially equivalent nodes.

## Links Theory Background

This library is inspired by [Links Theory](https://habr.com/ru/articles/903430/), which represents all data as doublet-links following the formula **L → L²** (a link connects links).

Key concepts:
- **Doublet-Link**: An ordered pair (source, target) representing a connection
- **Link Network**: A collection of doublet-links forming a structure
- **Links Notation (LiNo)**: Human-readable format for describing networks

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test module
cargo test drug_discovery
cargo test circuit_verification
```

### Code Quality

```bash
# Format code
cargo fmt

# Run lints
cargo clippy --all-targets --all-features

# Run all checks
cargo fmt --check && cargo clippy --all-targets --all-features && cargo test
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[Unlicense](LICENSE) - Public Domain

## Acknowledgments

- Inspired by [Links Theory](https://habr.com/ru/articles/903430/) by Konstantin Encyclopedist
- Uses concepts from the [Weisfeiler-Lehman graph isomorphism test](https://en.wikipedia.org/wiki/Weisfeiler_Leman_graph_isomorphism_test)
- Related projects: [links-notation](https://crates.io/crates/links-notation), [links-client](https://crates.io/crates/links-client)
