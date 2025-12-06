"""
Safety and operator control mechanisms for Merkle Entropy Service.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Policy: C = 0 | Human-in-the-Loop | Operator Control
"""

from typing import List, Optional, Dict, Tuple
from datetime import datetime
import hashlib
import json


class SafetyConfig:
    """Safety configuration parameters."""
    
    # Input size limits (prevent DoS)
    MAX_DATA_BLOCKS = 10000
    MAX_BLOCK_SIZE = 10 * 1024 * 1024  # 10MB per block
    MAX_TOTAL_SIZE = 100 * 1024 * 1024  # 100MB total
    
    # Rate limiting
    MAX_REQUESTS_PER_MINUTE = 100
    
    # Operator approval required for operations
    REQUIRES_APPROVAL = False  # Read-only operations don't require approval
    APPROVAL_TIMEOUT_SECONDS = 300  # 5 minutes


class InputValidator:
    """Validates and sanitizes input data."""
    
    @staticmethod
    def validate_data_blocks(data_blocks: List[str]) -> Tuple[bool, Optional[str]]:
        """
        Validate data blocks for safety.
        
        Returns:
            (is_valid, error_message)
        """
        # Check count
        if len(data_blocks) > SafetyConfig.MAX_DATA_BLOCKS:
            return False, f"Too many data blocks: {len(data_blocks)} > {SafetyConfig.MAX_DATA_BLOCKS}"
        
        # Check individual block sizes
        total_size = 0
        for i, block in enumerate(data_blocks):
            block_size = len(block.encode('utf-8'))
            if block_size > SafetyConfig.MAX_BLOCK_SIZE:
                return False, f"Block {i} too large: {block_size} > {SafetyConfig.MAX_BLOCK_SIZE}"
            total_size += block_size
        
        # Check total size
        if total_size > SafetyConfig.MAX_TOTAL_SIZE:
            return False, f"Total size too large: {total_size} > {SafetyConfig.MAX_TOTAL_SIZE}"
        
        return True, None


class AuditLogger:
    """Immutable audit logging for all operations."""
    
    def __init__(self):
        self.logs: List[Dict] = []
    
    def log_operation(
        self,
        operation: str,
        input_hash: str,
        output_hash: str,
        operator_id: Optional[str] = None,
        approved: bool = True,
        metadata: Optional[Dict] = None
    ) -> str:
        """
        Log an operation with cryptographic integrity.
        
        Returns:
            Log entry hash
        """
        entry = {
            'timestamp': datetime.utcnow().isoformat(),
            'operation': operation,
            'input_hash': input_hash,
            'output_hash': output_hash,
            'operator_id': operator_id,
            'approved': approved,
            'metadata': metadata or {},
        }
        
        # Create hash of log entry for integrity
        entry_json = json.dumps(entry, sort_keys=True)
        entry_hash = hashlib.sha256(entry_json.encode('utf-8')).hexdigest()
        entry['entry_hash'] = entry_hash
        
        self.logs.append(entry)
        return entry_hash
    
    def get_audit_trail(self) -> List[Dict]:
        """Get complete audit trail."""
        return self.logs.copy()
    
    def verify_integrity(self) -> bool:
        """Verify audit trail integrity."""
        for entry in self.logs:
            # Recompute hash
            entry_copy = entry.copy()
            entry_hash = entry_copy.pop('entry_hash')
            entry_json = json.dumps(entry_copy, sort_keys=True)
            computed_hash = hashlib.sha256(entry_json.encode('utf-8')).hexdigest()
            
            if entry_hash != computed_hash:
                return False
        return True


# Global audit logger instance
_audit_logger = AuditLogger()


def get_audit_logger() -> AuditLogger:
    """Get the global audit logger instance."""
    return _audit_logger

