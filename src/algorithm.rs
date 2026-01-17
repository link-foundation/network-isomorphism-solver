//! Isomorphism algorithms including Weisfeiler-Lehman and backtracking.

use std::collections::{HashMap, HashSet};

use crate::{LinkId, LinkNetwork};

/// Represents a node signature for the Weisfeiler-Lehman algorithm.
/// This signature captures the structural neighborhood of a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeSignature {
    own_label: u64,
    neighbor_labels: Vec<u64>,
}

/// Computes node signatures using the Weisfeiler-Lehman algorithm.
/// This is a powerful heuristic for distinguishing non-isomorphic networks.
pub fn compute_wl_signatures(network: &LinkNetwork, iterations: usize) -> HashMap<LinkId, u64> {
    let mut labels: HashMap<LinkId, u64> = network
        .nodes()
        .iter()
        .map(|&node| (node, network.degree(node) as u64))
        .collect();

    // Get sorted nodes for deterministic iteration
    let mut sorted_nodes: Vec<LinkId> = network.nodes().iter().copied().collect();
    sorted_nodes.sort_unstable();

    for _ in 0..iterations {
        let mut new_labels: HashMap<LinkId, u64> = HashMap::new();
        let mut signature_to_label: HashMap<NodeSignature, u64> = HashMap::new();
        let mut next_label: u64 = 0;

        // Sort signatures to ensure deterministic label assignment
        let mut node_signatures: Vec<(LinkId, NodeSignature)> = Vec::new();

        for &node in &sorted_nodes {
            let mut neighbor_labels: Vec<u64> = network
                .neighbors(node)
                .map(|neighbors| neighbors.iter().map(|&n| labels[&n]).collect())
                .unwrap_or_default();
            neighbor_labels.sort_unstable();

            let signature = NodeSignature {
                own_label: labels[&node],
                neighbor_labels,
            };

            node_signatures.push((node, signature));
        }

        // Sort by signature to ensure consistent label assignment
        node_signatures.sort_by(|a, b| {
            a.1.own_label
                .cmp(&b.1.own_label)
                .then_with(|| a.1.neighbor_labels.cmp(&b.1.neighbor_labels))
        });

        for (node, signature) in node_signatures {
            let label = *signature_to_label.entry(signature).or_insert_with(|| {
                let l = next_label;
                next_label += 1;
                l
            });

            new_labels.insert(node, label);
        }

        labels = new_labels;
    }

    labels
}

/// Computes a canonical hash for the network based on WL signatures.
pub fn compute_network_hash(network: &LinkNetwork) -> Vec<u64> {
    let signatures = compute_wl_signatures(network, 3);
    let mut hash: Vec<u64> = signatures.values().copied().collect();
    hash.sort_unstable();
    hash
}

/// Attempts to find a valid isomorphism mapping using backtracking.
pub fn find_isomorphism_mapping(
    network1: &LinkNetwork,
    network2: &LinkNetwork,
    nodes1: &[LinkId],
    nodes2: &[LinkId],
) -> Option<HashMap<LinkId, LinkId>> {
    let mut mapping: HashMap<LinkId, LinkId> = HashMap::new();
    let mut used: HashSet<LinkId> = HashSet::new();

    backtrack(
        network1,
        network2,
        nodes1,
        nodes2,
        0,
        &mut mapping,
        &mut used,
    )
}

/// Recursive backtracking to find valid isomorphism mapping.
fn backtrack(
    network1: &LinkNetwork,
    network2: &LinkNetwork,
    nodes1: &[LinkId],
    nodes2: &[LinkId],
    idx: usize,
    mapping: &mut HashMap<LinkId, LinkId>,
    used: &mut HashSet<LinkId>,
) -> Option<HashMap<LinkId, LinkId>> {
    if idx == nodes1.len() {
        // Verify the complete mapping
        if verify_mapping(network1, network2, mapping) {
            return Some(mapping.clone());
        }
        return None;
    }

    let node1 = nodes1[idx];

    // Try mapping to each compatible node in network2
    for &node2 in nodes2 {
        if used.contains(&node2) {
            continue;
        }

        // Must have same degree
        if network1.degree(node1) != network2.degree(node2) {
            continue;
        }

        // Check partial consistency
        if !is_partial_consistent(network1, network2, mapping, node1, node2) {
            continue;
        }

        mapping.insert(node1, node2);
        used.insert(node2);

        if let Some(result) = backtrack(network1, network2, nodes1, nodes2, idx + 1, mapping, used)
        {
            return Some(result);
        }

        mapping.remove(&node1);
        used.remove(&node2);
    }

    None
}

/// Checks if adding a mapping is consistent with existing partial mapping.
pub fn is_partial_consistent(
    network1: &LinkNetwork,
    network2: &LinkNetwork,
    mapping: &HashMap<LinkId, LinkId>,
    node1: LinkId,
    node2: LinkId,
) -> bool {
    if let Some(neighbors1) = network1.neighbors(node1) {
        for &neighbor1 in neighbors1 {
            if let Some(&mapped_neighbor) = mapping.get(&neighbor1) {
                // Check if node2 is connected to mapped_neighbor in network2
                if let Some(neighbors2) = network2.neighbors(node2) {
                    if !neighbors2.contains(&mapped_neighbor) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    true
}

/// Verifies that a complete mapping is a valid isomorphism.
pub fn verify_mapping(
    network1: &LinkNetwork,
    network2: &LinkNetwork,
    mapping: &HashMap<LinkId, LinkId>,
) -> bool {
    for link in network1.links() {
        let Some(&mapped_source) = mapping.get(&link.source) else {
            return false;
        };
        let Some(&mapped_target) = mapping.get(&link.target) else {
            return false;
        };

        // Check if this edge exists in network2
        let edge_exists = network2
            .neighbors(mapped_source)
            .is_some_and(|n| n.contains(&mapped_target));

        if !edge_exists {
            return false;
        }
    }

    true
}

/// Recursive helper to find all automorphisms.
pub fn find_all_automorphisms(
    network: &LinkNetwork,
    nodes: &[LinkId],
    sig: &HashMap<LinkId, u64>,
    idx: usize,
    mapping: &mut HashMap<LinkId, LinkId>,
    used: &mut HashSet<LinkId>,
    automorphisms: &mut Vec<HashMap<LinkId, LinkId>>,
) {
    if idx == nodes.len() {
        if verify_mapping(network, network, mapping) {
            automorphisms.push(mapping.clone());
        }
        return;
    }

    let node1 = nodes[idx];
    let label1 = sig[&node1];

    for &node2 in nodes {
        if used.contains(&node2) {
            continue;
        }

        if sig[&node2] != label1 {
            continue;
        }

        if network.degree(node1) != network.degree(node2) {
            continue;
        }

        if !is_partial_consistent(network, network, mapping, node1, node2) {
            continue;
        }

        mapping.insert(node1, node2);
        used.insert(node2);

        find_all_automorphisms(network, nodes, sig, idx + 1, mapping, used, automorphisms);

        mapping.remove(&node1);
        used.remove(&node2);
    }
}

/// Recursive backtracking for subnetwork isomorphism.
pub fn subnetwork_backtrack(
    network: &LinkNetwork,
    pattern: &LinkNetwork,
    pattern_nodes: &[LinkId],
    network_nodes: &[LinkId],
    idx: usize,
    mapping: &mut HashMap<LinkId, LinkId>,
    used: &mut HashSet<LinkId>,
) -> bool {
    if idx == pattern_nodes.len() {
        return verify_subnetwork_mapping(network, pattern, mapping);
    }

    let pattern_node = pattern_nodes[idx];

    for &network_node in network_nodes {
        if used.contains(&network_node) {
            continue;
        }

        if network.degree(network_node) < pattern.degree(pattern_node) {
            continue;
        }

        if !is_partial_consistent(pattern, network, mapping, pattern_node, network_node) {
            continue;
        }

        mapping.insert(pattern_node, network_node);
        used.insert(network_node);

        if subnetwork_backtrack(
            network,
            pattern,
            pattern_nodes,
            network_nodes,
            idx + 1,
            mapping,
            used,
        ) {
            return true;
        }

        mapping.remove(&pattern_node);
        used.remove(&network_node);
    }

    false
}

/// Verifies that a mapping represents a valid subnetwork isomorphism.
fn verify_subnetwork_mapping(
    network: &LinkNetwork,
    pattern: &LinkNetwork,
    mapping: &HashMap<LinkId, LinkId>,
) -> bool {
    for link in pattern.links() {
        let Some(&mapped_source) = mapping.get(&link.source) else {
            return false;
        };
        let Some(&mapped_target) = mapping.get(&link.target) else {
            return false;
        };

        let edge_exists = network
            .neighbors(mapped_source)
            .is_some_and(|n| n.contains(&mapped_target));

        if !edge_exists {
            return false;
        }
    }

    true
}
