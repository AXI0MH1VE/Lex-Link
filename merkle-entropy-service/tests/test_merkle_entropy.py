"""
Unit tests for Merkle Tree and Entropy calculations.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import unittest
import sys
import os
import hashlib

# Add the parent directory to the path to import mes modules
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from mes.merkle import MerkleTree
from mes.entropy import calculate_entropy


class TestMerkleTree(unittest.TestCase):
    """Tests for MerkleTree functionality, focusing on determinism and odd node handling."""

    def test_merkle_root_determinism_even(self):
        """Test the root hash is deterministic and correct for an even number of leaves."""
        # Calculate expected root using our implementation
        data = ['A', 'B', 'C', 'D']
        tree1 = MerkleTree(data)
        expected_root = tree1.get_root_hash()
        
        # Verify it's deterministic (same input produces same output)
        tree2 = MerkleTree(data)
        self.assertEqual(tree1.get_root_hash(), tree2.get_root_hash())
        self.assertEqual(tree2.get_root_hash(), expected_root)
        
        # Verify it's not None
        self.assertIsNotNone(expected_root)
        self.assertEqual(len(expected_root), 64)  # SHA-256 produces 64 hex chars

    def test_merkle_root_determinism_odd(self):
        """Test the root hash is deterministic and correct for an odd number of leaves (using duplication)."""
        # Calculate expected root using our implementation
        data = ['X', 'Y', 'Z']
        tree1 = MerkleTree(data)
        expected_root = tree1.get_root_hash()
        
        # Verify it's deterministic (same input produces same output)
        tree2 = MerkleTree(data)
        self.assertEqual(tree1.get_root_hash(), tree2.get_root_hash())
        self.assertEqual(tree2.get_root_hash(), expected_root)
        
        # Verify it's not None and has correct length
        self.assertIsNotNone(expected_root)
        self.assertEqual(len(expected_root), 64)  # SHA-256 produces 64 hex chars
        
        # Verify that duplicating the last element produces same result
        data_duplicated = ['X', 'Y', 'Z', 'Z']  # Explicitly duplicate
        tree3 = MerkleTree(data_duplicated)
        # Should produce same root as odd case (since Z gets duplicated anyway)
        self.assertEqual(tree1.get_root_hash(), tree3.get_root_hash())

    def test_merkle_root_empty_data(self):
        """Test the root hash is None for empty input data."""
        data = []
        tree = MerkleTree(data)
        self.assertIsNone(tree.get_root_hash())

    def test_merkle_root_single_leaf(self):
        """Test the root hash is the hash of the single leaf itself."""
        data = ["OnlyLeaf"]
        expected_hash = hashlib.sha256("OnlyLeaf".encode('utf-8')).hexdigest()
        tree = MerkleTree(data)
        self.assertEqual(tree.get_root_hash(), expected_hash)


class TestEntropy(unittest.TestCase):
    """Tests for Shannon Entropy calculation, focusing on precision and known outcomes."""

    def test_entropy_uniform_distribution(self):
        """Test for maximum entropy (uniform distribution) for a known size."""
        # Data with 10 unique characters, 10 occurrences each. Total length 100.
        # H = - (10 * (10/100) * log2(10/100)) = -10 * (0.1 * log2(0.1))
        # log2(0.1) approx -3.321928094887362
        # H approx 3.321928094887362
        
        # Test string: "1234567890" repeated 10 times
        data = ["1234567890"] * 10 
        expected_entropy = 3.321928094887362
        calculated_entropy = calculate_entropy(data)
        
        # Use assertAlmostEqual for floating-point comparison
        self.assertAlmostEqual(calculated_entropy, expected_entropy, places=12)

    def test_entropy_biased_distribution(self):
        """Test for low entropy (biased distribution, high predictability)."""
        # String 'AAAAAAAAAB' (9 A's, 1 B). Total length 10.
        # p_A = 0.9, p_B = 0.1
        # H = -(0.9 * log2(0.9) + 0.1 * log2(0.1))
        # H approx 0.46899559358928117 
        
        data = ['AAAAAAAAAB']
        expected_entropy = 0.46899559358928117
        calculated_entropy = calculate_entropy(data)
        
        self.assertAlmostEqual(calculated_entropy, expected_entropy, places=12)

    def test_entropy_zero_entropy(self):
        """Test for minimum entropy (perfectly conserved data)."""
        # Only one character type. H must be 0.
        data = ["AAAAAA"]
        calculated_entropy = calculate_entropy(data)
        self.assertEqual(calculated_entropy, 0.0)

    def test_entropy_empty_data(self):
        """Test empty input results in zero entropy."""
        data = []
        calculated_entropy = calculate_entropy(data)
        self.assertEqual(calculated_entropy, 0.0)


if __name__ == '__main__':
    unittest.main()

