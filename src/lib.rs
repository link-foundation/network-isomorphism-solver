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
//! This library uses the [`links-notation`](https://crates.io/crates/links-notation) crate
//! for parsing Links Notation (`LiNo`) format, which is the standard notation for describing
//! link structures.
//!
//! # Real-World Applications
//!
//! - **Drug Discovery**: Compare molecular structures to identify similar compounds
//! - **Circuit Verification**: Check if two electronic circuit designs are equivalent
//! - **Computer Vision**: Match patterns in image structures for object recognition
//! - **Social Network Analysis**: Detect isomorphic substructures in user networks
//! - **Cryptography**: Zero-knowledge proofs relying on graph isomorphism hardness
//!
//! # Links Notation (`LiNo`) Examples
//!
//! Networks can be described using Links Notation from the
//! [links-notation](https://github.com/link-foundation/links-notation) library:
//!
//! ```text
//! // Doublet format (source target)
//! (1 2) (2 3) (3 1)
//!
//! // Named links
//! (bond1: C1 C2) (bond2: C2 C3)
//!
//! // Nested structures
//! ((A B) (B C) (C A))
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

mod algorithm;

use std::collections::{HashMap, HashSet};

use algorithm::{
    compute_network_hash, compute_wl_signatures, find_all_automorphisms, find_isomorphism_mapping,
    subnetwork_backtrack,
};

// Re-export links-notation crate for users who want to parse LiNo directly
pub use links_notation;
use links_notation::{parse_lino, LiNo, ParseError};

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

    /// Creates a link network from Links Notation (`LiNo`) string using the
    /// [`links-notation`](https://crates.io/crates/links-notation) crate parser.
    ///
    /// This method parses proper `LiNo` format: `(source target)` doublets.
    /// For nested structures, it extracts the top-level doublets.
    ///
    /// # Arguments
    ///
    /// * `lino` - Links Notation string in `LiNo` format
    ///
    /// # Returns
    ///
    /// `Ok(LinkNetwork)` if parsing succeeds, `Err(ParseError)` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use network_isomorphism_solver::LinkNetwork;
    ///
    /// // Parse LiNo doublet format
    /// let network = LinkNetwork::from_lino("(1 2) (2 3) (3 1)").unwrap();
    /// assert_eq!(network.node_count(), 3);
    /// assert_eq!(network.link_count(), 3);
    ///
    /// // Parse named links
    /// let network2 = LinkNetwork::from_lino("(link1: A B) (link2: B C)").unwrap();
    /// assert_eq!(network2.node_count(), 3);
    /// assert_eq!(network2.link_count(), 2);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the `LiNo` string cannot be parsed
    pub fn from_lino(lino: &str) -> Result<Self, ParseError> {
        let parsed = parse_lino(lino)?;
        let mut network = Self::new();
        let mut name_to_id: HashMap<String, LinkId> = HashMap::new();
        let mut next_id: LinkId = 1;

        // Helper to get or create an ID for a reference name
        let mut get_id = |name: &str| -> LinkId {
            *name_to_id.entry(name.to_string()).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            })
        };

        // Extract doublets from the parsed LiNo structure
        Self::extract_doublets_from_lino(&parsed, &mut network, &mut get_id);

        Ok(network)
    }

    /// Recursively extracts doublet links from a `LiNo` structure.
    fn extract_doublets_from_lino<F>(lino: &LiNo<String>, network: &mut Self, get_id: &mut F)
    where
        F: FnMut(&str) -> LinkId,
    {
        match lino {
            LiNo::Ref(_) => {
                // Single reference, not a link
            }
            LiNo::Link { values, .. } => {
                // Check if this is a doublet (exactly 2 values that are both refs)
                if values.len() == 2 {
                    if let (LiNo::Ref(source), LiNo::Ref(target)) = (&values[0], &values[1]) {
                        let source_id = get_id(source);
                        let target_id = get_id(target);
                        network.add_link(source_id, target_id);
                        return;
                    }
                }

                // For triplets (source rel target), extract as doublet (source target)
                if values.len() == 3 {
                    if let (LiNo::Ref(source), LiNo::Ref(_), LiNo::Ref(target)) =
                        (&values[0], &values[1], &values[2])
                    {
                        let source_id = get_id(source);
                        let target_id = get_id(target);
                        network.add_link(source_id, target_id);
                        return;
                    }
                }

                // Recursively process nested links
                for value in values {
                    Self::extract_doublets_from_lino(value, network, get_id);
                }
            }
        }
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
