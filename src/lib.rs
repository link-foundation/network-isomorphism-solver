#![allow(clippy::similar_names)]

//! Network Isomorphism Solver using Links Theory
//!
//! This library provides algorithms for determining if two networks (represented as links)
//! are isomorphic, meaning they are structurally identical via node relabeling.
//!
//! Based on [Links Theory](https://habr.com/en/articles/895896), this implementation
//! uses doublet-links (ordered pairs of references) as the fundamental data structure,
//! replacing traditional graph theory concepts of vertices and edges with a unified
//! link concept where "a link connects links".
//!
//! # Real-World Applications
//!
//! - **Drug Discovery**: Compare molecular structures to identify similar compounds
//! - **Circuit Verification**: Check if two electronic circuit designs are equivalent
//! - **Computer Vision**: Match patterns in image structures for object recognition
//! - **Social Network Analysis**: Detect isomorphic substructures in user networks
//! - **Cryptography**: Zero-knowledge proofs relying on graph isomorphism hardness
//!
//! # Links Notation Examples
//!
//! Networks can be described using Links Notation (LiNo):
//!
//! ```text
//! # Molecular structure (benzene ring)
//! C1 bonds C2
//! C2 bonds C3
//! C3 bonds C4
//! C4 bonds C5
//! C5 bonds C6
//! C6 bonds C1
//! ```
//!
//! # Example
//!
//! ```
//! use network_isomorphism_solver::{LinkNetwork, is_isomorphic};
//!
//! let mut network1 = LinkNetwork::new();
//! network1.add_link(1, 2);
//! network1.add_link(2, 3);
//! network1.add_link(3, 1);
//!
//! let mut network2 = LinkNetwork::new();
//! network2.add_link(10, 20);
//! network2.add_link(20, 30);
//! network2.add_link(30, 10);
//!
//! assert!(is_isomorphic(&network1, &network2));
//! ```

use std::collections::{HashMap, HashSet};

/// Package version (matches Cargo.toml version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A unique identifier for a link in the network.
/// In Links Theory, each link has its own reference enabling cross-references.
pub type LinkId = u64;

/// A doublet-link representing a connection between two link references.
/// This is the fundamental building block in Links Theory: L → L²
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DoubletLink {
    /// The source link reference
    pub source: LinkId,
    /// The target link reference
    pub target: LinkId,
}

impl DoubletLink {
    /// Creates a new doublet-link.
    ///
    /// # Arguments
    ///
    /// * `source` - The source link reference
    /// * `target` - The target link reference
    #[must_use]
    pub const fn new(source: LinkId, target: LinkId) -> Self {
        Self { source, target }
    }
}

/// A network of links representing interconnected structures.
///
/// In Links Theory, this replaces the traditional graph concept with a unified
/// link-based model where vertices and edges are both represented as links.
#[derive(Debug, Clone, Default)]
pub struct LinkNetwork {
    /// All doublet-links in the network
    links: Vec<DoubletLink>,
    /// Set of all unique link references (nodes)
    nodes: HashSet<LinkId>,
    /// Adjacency list: node -> set of connected nodes
    adjacency: HashMap<LinkId, HashSet<LinkId>>,
}

impl LinkNetwork {
    /// Creates a new empty link network.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a link network from Links Notation string.
    ///
    /// Parses simple triplet notation where each line represents a link:
    /// `source relationship target`
    ///
    /// # Arguments
    ///
    /// * `notation` - Links Notation string describing the network
    ///
    /// # Returns
    ///
    /// A new `LinkNetwork` parsed from the notation
    ///
    /// # Example
    ///
    /// ```
    /// use network_isomorphism_solver::LinkNetwork;
    ///
    /// let network = LinkNetwork::from_notation("
    ///     A connects B
    ///     B connects C
    ///     C connects A
    /// ");
    /// assert_eq!(network.node_count(), 3);
    /// assert_eq!(network.link_count(), 3);
    /// ```
    #[must_use]
    pub fn from_notation(notation: &str) -> Self {
        let mut network = Self::new();
        let mut name_to_id: HashMap<String, LinkId> = HashMap::new();
        let mut next_id: LinkId = 1;

        for line in notation.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // Triplet format: source relationship target
                let source_name = parts[0];
                let target_name = parts[parts.len() - 1];

                let source_id = *name_to_id
                    .entry(source_name.to_string())
                    .or_insert_with(|| {
                        let id = next_id;
                        next_id += 1;
                        id
                    });

                let target_id = *name_to_id
                    .entry(target_name.to_string())
                    .or_insert_with(|| {
                        let id = next_id;
                        next_id += 1;
                        id
                    });

                network.add_link(source_id, target_id);
            } else if parts.len() == 2 {
                // Doublet format: source target
                let source_name = parts[0];
                let target_name = parts[1];

                let source_id = *name_to_id
                    .entry(source_name.to_string())
                    .or_insert_with(|| {
                        let id = next_id;
                        next_id += 1;
                        id
                    });

                let target_id = *name_to_id
                    .entry(target_name.to_string())
                    .or_insert_with(|| {
                        let id = next_id;
                        next_id += 1;
                        id
                    });

                network.add_link(source_id, target_id);
            }
        }

        network
    }

    /// Adds a doublet-link to the network.
    ///
    /// # Arguments
    ///
    /// * `source` - The source link reference
    /// * `target` - The target link reference
    pub fn add_link(&mut self, source: LinkId, target: LinkId) {
        self.links.push(DoubletLink::new(source, target));
        self.nodes.insert(source);
        self.nodes.insert(target);

        self.adjacency.entry(source).or_default().insert(target);
        self.adjacency.entry(target).or_default().insert(source);
    }

    /// Returns the number of nodes (unique link references) in the network.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of links in the network.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Returns all nodes in the network.
    #[must_use]
    pub fn nodes(&self) -> &HashSet<LinkId> {
        &self.nodes
    }

    /// Returns all links in the network.
    #[must_use]
    pub fn links(&self) -> &[DoubletLink] {
        &self.links
    }

    /// Returns the degree (number of connections) of a node.
    #[must_use]
    pub fn degree(&self, node: LinkId) -> usize {
        self.adjacency.get(&node).map_or(0, HashSet::len)
    }

    /// Returns the neighbors of a node.
    #[must_use]
    pub fn neighbors(&self, node: LinkId) -> Option<&HashSet<LinkId>> {
        self.adjacency.get(&node)
    }

    /// Computes the degree sequence of the network (sorted list of degrees).
    #[must_use]
    pub fn degree_sequence(&self) -> Vec<usize> {
        let mut degrees: Vec<usize> = self.nodes.iter().map(|&n| self.degree(n)).collect();
        degrees.sort_unstable();
        degrees
    }
}

/// Represents a node signature for the Weisfeiler-Lehman algorithm.
/// This signature captures the structural neighborhood of a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeSignature {
    own_label: u64,
    neighbor_labels: Vec<u64>,
}

/// Computes node signatures using the Weisfeiler-Lehman algorithm.
/// This is a powerful heuristic for distinguishing non-isomorphic networks.
fn compute_wl_signatures(network: &LinkNetwork, iterations: usize) -> HashMap<LinkId, u64> {
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
fn compute_network_hash(network: &LinkNetwork) -> Vec<u64> {
    let signatures = compute_wl_signatures(network, 3);
    let mut hash: Vec<u64> = signatures.values().copied().collect();
    hash.sort_unstable();
    hash
}

/// Result of an isomorphism check, containing the mapping if isomorphic.
#[derive(Debug, Clone)]
pub struct IsomorphismResult {
    /// Whether the networks are isomorphic
    pub is_isomorphic: bool,
    /// The node mapping from network1 to network2 (if isomorphic)
    pub mapping: Option<HashMap<LinkId, LinkId>>,
}

impl IsomorphismResult {
    /// Creates a result indicating the networks are not isomorphic.
    #[must_use]
    const fn not_isomorphic() -> Self {
        Self {
            is_isomorphic: false,
            mapping: None,
        }
    }

    /// Creates a result indicating the networks are isomorphic with the given mapping.
    #[must_use]
    fn isomorphic(mapping: HashMap<LinkId, LinkId>) -> Self {
        Self {
            is_isomorphic: true,
            mapping: Some(mapping),
        }
    }
}

/// Checks if two link networks are isomorphic.
///
/// Uses a combination of fast invariant checks and the Weisfeiler-Lehman algorithm
/// with backtracking refinement for accurate isomorphism detection.
///
/// # Arguments
///
/// * `network1` - The first link network
/// * `network2` - The second link network
///
/// # Returns
///
/// `true` if the networks are isomorphic, `false` otherwise
///
/// # Example
///
/// ```
/// use network_isomorphism_solver::{LinkNetwork, is_isomorphic};
///
/// // Create two isomorphic triangle networks
/// let mut net1 = LinkNetwork::new();
/// net1.add_link(1, 2);
/// net1.add_link(2, 3);
/// net1.add_link(3, 1);
///
/// let mut net2 = LinkNetwork::new();
/// net2.add_link(100, 200);
/// net2.add_link(200, 300);
/// net2.add_link(300, 100);
///
/// assert!(is_isomorphic(&net1, &net2));
/// ```
#[must_use]
pub fn is_isomorphic(network1: &LinkNetwork, network2: &LinkNetwork) -> bool {
    check_isomorphism(network1, network2).is_isomorphic
}

/// Checks if two link networks are isomorphic and returns the mapping.
///
/// # Arguments
///
/// * `network1` - The first link network
/// * `network2` - The second link network
///
/// # Returns
///
/// An `IsomorphismResult` containing whether the networks are isomorphic
/// and the node mapping if they are
#[must_use]
pub fn check_isomorphism(network1: &LinkNetwork, network2: &LinkNetwork) -> IsomorphismResult {
    // Quick structural checks
    if network1.node_count() != network2.node_count() {
        return IsomorphismResult::not_isomorphic();
    }

    if network1.link_count() != network2.link_count() {
        return IsomorphismResult::not_isomorphic();
    }

    // Empty networks are isomorphic
    if network1.node_count() == 0 {
        return IsomorphismResult::isomorphic(HashMap::new());
    }

    // Check degree sequences
    if network1.degree_sequence() != network2.degree_sequence() {
        return IsomorphismResult::not_isomorphic();
    }

    // Check WL hashes
    let hash1 = compute_network_hash(network1);
    let hash2 = compute_network_hash(network2);

    if hash1 != hash2 {
        return IsomorphismResult::not_isomorphic();
    }

    // Group nodes by their WL signatures for both networks
    let sig1 = compute_wl_signatures(network1, 3);
    let sig2 = compute_wl_signatures(network2, 3);

    let mut label_to_nodes1: HashMap<u64, Vec<LinkId>> = HashMap::new();
    for (&node, &label) in &sig1 {
        label_to_nodes1.entry(label).or_default().push(node);
    }

    let mut label_to_nodes2: HashMap<u64, Vec<LinkId>> = HashMap::new();
    for (&node, &label) in &sig2 {
        label_to_nodes2.entry(label).or_default().push(node);
    }

    // Verify partitions match
    let mut partition1: Vec<usize> = label_to_nodes1.values().map(Vec::len).collect();
    let mut partition2: Vec<usize> = label_to_nodes2.values().map(Vec::len).collect();
    partition1.sort_unstable();
    partition2.sort_unstable();

    if partition1 != partition2 {
        return IsomorphismResult::not_isomorphic();
    }

    // Try to find a valid mapping using backtracking
    // Sort nodes by degree (ascending) for better pruning - start with boundary nodes
    let mut nodes1: Vec<LinkId> = network1.nodes().iter().copied().collect();
    nodes1.sort_by_key(|&n| network1.degree(n));
    let mut nodes2: Vec<LinkId> = network2.nodes().iter().copied().collect();
    nodes2.sort_by_key(|&n| network2.degree(n));

    if let Some(mapping) = find_isomorphism_mapping(network1, network2, &nodes1, &nodes2) {
        return IsomorphismResult::isomorphic(mapping);
    }

    IsomorphismResult::not_isomorphic()
}

/// Attempts to find a valid isomorphism mapping using backtracking.
fn find_isomorphism_mapping(
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
fn is_partial_consistent(
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
fn verify_mapping(
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

/// Finds all automorphisms of a network.
///
/// An automorphism is an isomorphism from a network to itself,
/// representing its symmetries.
///
/// # Arguments
///
/// * `network` - The link network to analyze
///
/// # Returns
///
/// A vector of all automorphism mappings
#[must_use]
pub fn find_automorphisms(network: &LinkNetwork) -> Vec<HashMap<LinkId, LinkId>> {
    let nodes: Vec<LinkId> = network.nodes().iter().copied().collect();
    let sig = compute_wl_signatures(network, 3);
    let mut automorphisms = Vec::new();
    let mut mapping: HashMap<LinkId, LinkId> = HashMap::new();
    let mut used: HashSet<LinkId> = HashSet::new();

    find_all_automorphisms(
        network,
        &nodes,
        &sig,
        0,
        &mut mapping,
        &mut used,
        &mut automorphisms,
    );

    automorphisms
}

/// Recursive helper to find all automorphisms.
fn find_all_automorphisms(
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

/// Checks if a network contains a subnetwork isomorphic to the pattern.
///
/// Useful for pattern matching in networks, such as finding molecular
/// substructures or detecting specific network motifs.
///
/// # Arguments
///
/// * `network` - The network to search in
/// * `pattern` - The pattern network to find
///
/// # Returns
///
/// `true` if the pattern exists as a subnetwork
#[must_use]
pub fn contains_subnetwork(network: &LinkNetwork, pattern: &LinkNetwork) -> bool {
    if pattern.node_count() > network.node_count() {
        return false;
    }

    if pattern.link_count() > network.link_count() {
        return false;
    }

    if pattern.node_count() == 0 {
        return true;
    }

    let pattern_nodes: Vec<LinkId> = pattern.nodes().iter().copied().collect();
    let network_nodes: Vec<LinkId> = network.nodes().iter().copied().collect();

    let mut mapping: HashMap<LinkId, LinkId> = HashMap::new();
    let mut used: HashSet<LinkId> = HashSet::new();

    subnetwork_backtrack(
        network,
        pattern,
        &pattern_nodes,
        &network_nodes,
        0,
        &mut mapping,
        &mut used,
    )
}

/// Recursive backtracking for subnetwork isomorphism.
fn subnetwork_backtrack(
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

#[cfg(test)]
mod tests {
    use super::*;

    mod link_network_tests {
        use super::*;

        #[test]
        fn test_new_network() {
            let network = LinkNetwork::new();
            assert_eq!(network.node_count(), 0);
            assert_eq!(network.link_count(), 0);
        }

        #[test]
        fn test_add_link() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            assert_eq!(network.node_count(), 2);
            assert_eq!(network.link_count(), 1);
        }

        #[test]
        fn test_degree() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            network.add_link(1, 3);
            network.add_link(1, 4);
            assert_eq!(network.degree(1), 3);
            assert_eq!(network.degree(2), 1);
        }

        #[test]
        fn test_degree_sequence() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            network.add_link(2, 3);
            network.add_link(3, 1);
            assert_eq!(network.degree_sequence(), vec![2, 2, 2]);
        }

        #[test]
        fn test_from_notation() {
            let network = LinkNetwork::from_notation(
                "
                A connects B
                B connects C
                C connects A
            ",
            );
            assert_eq!(network.node_count(), 3);
            assert_eq!(network.link_count(), 3);
        }

        #[test]
        fn test_from_notation_doublet() {
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
    }

    mod isomorphism_tests {
        use super::*;

        #[test]
        fn test_empty_networks_isomorphic() {
            let net1 = LinkNetwork::new();
            let net2 = LinkNetwork::new();
            assert!(is_isomorphic(&net1, &net2));
        }

        #[test]
        fn test_single_link_isomorphic() {
            let mut net1 = LinkNetwork::new();
            net1.add_link(1, 2);

            let mut net2 = LinkNetwork::new();
            net2.add_link(100, 200);

            assert!(is_isomorphic(&net1, &net2));
        }

        #[test]
        fn test_triangle_isomorphic() {
            let mut net1 = LinkNetwork::new();
            net1.add_link(1, 2);
            net1.add_link(2, 3);
            net1.add_link(3, 1);

            let mut net2 = LinkNetwork::new();
            net2.add_link(10, 20);
            net2.add_link(20, 30);
            net2.add_link(30, 10);

            assert!(is_isomorphic(&net1, &net2));
        }

        #[test]
        fn test_different_sizes_not_isomorphic() {
            let mut net1 = LinkNetwork::new();
            net1.add_link(1, 2);
            net1.add_link(2, 3);

            let mut net2 = LinkNetwork::new();
            net2.add_link(1, 2);

            assert!(!is_isomorphic(&net1, &net2));
        }

        #[test]
        fn test_different_structure_not_isomorphic() {
            // Line: 1-2-3-4
            let mut net1 = LinkNetwork::new();
            net1.add_link(1, 2);
            net1.add_link(2, 3);
            net1.add_link(3, 4);

            // Star: 1 connected to 2, 3, 4
            let mut net2 = LinkNetwork::new();
            net2.add_link(1, 2);
            net2.add_link(1, 3);
            net2.add_link(1, 4);

            assert!(!is_isomorphic(&net1, &net2));
        }

        #[test]
        fn test_square_isomorphic() {
            let mut net1 = LinkNetwork::new();
            net1.add_link(1, 2);
            net1.add_link(2, 3);
            net1.add_link(3, 4);
            net1.add_link(4, 1);

            let mut net2 = LinkNetwork::new();
            net2.add_link(10, 20);
            net2.add_link(20, 30);
            net2.add_link(30, 40);
            net2.add_link(40, 10);

            assert!(is_isomorphic(&net1, &net2));
        }

        #[test]
        fn test_isomorphism_returns_mapping() {
            let mut net1 = LinkNetwork::new();
            net1.add_link(1, 2);
            net1.add_link(2, 3);
            net1.add_link(3, 1);

            let mut net2 = LinkNetwork::new();
            net2.add_link(10, 20);
            net2.add_link(20, 30);
            net2.add_link(30, 10);

            let result = check_isomorphism(&net1, &net2);
            assert!(result.is_isomorphic);
            assert!(result.mapping.is_some());

            let mapping = result.mapping.unwrap();
            assert_eq!(mapping.len(), 3);
        }
    }

    mod automorphism_tests {
        use super::*;

        #[test]
        fn test_single_edge_automorphisms() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);

            let automorphisms = find_automorphisms(&network);
            // A single edge has 2 automorphisms: identity and swap
            assert_eq!(automorphisms.len(), 2);
        }

        #[test]
        fn test_triangle_automorphisms() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            network.add_link(2, 3);
            network.add_link(3, 1);

            let automorphisms = find_automorphisms(&network);
            // A triangle has 6 automorphisms (D3 symmetry group)
            assert_eq!(automorphisms.len(), 6);
        }

        #[test]
        fn test_line_automorphisms() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            network.add_link(2, 3);

            let automorphisms = find_automorphisms(&network);
            // A line 1-2-3 has 2 automorphisms: identity and reflection
            assert_eq!(automorphisms.len(), 2);
        }
    }

    mod subnetwork_tests {
        use super::*;

        #[test]
        fn test_contains_single_edge() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            network.add_link(2, 3);
            network.add_link(3, 1);

            let mut pattern = LinkNetwork::new();
            pattern.add_link(10, 20);

            assert!(contains_subnetwork(&network, &pattern));
        }

        #[test]
        fn test_contains_triangle_in_larger() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);
            network.add_link(2, 3);
            network.add_link(3, 1);
            network.add_link(3, 4);

            let mut pattern = LinkNetwork::new();
            pattern.add_link(10, 20);
            pattern.add_link(20, 30);
            pattern.add_link(30, 10);

            assert!(contains_subnetwork(&network, &pattern));
        }

        #[test]
        fn test_pattern_larger_than_network() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);

            let mut pattern = LinkNetwork::new();
            pattern.add_link(1, 2);
            pattern.add_link(2, 3);
            pattern.add_link(3, 1);

            assert!(!contains_subnetwork(&network, &pattern));
        }

        #[test]
        fn test_empty_pattern() {
            let mut network = LinkNetwork::new();
            network.add_link(1, 2);

            let pattern = LinkNetwork::new();

            assert!(contains_subnetwork(&network, &pattern));
        }
    }
}
