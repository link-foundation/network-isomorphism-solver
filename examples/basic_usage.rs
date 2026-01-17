//! Basic usage example for network-isomorphism-solver.
//!
//! This example demonstrates the core functionality using Links Theory.
//!
//! Run with: `cargo run --example basic_usage`

use network_isomorphism_solver::{
    check_isomorphism, contains_subnetwork, find_automorphisms, is_isomorphic, LinkNetwork,
};

fn main() {
    println!("Network Isomorphism Solver - Basic Usage Examples\n");
    println!("Based on Links Theory: https://habr.com/en/articles/895896\n");

    // Example 1: Creating networks and checking isomorphism
    println!("=== Example 1: Basic Isomorphism Check ===\n");

    let mut network1 = LinkNetwork::new();
    network1.add_link(1, 2);
    network1.add_link(2, 3);
    network1.add_link(3, 1);

    let mut network2 = LinkNetwork::new();
    network2.add_link(100, 200);
    network2.add_link(200, 300);
    network2.add_link(300, 100);

    println!("Network 1: Triangle with nodes 1, 2, 3");
    println!("Network 2: Triangle with nodes 100, 200, 300");
    println!(
        "Are they isomorphic? {}\n",
        is_isomorphic(&network1, &network2)
    );

    // Example 2: Using Links Notation
    println!("=== Example 2: Links Notation ===\n");

    let molecular_structure = LinkNetwork::from_notation(
        "
        # Benzene ring (6-membered carbon ring)
        C1 bonds C2
        C2 bonds C3
        C3 bonds C4
        C4 bonds C5
        C5 bonds C6
        C6 bonds C1
    ",
    );

    println!("Parsed benzene ring from Links Notation:");
    println!("  Nodes: {}", molecular_structure.node_count());
    println!("  Links: {}\n", molecular_structure.link_count());

    // Example 3: Getting the isomorphism mapping
    println!("=== Example 3: Isomorphism Mapping ===\n");

    let result = check_isomorphism(&network1, &network2);
    if result.is_isomorphic {
        println!("Networks are isomorphic!");
        if let Some(mapping) = result.mapping {
            println!("Node mapping from network1 to network2:");
            for (from, to) in mapping {
                println!("  {from} -> {to}");
            }
        }
    }
    println!();

    // Example 4: Finding automorphisms (symmetries)
    println!("=== Example 4: Network Symmetries ===\n");

    let automorphisms = find_automorphisms(&network1);
    println!(
        "Triangle has {} automorphisms (D3 symmetry group)",
        automorphisms.len()
    );
    println!("This includes all rotations and reflections of the triangle.\n");

    // Example 5: Subnetwork detection
    println!("=== Example 5: Subnetwork Detection ===\n");

    let large_network = LinkNetwork::from_notation(
        "
        A connects B
        B connects C
        C connects D
        D connects A
        A connects C
    ",
    );

    let triangle_pattern = LinkNetwork::from_notation(
        "
        X connects Y
        Y connects Z
        Z connects X
    ",
    );

    println!("Large network: Square with diagonal (5 links)");
    println!("Pattern: Triangle (3 links)");
    println!(
        "Does the large network contain the triangle pattern? {}",
        contains_subnetwork(&large_network, &triangle_pattern)
    );
    println!();

    // Example 6: Real-world use case - Molecular comparison
    println!("=== Example 6: Drug Discovery Use Case ===\n");

    let molecule1 = LinkNetwork::from_notation(
        "
        carbon1 bonds oxygen
        carbon1 bonds carbon2
        carbon2 bonds nitrogen
    ",
    );

    let molecule2 = LinkNetwork::from_notation(
        "
        C bonds O
        C bonds C2
        C2 bonds N
    ",
    );

    println!("Comparing two molecular structures...");
    println!(
        "Molecules are structurally equivalent: {}",
        is_isomorphic(&molecule1, &molecule2)
    );
    println!("\nThis can be used for pharmaceutical screening to identify");
    println!("structurally similar compounds with different atom labels.\n");

    println!("=== Summary ===");
    println!("The network isomorphism solver uses Links Theory's core concept:");
    println!("L -> L^2 (a link connects links)");
    println!("\nKey features demonstrated:");
    println!("- Network creation and manipulation");
    println!("- Links Notation parsing");
    println!("- Isomorphism detection with mapping");
    println!("- Symmetry (automorphism) analysis");
    println!("- Subnetwork/pattern detection");
}
