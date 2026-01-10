//! Network Isomorphism Solver CLI
//!
//! A command-line tool demonstrating network isomorphism detection
//! using Links Theory and Links Notation (`LiNo`).

use network_isomorphism_solver::{
    check_isomorphism, contains_subnetwork, find_automorphisms, is_isomorphic, LinkNetwork,
};

fn main() {
    println!(
        "Network Isomorphism Solver v{}",
        network_isomorphism_solver::VERSION
    );
    println!("Based on Links Theory: https://habr.com/en/articles/895896");
    println!("Using links-notation crate: https://crates.io/crates/links-notation\n");

    // Example 1: Drug Discovery - Comparing molecular structures
    println!("=== Example 1: Drug Discovery ===");
    println!("Comparing two molecular structures (benzene rings)\n");

    // Benzene ring structure (6-membered carbon ring)
    let benzene1 = LinkNetwork::from_notation(
        "
        C1 bonds C2
        C2 bonds C3
        C3 bonds C4
        C4 bonds C5
        C5 bonds C6
        C6 bonds C1
    ",
    );

    // Same structure with different atom labels
    let benzene2 = LinkNetwork::from_notation(
        "
        A bonds B
        B bonds C
        C bonds D
        D bonds E
        E bonds F
        F bonds A
    ",
    );

    let result = check_isomorphism(&benzene1, &benzene2);
    println!(
        "Benzene rings are isomorphic: {} (structures are equivalent)",
        result.is_isomorphic
    );
    if let Some(mapping) = result.mapping {
        println!("Atom mapping found: {mapping:?}\n");
    }

    // Example 2: Circuit Verification
    println!("=== Example 2: Circuit Verification ===");
    println!("Checking if two circuit designs are equivalent\n");

    // Circuit 1: Simple logic gate connections
    let circuit1 = LinkNetwork::from_notation(
        "
        input1 connects gate1
        input2 connects gate1
        gate1 connects gate2
        gate2 connects output
    ",
    );

    // Circuit 2: Different labeling, same topology
    let circuit2 = LinkNetwork::from_notation(
        "
        A connects X
        B connects X
        X connects Y
        Y connects Z
    ",
    );

    println!(
        "Circuits are equivalent: {}\n",
        is_isomorphic(&circuit1, &circuit2)
    );

    // Example 3: Social Network Analysis
    println!("=== Example 3: Social Network Analysis ===");
    println!("Detecting fraud patterns in user networks\n");

    // Full social network
    let social_network = LinkNetwork::from_notation(
        "
        Alice knows Bob
        Bob knows Charlie
        Charlie knows David
        David knows Alice
        Alice knows Charlie
        Bob knows David
    ",
    );

    // Suspicious pattern: tightly connected group
    let fraud_pattern = LinkNetwork::from_notation(
        "
        X knows Y
        Y knows Z
        Z knows X
    ",
    );

    println!(
        "Social network contains fraud pattern (triangle): {}\n",
        contains_subnetwork(&social_network, &fraud_pattern)
    );

    // Example 4: Network Symmetry Analysis (Cryptography)
    println!("=== Example 4: Cryptographic Analysis ===");
    println!("Finding symmetries in network structures\n");

    // Square network
    let mut square = LinkNetwork::new();
    square.add_link(1, 2);
    square.add_link(2, 3);
    square.add_link(3, 4);
    square.add_link(4, 1);

    let automorphisms = find_automorphisms(&square);
    println!(
        "Square network has {} automorphisms (symmetries)",
        automorphisms.len()
    );
    println!("This corresponds to the D4 dihedral group (4 rotations + 4 reflections)\n");

    // Example 5: Computer Vision - Pattern Matching
    println!("=== Example 5: Computer Vision ===");
    println!("Pattern matching in image structures\n");

    // Larger image structure
    let image_structure = LinkNetwork::from_notation(
        "
        pixel1 adjacent pixel2
        pixel2 adjacent pixel3
        pixel3 adjacent pixel4
        pixel4 adjacent pixel1
        pixel2 adjacent pixel4
    ",
    );

    // Looking for edge pattern
    let edge_pattern = LinkNetwork::from_notation(
        "
        A adjacent B
        B adjacent C
    ",
    );

    println!(
        "Image contains edge pattern: {}\n",
        contains_subnetwork(&image_structure, &edge_pattern)
    );

    // Example 6: Links Notation (LiNo) parsing
    println!("=== Example 6: Links Notation (LiNo) Parsing ===");
    println!("Using the links-notation crate for standard LiNo format\n");

    // Parse using from_lino with the links-notation crate
    let network1 = LinkNetwork::from_lino("(1 2) (2 3) (3 1)").expect("Failed to parse LiNo");
    let network2 = LinkNetwork::from_lino("(A B) (B C) (C A)").expect("Failed to parse LiNo");

    println!(
        "Network 1 from LiNo '(1 2) (2 3) (3 1)': {} nodes, {} links",
        network1.node_count(),
        network1.link_count()
    );
    println!(
        "Network 2 from LiNo '(A B) (B C) (C A)': {} nodes, {} links",
        network2.node_count(),
        network2.link_count()
    );
    println!(
        "Networks are isomorphic: {}\n",
        is_isomorphic(&network1, &network2)
    );

    // Parse named links
    let named_network = LinkNetwork::from_lino("(bond1: C1 C2) (bond2: C2 C3) (bond3: C3 C1)")
        .expect("Failed to parse named LiNo");
    println!(
        "Named links network: {} nodes, {} links",
        named_network.node_count(),
        named_network.link_count()
    );
    println!(
        "Named network isomorphic to network1: {}\n",
        is_isomorphic(&network1, &named_network)
    );

    println!("=== Summary ===");
    println!("The network isomorphism solver successfully demonstrated:");
    println!("1. Molecular structure comparison (drug discovery)");
    println!("2. Circuit equivalence verification");
    println!("3. Fraud pattern detection in social networks");
    println!("4. Symmetry analysis for cryptographic applications");
    println!("5. Pattern matching for computer vision");
    println!("6. Links Notation (LiNo) parsing with links-notation crate");
    println!("\nAll examples use Links Theory's doublet-link representation");
    println!("where L -> L^2 (a link connects links).");
    println!("\nLinks Notation library: https://github.com/link-foundation/links-notation");
}
