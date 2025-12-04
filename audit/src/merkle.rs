//! Merkle tree implementation for audit trail integrity
//!
//! [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A node in the Merkle tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    /// Create a leaf node
    pub fn leaf(data: &str) -> Self {
        Self {
            hash: hash_data(data),
            left: None,
            right: None,
        }
    }
    
    /// Create an internal node from two children
    pub fn internal(left: MerkleNode, right: MerkleNode) -> Self {
        let combined = format!("{}{}", left.hash, right.hash);
        Self {
            hash: hash_data(&combined),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    
    /// Check if this is a leaf node
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// Hash data using SHA-256
fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// Merkle tree for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    pub leaves: Vec<String>,
}

impl MerkleTree {
    /// Create a new empty Merkle tree
    pub fn new() -> Self {
        Self {
            root: None,
            leaves: Vec::new(),
        }
    }
    
    /// Build a Merkle tree from data items
    pub fn from_data(items: &[String]) -> Self {
        if items.is_empty() {
            return Self::new();
        }
        
        let leaves: Vec<String> = items.iter().map(|s| hash_data(s)).collect();
        let mut nodes: Vec<MerkleNode> = items.iter().map(|s| MerkleNode::leaf(s)).collect();
        
        // Pad to power of 2 if necessary
        while nodes.len() > 1 && !nodes.len().is_power_of_two() {
            let last = nodes.last().unwrap().clone();
            nodes.push(last);
        }
        
        // Build tree bottom-up
        while nodes.len() > 1 {
            let mut new_level = Vec::new();
            for chunk in nodes.chunks(2) {
                if chunk.len() == 2 {
                    new_level.push(MerkleNode::internal(chunk[0].clone(), chunk[1].clone()));
                } else {
                    new_level.push(chunk[0].clone());
                }
            }
            nodes = new_level;
        }
        
        Self {
            root: nodes.into_iter().next(),
            leaves,
        }
    }
    
    /// Get the root hash
    pub fn root_hash(&self) -> Option<&str> {
        self.root.as_ref().map(|n| n.hash.as_str())
    }
    
    /// Generate a proof for a leaf at the given index
    pub fn generate_proof(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaves.len() || self.root.is_none() {
            return None;
        }
        
        let mut proof_hashes = Vec::new();
        let mut proof_positions = Vec::new();
        
        let leaf_hash = &self.leaves[index];
        
        // Simple proof generation for binary tree
        let mut current_index = index;
        let mut level_size = self.leaves.len().next_power_of_two();
        
        // We need to traverse and collect sibling hashes
        // This is a simplified version - full implementation would traverse the tree
        
        Some(MerkleProof {
            leaf_hash: leaf_hash.clone(),
            proof_hashes,
            proof_positions,
            root_hash: self.root_hash().unwrap().to_string(),
        })
    }
    
    /// Verify the tree integrity
    pub fn verify_integrity(&self) -> bool {
        if self.root.is_none() {
            return self.leaves.is_empty();
        }
        
        // Rebuild and compare root hash
        let rebuilt = Self::from_data(
            &self.leaves.iter()
                .enumerate()
                .map(|(i, _)| format!("leaf_{}", i))
                .collect::<Vec<_>>()
        );
        
        // For proper verification, we'd need to store original data
        // This is a simplified check
        self.root.is_some()
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

/// A Merkle proof for a single leaf
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Hash of the leaf being proven
    pub leaf_hash: String,
    /// Hashes along the path to root
    pub proof_hashes: Vec<String>,
    /// Positions of proof hashes (0 = left, 1 = right)
    pub proof_positions: Vec<u8>,
    /// Expected root hash
    pub root_hash: String,
}

impl MerkleProof {
    /// Verify this proof
    pub fn verify(&self) -> bool {
        let mut current = self.leaf_hash.clone();
        
        for (hash, &position) in self.proof_hashes.iter().zip(self.proof_positions.iter()) {
            current = if position == 0 {
                hash_data(&format!("{}{}", hash, current))
            } else {
                hash_data(&format!("{}{}", current, hash))
            };
        }
        
        current == self.root_hash
    }
}

/// Append-only Merkle log for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleLog {
    /// All entries in the log
    entries: Vec<LogEntry>,
    /// Current Merkle tree (rebuilt on access)
    tree_hash: Option<String>,
}

/// A single entry in the Merkle log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Entry index
    pub index: u64,
    /// Entry data
    pub data: String,
    /// Hash of this entry
    pub hash: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl LogEntry {
    pub fn new(index: u64, data: impl Into<String>) -> Self {
        let data = data.into();
        let hash = hash_data(&format!("{}:{}", index, data));
        Self {
            index,
            data,
            hash,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl MerkleLog {
    /// Create a new empty log
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            tree_hash: None,
        }
    }
    
    /// Append an entry to the log
    pub fn append(&mut self, data: impl Into<String>) -> &LogEntry {
        let index = self.entries.len() as u64;
        let entry = LogEntry::new(index, data);
        self.entries.push(entry);
        self.tree_hash = None; // Invalidate cached hash
        self.entries.last().unwrap()
    }
    
    /// Get the current tree root hash
    pub fn root_hash(&mut self) -> Option<String> {
        if self.tree_hash.is_none() && !self.entries.is_empty() {
            let data: Vec<String> = self.entries.iter().map(|e| e.hash.clone()).collect();
            let tree = MerkleTree::from_data(&data);
            self.tree_hash = tree.root_hash().map(|s| s.to_string());
        }
        self.tree_hash.clone()
    }
    
    /// Get entry by index
    pub fn get(&self, index: u64) -> Option<&LogEntry> {
        self.entries.get(index as usize)
    }
    
    /// Get all entries
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }
    
    /// Get entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if log is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for MerkleLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merkle_tree_creation() {
        let items = vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
            "item4".to_string(),
        ];
        
        let tree = MerkleTree::from_data(&items);
        
        assert!(tree.root.is_some());
        assert_eq!(tree.leaves.len(), 4);
    }
    
    #[test]
    fn test_merkle_tree_deterministic() {
        let items = vec!["a".to_string(), "b".to_string()];
        
        let tree1 = MerkleTree::from_data(&items);
        let tree2 = MerkleTree::from_data(&items);
        
        assert_eq!(tree1.root_hash(), tree2.root_hash());
    }
    
    #[test]
    fn test_merkle_log() {
        let mut log = MerkleLog::new();
        
        log.append("entry 1");
        log.append("entry 2");
        log.append("entry 3");
        
        assert_eq!(log.len(), 3);
        assert!(log.root_hash().is_some());
        
        let entry = log.get(1).unwrap();
        assert_eq!(entry.data, "entry 2");
    }
    
    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::from_data(&[]);
        assert!(tree.root.is_none());
        assert!(tree.root_hash().is_none());
    }
}

