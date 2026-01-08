use network_isomorphism_solver::{is_isomorphic, check_isomorphism, LinkNetwork};

fn main() {
    // XOR gate implementation 1
    let xor1 = LinkNetwork::from_notation(
        "
        A connects NAND1
        B connects NAND1
        A connects NAND2
        NAND1 connects NAND2
        B connects NAND3
        NAND1 connects NAND3
        NAND2 connects NAND4
        NAND3 connects NAND4
    ",
    );

    // XOR gate implementation 2 (same topology, different labels)
    let xor2 = LinkNetwork::from_notation(
        "
        X connects G1
        Y connects G1
        X connects G2
        G1 connects G2
        Y connects G3
        G1 connects G3
        G2 connects G4
        G3 connects G4
    ",
    );

    println!("XOR1: {} nodes, {} links", xor1.node_count(), xor1.link_count());
    println!("XOR2: {} nodes, {} links", xor2.node_count(), xor2.link_count());

    println!("XOR1 degree sequence: {:?}", xor1.degree_sequence());
    println!("XOR2 degree sequence: {:?}", xor2.degree_sequence());

    let result = check_isomorphism(&xor1, &xor2);
    println!("Isomorphic: {}", result.is_isomorphic);
    if let Some(mapping) = result.mapping {
        println!("Mapping: {:?}", mapping);
    }
}
