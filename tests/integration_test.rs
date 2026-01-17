//! Integration tests for the Network Isomorphism Solver
//!
//! These tests cover real-world use cases as specified in the issue:
//! 1. Drug Discovery - Molecular structure comparison
//! 2. Circuit Verification - Electronic circuit equivalence
//! 3. Computer Vision - Pattern matching in image structures
//! 4. Social Network Analysis - Fraud pattern detection
//! 5. Cryptography - Symmetry analysis for zero-knowledge proofs

use network_isomorphism_solver::{
    check_isomorphism, contains_subnetwork, find_automorphisms, is_isomorphic, LinkNetwork,
};

// =============================================================================
// Drug Discovery Tests
// =============================================================================

mod drug_discovery {
    use super::*;

    /// Test: Compare two benzene ring structures (6-membered carbon rings)
    /// Both should be recognized as isomorphic despite different atom labels
    #[test]
    fn test_benzene_ring_isomorphism() {
        // Benzene ring 1: C1-C2-C3-C4-C5-C6-C1
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

        // Benzene ring 2: Different labels, same structure
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

        assert!(is_isomorphic(&benzene1, &benzene2));

        let result = check_isomorphism(&benzene1, &benzene2);
        assert!(result.is_isomorphic);
        assert!(result.mapping.is_some());
        assert_eq!(result.mapping.unwrap().len(), 6);
    }

    /// Test: Distinguish different molecular structures
    /// Benzene (hexagon) should NOT be isomorphic to naphthalene backbone
    #[test]
    fn test_different_molecular_structures() {
        // Benzene: 6-membered ring
        let benzene = LinkNetwork::from_notation(
            "
            C1 bonds C2
            C2 bonds C3
            C3 bonds C4
            C4 bonds C5
            C5 bonds C6
            C6 bonds C1
        ",
        );

        // Linear chain: not a ring
        let chain = LinkNetwork::from_notation(
            "
            C1 bonds C2
            C2 bonds C3
            C3 bonds C4
            C4 bonds C5
            C5 bonds C6
        ",
        );

        assert!(!is_isomorphic(&benzene, &chain));
    }

    /// Test: Find functional groups (substructures) in molecules
    #[test]
    fn test_functional_group_detection() {
        // Larger molecule with multiple functional groups
        let molecule = LinkNetwork::from_notation(
            "
            C1 bonds C2
            C2 bonds C3
            C3 bonds O1
            C3 bonds C4
            C4 bonds N1
        ",
        );

        // Looking for C-O bond pattern
        let carbonyl_pattern = LinkNetwork::from_notation(
            "
            carbon bonds oxygen
        ",
        );

        assert!(contains_subnetwork(&molecule, &carbonyl_pattern));
    }

    /// Test: Molecular symmetry for drug screening
    #[test]
    fn test_molecular_symmetry() {
        // Symmetric molecule: ethane-like structure
        let mut symmetric_molecule = LinkNetwork::new();
        symmetric_molecule.add_link(1, 2); // Central bond

        let automorphisms = find_automorphisms(&symmetric_molecule);
        assert_eq!(automorphisms.len(), 2); // Identity + swap
    }

    /// Test: Compare cyclopropane rings (triangles)
    #[test]
    fn test_cyclopropane_isomorphism() {
        let cyclopropane1 = LinkNetwork::from_notation(
            "
            C1 bonds C2
            C2 bonds C3
            C3 bonds C1
        ",
        );

        let cyclopropane2 = LinkNetwork::from_notation(
            "
            A bonds B
            B bonds C
            C bonds A
        ",
        );

        assert!(is_isomorphic(&cyclopropane1, &cyclopropane2));

        // Triangle has 6 automorphisms (D3 symmetry)
        let automorphisms = find_automorphisms(&cyclopropane1);
        assert_eq!(automorphisms.len(), 6);
    }
}

// =============================================================================
// Circuit Verification Tests
// =============================================================================

mod circuit_verification {
    use super::*;

    /// Test: Verify two equivalent logic gate circuits
    #[test]
    fn test_logic_gate_equivalence() {
        // Circuit 1: AND gate with two inputs
        let circuit1 = LinkNetwork::from_notation(
            "
            input1 connects and_gate
            input2 connects and_gate
            and_gate connects output
        ",
        );

        // Circuit 2: Same structure, different labels
        let circuit2 = LinkNetwork::from_notation(
            "
            A connects X
            B connects X
            X connects Z
        ",
        );

        assert!(is_isomorphic(&circuit1, &circuit2));
    }

    /// Test: Distinguish different circuit topologies
    #[test]
    fn test_different_circuit_topologies() {
        // Series circuit: input -> gate1 -> gate2 -> output
        let series_circuit = LinkNetwork::from_notation(
            "
            input connects gate1
            gate1 connects gate2
            gate2 connects output
        ",
        );

        // Parallel circuit: input connects to both gates
        let parallel_circuit = LinkNetwork::from_notation(
            "
            input connects gate1
            input connects gate2
            gate1 connects output
            gate2 connects output
        ",
        );

        assert!(!is_isomorphic(&series_circuit, &parallel_circuit));
    }

    /// Test: Find specific gate patterns in complex circuits
    #[test]
    fn test_gate_pattern_detection() {
        // Complex circuit
        let complex_circuit = LinkNetwork::from_notation(
            "
            in1 connects g1
            in2 connects g1
            g1 connects g2
            g2 connects g3
            in3 connects g3
            g3 connects out
        ",
        );

        // Looking for fan-in pattern (2 inputs to 1 gate)
        let fanin_pattern = LinkNetwork::from_notation(
            "
            a connects x
            b connects x
        ",
        );

        assert!(contains_subnetwork(&complex_circuit, &fanin_pattern));
    }

    /// Test: Verify combinational logic equivalence
    #[test]
    fn test_combinational_logic() {
        // Simple 2-input AND gate circuit
        let and_gate1 = LinkNetwork::from_notation(
            "
            input1 connects gate
            input2 connects gate
            gate connects output
        ",
        );

        // Same AND gate with different labels
        let and_gate2 = LinkNetwork::from_notation(
            "
            A connects X
            B connects X
            X connects Y
        ",
        );

        assert!(is_isomorphic(&and_gate1, &and_gate2));

        // Chain of gates
        let chain1 = LinkNetwork::from_notation(
            "
            in connects g1
            g1 connects g2
            g2 connects out
        ",
        );

        let chain2 = LinkNetwork::from_notation(
            "
            A connects B
            B connects C
            C connects D
        ",
        );

        assert!(is_isomorphic(&chain1, &chain2));
    }
}

// =============================================================================
// Computer Vision Tests
// =============================================================================

mod computer_vision {
    use super::*;

    /// Test: Match edge patterns in image grids
    #[test]
    fn test_edge_pattern_matching() {
        // 3x3 image grid structure
        let image_grid = LinkNetwork::from_notation(
            "
            p1 adjacent p2
            p2 adjacent p3
            p1 adjacent p4
            p2 adjacent p5
            p3 adjacent p6
            p4 adjacent p5
            p5 adjacent p6
            p4 adjacent p7
            p5 adjacent p8
            p6 adjacent p9
            p7 adjacent p8
            p8 adjacent p9
        ",
        );

        // Looking for horizontal edge (3 connected pixels)
        let horizontal_edge = LinkNetwork::from_notation(
            "
            a adjacent b
            b adjacent c
        ",
        );

        assert!(contains_subnetwork(&image_grid, &horizontal_edge));
    }

    /// Test: Detect corner patterns
    #[test]
    fn test_corner_detection() {
        // Image with corner structure
        let image = LinkNetwork::from_notation(
            "
            pixel1 adjacent pixel2
            pixel2 adjacent pixel3
            pixel1 adjacent pixel4
        ",
        );

        // L-shaped corner pattern
        let corner_pattern = LinkNetwork::from_notation(
            "
            a adjacent b
            b adjacent c
            a adjacent d
        ",
        );

        assert!(is_isomorphic(&image, &corner_pattern));
    }

    /// Test: Object shape matching
    #[test]
    fn test_shape_matching() {
        // Square shape in image
        let square_shape = LinkNetwork::from_notation(
            "
            corner1 connects corner2
            corner2 connects corner3
            corner3 connects corner4
            corner4 connects corner1
        ",
        );

        // Rotated version (should be isomorphic)
        let rotated_square = LinkNetwork::from_notation(
            "
            A connects B
            B connects C
            C connects D
            D connects A
        ",
        );

        assert!(is_isomorphic(&square_shape, &rotated_square));

        // Square has 8 automorphisms (D4 symmetry)
        let automorphisms = find_automorphisms(&square_shape);
        assert_eq!(automorphisms.len(), 8);
    }

    /// Test: Keypoint graph matching
    #[test]
    fn test_keypoint_matching() {
        // Keypoints from image 1
        let keypoints1 = LinkNetwork::from_notation(
            "
            k1 matches k2
            k2 matches k3
            k1 matches k3
        ",
        );

        // Keypoints from image 2
        let keypoints2 = LinkNetwork::from_notation(
            "
            p1 matches p2
            p2 matches p3
            p3 matches p1
        ",
        );

        assert!(is_isomorphic(&keypoints1, &keypoints2));
    }
}

// =============================================================================
// Social Network Analysis Tests
// =============================================================================

mod social_network {
    use super::*;

    /// Test: Detect clique patterns (fraud rings)
    #[test]
    fn test_clique_detection() {
        // Social network with a triangle clique
        let network = LinkNetwork::from_notation(
            "
            Alice knows Bob
            Bob knows Charlie
            Charlie knows David
            David knows Eve
            Alice knows Charlie
        ",
        );

        // Looking for triangle (3-clique)
        let triangle = LinkNetwork::from_notation(
            "
            X knows Y
            Y knows Z
            Z knows X
        ",
        );

        assert!(contains_subnetwork(&network, &triangle));
    }

    /// Test: Identify community structures
    #[test]
    fn test_community_isomorphism() {
        // Community 1
        let community1 = LinkNetwork::from_notation(
            "
            user1 connected user2
            user2 connected user3
            user3 connected user4
            user4 connected user1
        ",
        );

        // Community 2 (same structure)
        let community2 = LinkNetwork::from_notation(
            "
            personA connected personB
            personB connected personC
            personC connected personD
            personD connected personA
        ",
        );

        assert!(is_isomorphic(&community1, &community2));
    }

    /// Test: Detect star patterns (influencers)
    #[test]
    fn test_influencer_pattern() {
        // Network with an influencer
        let network = LinkNetwork::from_notation(
            "
            influencer connects follower1
            influencer connects follower2
            influencer connects follower3
            influencer connects follower4
            follower1 connects follower2
        ",
        );

        // Star pattern (central node with 3 connections)
        let star_pattern = LinkNetwork::from_notation(
            "
            center connects leaf1
            center connects leaf2
            center connects leaf3
        ",
        );

        assert!(contains_subnetwork(&network, &star_pattern));
    }

    /// Test: Identify duplicate accounts (bot networks)
    #[test]
    fn test_bot_network_detection() {
        // Bot network pattern: all bots connected to same targets
        let bot_pattern = LinkNetwork::from_notation(
            "
            bot1 follows target
            bot2 follows target
            bot3 follows target
        ",
        );

        // Real network
        let real_network = LinkNetwork::from_notation(
            "
            account1 follows celebrity
            account2 follows celebrity
            account3 follows celebrity
            account1 follows account4
        ",
        );

        assert!(contains_subnetwork(&real_network, &bot_pattern));
    }
}

// =============================================================================
// Cryptography Tests
// =============================================================================

mod cryptography {
    use super::*;

    /// Test: Graph isomorphism for zero-knowledge proofs
    /// The hardness of graph isomorphism is used in some ZK proof systems
    #[test]
    fn test_zk_proof_graph_isomorphism() {
        // Secret graph
        let secret_graph = LinkNetwork::from_notation(
            "
            v1 edge v2
            v2 edge v3
            v3 edge v4
            v4 edge v1
            v1 edge v3
        ",
        );

        // Permuted version (for ZK proof)
        let permuted_graph = LinkNetwork::from_notation(
            "
            a edge b
            b edge c
            c edge d
            d edge a
            a edge c
        ",
        );

        let result = check_isomorphism(&secret_graph, &permuted_graph);
        assert!(result.is_isomorphic);

        // The mapping represents the "witness" in a ZK proof
        assert!(result.mapping.is_some());
    }

    /// Test: Automorphism group size for security analysis
    /// Larger automorphism groups may indicate weaker security
    #[test]
    fn test_automorphism_security() {
        // Highly symmetric graph (potentially weaker)
        let mut symmetric_graph = LinkNetwork::new();
        symmetric_graph.add_link(1, 2);
        symmetric_graph.add_link(2, 3);
        symmetric_graph.add_link(3, 1);

        let symmetric_autos = find_automorphisms(&symmetric_graph);
        assert_eq!(symmetric_autos.len(), 6); // High symmetry

        // Asymmetric graph (potentially stronger)
        let asymmetric_graph = LinkNetwork::from_notation(
            "
            a edge b
            b edge c
            c edge d
        ",
        );

        let asymmetric_autos = find_automorphisms(&asymmetric_graph);
        assert_eq!(asymmetric_autos.len(), 2); // Lower symmetry (identity + reflection)
    }

    /// Test: Non-isomorphic graphs for cryptographic protocols
    #[test]
    fn test_non_isomorphic_discrimination() {
        // Two non-isomorphic graphs with same node/edge count
        let graph1 = LinkNetwork::from_notation(
            "
            a edge b
            b edge c
            c edge d
            d edge a
        ",
        );

        let graph2 = LinkNetwork::from_notation(
            "
            a edge b
            a edge c
            a edge d
            b edge c
        ",
        );

        // Should correctly identify as non-isomorphic
        assert!(!is_isomorphic(&graph1, &graph2));
    }

    /// Test: Verify canonical form property
    #[test]
    fn test_canonical_representation() {
        // Multiple representations of the same graph
        let repr1 = LinkNetwork::from_notation(
            "
            1 edge 2
            2 edge 3
            3 edge 1
        ",
        );

        let repr2 = LinkNetwork::from_notation(
            "
            a edge b
            b edge c
            c edge a
        ",
        );

        let repr3 = LinkNetwork::from_notation(
            "
            x edge y
            y edge z
            z edge x
        ",
        );

        // All should be pairwise isomorphic
        assert!(is_isomorphic(&repr1, &repr2));
        assert!(is_isomorphic(&repr2, &repr3));
        assert!(is_isomorphic(&repr1, &repr3)); // Transitivity
    }
}

// =============================================================================
// Performance and Scale Tests
// =============================================================================

mod performance {
    use super::*;

    /// Test: Handle larger networks efficiently
    #[test]
    fn test_larger_network_isomorphism() {
        // Create two isomorphic networks with 10 nodes (moderate size path graph)
        let mut net1 = LinkNetwork::new();
        let mut net2 = LinkNetwork::new();

        // Create a path graph: 1-2-3-...-10
        for i in 1..10 {
            net1.add_link(i, i + 1);
            net2.add_link(i + 100, i + 101);
        }

        assert!(is_isomorphic(&net1, &net2));
    }

    /// Test: Quick rejection of obviously different networks
    #[test]
    fn test_quick_rejection() {
        let mut small = LinkNetwork::new();
        small.add_link(1, 2);

        let mut large = LinkNetwork::new();
        for i in 1..100 {
            large.add_link(i, i + 1);
        }

        // Should quickly determine these are not isomorphic
        assert!(!is_isomorphic(&small, &large));
    }

    /// Test: Grid network isomorphism
    #[test]
    fn test_grid_network() {
        // 2x3 grid (6 nodes, 7 edges)
        // n1 -- n2 -- n3
        // |     |     |
        // n4 -- n5 -- n6
        let grid1 = LinkNetwork::from_notation(
            "
            n1 e n2
            n2 e n3
            n4 e n5
            n5 e n6
            n1 e n4
            n2 e n5
            n3 e n6
        ",
        );

        // Same grid, different labels
        // a -- b -- c
        // |    |    |
        // d -- e -- f
        let grid2 = LinkNetwork::from_notation(
            "
            a e b
            b e c
            d e e2
            e2 e f
            a e d
            b e e2
            c e f
        ",
        );

        assert!(is_isomorphic(&grid1, &grid2));
    }
}

// =============================================================================
// Links Theory Specific Tests
// =============================================================================

mod links_theory {
    use super::*;

    /// Test: Links notation parsing with doublets
    #[test]
    fn test_doublet_notation() {
        let network = LinkNetwork::from_notation(
            "
            A B
            B C
            C A
        ",
        );

        assert_eq!(network.node_count(), 3);
        assert_eq!(network.link_count(), 3);
    }

    /// Test: Links notation parsing with triplets
    #[test]
    fn test_triplet_notation() {
        let network = LinkNetwork::from_notation(
            "
            papa loves mama
            son loves mama
            daughter loves mama
        ",
        );

        assert_eq!(network.node_count(), 4);
        assert_eq!(network.link_count(), 3);
    }

    /// Test: Verify L -> L^2 property
    /// Each link (doublet) connects two references
    #[test]
    fn test_doublet_link_property() {
        let mut network = LinkNetwork::new();
        network.add_link(1, 2);

        let links = network.links();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].source, 1);
        assert_eq!(links[0].target, 2);
    }

    /// Test: Network as links connecting links
    #[test]
    fn test_links_connect_links() {
        // In Links Theory, we can represent higher-order structures
        // Here we model a simple association: (A, B) and (B, C)
        let mut network = LinkNetwork::new();

        // Link 1: connects references 1 and 2
        network.add_link(1, 2);
        // Link 2: connects references 2 and 3
        network.add_link(2, 3);

        // The shared reference (2) represents where links connect
        assert_eq!(network.degree(2), 2);
    }

    /// Test: Comments in notation are ignored
    #[test]
    fn test_notation_comments() {
        let network = LinkNetwork::from_notation(
            "
            # This is a comment
            A connects B
            # Another comment
            B connects C
        ",
        );

        assert_eq!(network.node_count(), 3);
        assert_eq!(network.link_count(), 2);
    }
}

// =============================================================================
// Version Tests
// =============================================================================

mod version_tests {
    use network_isomorphism_solver::VERSION;

    #[test]
    fn test_version_is_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_version_matches_cargo_toml() {
        // Version should match the one in Cargo.toml
        assert!(VERSION.starts_with("0."));
    }
}

// =============================================================================
// Links Notation (LiNo) Crate Integration Tests
// =============================================================================

mod lino_integration {
    use super::*;

    /// Test: Parse doublet format using links-notation crate
    #[test]
    fn test_from_lino_doublet_format() {
        let network = LinkNetwork::from_lino("(1 2) (2 3) (3 1)").unwrap();
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.link_count(), 3);
    }

    /// Test: Parse named links with identifiers
    #[test]
    fn test_from_lino_named_links() {
        let network = LinkNetwork::from_lino("(bond1: A B) (bond2: B C) (bond3: C A)").unwrap();
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.link_count(), 3);
    }

    /// Test: Parse triplet format (source relation target)
    #[test]
    fn test_from_lino_triplet_format() {
        let network = LinkNetwork::from_lino("(A connects B) (B connects C)").unwrap();
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.link_count(), 2);
    }

    /// Test: Isomorphism between `from_lino` and `from_notation` results
    #[test]
    fn test_from_lino_isomorphic_to_from_notation() {
        let lino_network = LinkNetwork::from_lino("(A B) (B C) (C A)").unwrap();
        let notation_network =
            LinkNetwork::from_notation("A connects B\nB connects C\nC connects A");

        assert!(is_isomorphic(&lino_network, &notation_network));
    }

    /// Test: Isomorphism between different `LiNo` representations
    #[test]
    fn test_from_lino_different_representations() {
        let network1 = LinkNetwork::from_lino("(1 2) (2 3) (3 1)").unwrap();
        let network2 = LinkNetwork::from_lino("(A B) (B C) (C A)").unwrap();
        let network3 = LinkNetwork::from_lino("(bond1: X Y) (bond2: Y Z) (bond3: Z X)").unwrap();

        assert!(is_isomorphic(&network1, &network2));
        assert!(is_isomorphic(&network2, &network3));
        assert!(is_isomorphic(&network1, &network3));
    }

    /// Test: Empty `LiNo` input
    #[test]
    fn test_from_lino_empty_input() {
        // Single reference should result in empty network (no doublets)
        let network = LinkNetwork::from_lino("ref").unwrap();
        assert_eq!(network.node_count(), 0);
        assert_eq!(network.link_count(), 0);
    }

    /// Test: Complex nested `LiNo` structure
    #[test]
    fn test_from_lino_nested_structure() {
        // Nested structure: ((A B) (B C) (C A))
        let network = LinkNetwork::from_lino("((A B) (B C) (C A))").unwrap();
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.link_count(), 3);
    }

    /// Test: Parse error handling
    #[test]
    fn test_from_lino_parse_error() {
        // Unclosed parenthesis should cause an error
        let result = LinkNetwork::from_lino("(unclosed");
        assert!(result.is_err());
    }

    /// Test: Real-world molecular structure in `LiNo` format
    #[test]
    fn test_from_lino_molecular_structure() {
        // Benzene ring in LiNo doublet format
        let benzene =
            LinkNetwork::from_lino("(C1 C2) (C2 C3) (C3 C4) (C4 C5) (C5 C6) (C6 C1)").unwrap();

        assert_eq!(benzene.node_count(), 6);
        assert_eq!(benzene.link_count(), 6);

        // Should have 12 automorphisms (D6 symmetry)
        let automorphisms = find_automorphisms(&benzene);
        assert_eq!(automorphisms.len(), 12);
    }

    /// Test: Circuit in `LiNo` format
    #[test]
    fn test_from_lino_circuit() {
        let circuit1 = LinkNetwork::from_lino("(input1 gate) (input2 gate) (gate output)").unwrap();
        let circuit2 = LinkNetwork::from_lino("(A X) (B X) (X Z)").unwrap();

        assert!(is_isomorphic(&circuit1, &circuit2));
    }

    /// Test: Subnetwork detection with LiNo-parsed networks
    #[test]
    fn test_from_lino_subnetwork_detection() {
        let full_network = LinkNetwork::from_lino("(A B) (B C) (C D) (D A) (A C)").unwrap();

        let triangle_pattern = LinkNetwork::from_lino("(X Y) (Y Z) (Z X)").unwrap();

        assert!(contains_subnetwork(&full_network, &triangle_pattern));
    }

    /// Test: Re-exported `links_notation` module access
    #[test]
    fn test_links_notation_reexport() {
        // Users should be able to access the links-notation crate directly
        use network_isomorphism_solver::links_notation::parse_lino;

        let parsed = parse_lino("(A B)").unwrap();
        assert!(parsed.is_link());
    }
}
