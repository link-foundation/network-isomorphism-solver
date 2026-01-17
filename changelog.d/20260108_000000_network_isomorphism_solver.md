---
bump: minor
---

### Added
- Implemented network isomorphism solver using Links Theory concepts
- `DoubletLink` struct representing ordered pairs (source, target)
- `LinkNetwork` struct for storing collections of doublet-links
- `from_notation()` method for parsing Links Notation (LiNo) format
- `is_isomorphic()` function for quick isomorphism checking
- `check_isomorphism()` function returning node mapping
- `find_automorphisms()` function for discovering network symmetries
- `contains_subnetwork()` function for pattern matching
- Weisfeiler-Lehman algorithm for efficient canonical labeling
- Comprehensive tests for 5 real-world use cases:
  - Drug Discovery (molecular structure comparison)
  - Circuit Verification (logic gate equivalence)
  - Computer Vision (pattern matching)
  - Social Network Analysis (community detection)
  - Cryptography (zero-knowledge proofs)
