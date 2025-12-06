"""
Merkle Tree implementation for cryptographic data integrity verification.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import hashlib
from typing import List, Optional


class MerkleTree:
    """
    Implements a deterministic Merkle Tree using SHA-256 for cryptographic verification.
    """

    def __init__(self, data_blocks: List[str]):
        """Initializes the Merkle Tree with a list of data blocks."""
        if not data_blocks:
            self._root_hash = None
            return

        # 1. Hash the leaves
        self.leaves = [self._hash_data(block) for block in data_blocks]
        
        # 2. Build the tree layers
        self.tree_layers = [self.leaves]
        
        current_layer = self.leaves
        while len(current_layer) > 1:
            next_layer = self._build_next_layer(current_layer)
            self.tree_layers.append(next_layer)
            current_layer = next_layer
        
        self._root_hash = current_layer[0] if current_layer else None

    @staticmethod
    def _hash_data(data: str) -> str:
        """Computes the SHA-256 hash of a data string."""
        return hashlib.sha256(data.encode('utf-8')).hexdigest()

    def _build_next_layer(self, hashes: List[str]) -> List[str]:
        """
        Builds the next layer up by pairing, concatenating, and hashing the current layer.
        Handles odd number of nodes by duplicating the last node (Bitcoin-style).
        """
        next_layer = []
        
        # Handle odd count by duplicating the last hash
        if len(hashes) % 2 != 0:
            hashes.append(hashes[-1])

        # Pair hashes and compute parents
        for i in range(0, len(hashes), 2):
            hash1 = hashes[i]
            hash2 = hashes[i + 1]
            
            # Concatenate (fixed order: hash1 + hash2) and hash the result
            combined_hash = hash1 + hash2
            parent_hash = MerkleTree._hash_data(combined_hash)
            next_layer.append(parent_hash)
            
        return next_layer

    def get_root_hash(self) -> Optional[str]:
        """Returns the Merkle Root hash."""
        return self._root_hash

