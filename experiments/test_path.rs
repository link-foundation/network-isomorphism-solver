use network_isomorphism_solver::{LinkNetwork, is_isomorphic, check_isomorphism};

fn main() {
    // Small path graph test
    let mut net1 = LinkNetwork::new();
    net1.add_link(1, 2);
    net1.add_link(2, 3);
    net1.add_link(3, 4);
    
    let mut net2 = LinkNetwork::new();
    net2.add_link(100, 200);
    net2.add_link(200, 300);
    net2.add_link(300, 400);
    
    println!("Net1: {} nodes, {} links", net1.node_count(), net1.link_count());
    println!("Net2: {} nodes, {} links", net2.node_count(), net2.link_count());
    println!("Net1 degrees: {:?}", net1.degree_sequence());
    println!("Net2 degrees: {:?}", net2.degree_sequence());
    
    let result = check_isomorphism(&net1, &net2);
    println!("Isomorphic: {}", result.is_isomorphic);
    if let Some(mapping) = result.mapping {
        println!("Mapping: {:?}", mapping);
    }
}
