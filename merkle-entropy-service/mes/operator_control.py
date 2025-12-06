"""
Operator control and approval mechanisms.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Policy: Human-in-the-Loop | Operator Control
"""

from typing import Optional, Dict
from datetime import datetime, timedelta
from enum import Enum
import hashlib
import json


class OperationType(Enum):
    """Types of operations requiring operator control."""
    READ_ONLY = "read_only"  # No approval needed
    COMPUTE_ONLY = "compute_only"  # No approval needed (deterministic computation)
    STATE_CHANGE = "state_change"  # Requires approval
    CRITICAL = "critical"  # Requires explicit operator approval


class ApprovalStatus(Enum):
    """Approval status for operations."""
    PENDING = "pending"
    APPROVED = "approved"
    REJECTED = "rejected"
    EXPIRED = "expired"


class OperatorApproval:
    """Operator approval tracking."""
    
    def __init__(self):
        self.pending_approvals: Dict[str, Dict] = {}
    
    def request_approval(
        self,
        operation_id: str,
        operation_type: OperationType,
        description: str,
        operator_id: Optional[str] = None,
        timeout_seconds: int = 300
    ) -> Dict:
        """
        Request operator approval for an operation.
        
        Returns:
            Approval request with status
        """
        expires_at = datetime.utcnow() + timedelta(seconds=timeout_seconds)
        
        approval_request = {
            'operation_id': operation_id,
            'operation_type': operation_type.value,
            'description': description,
            'operator_id': operator_id,
            'status': ApprovalStatus.PENDING.value,
            'requested_at': datetime.utcnow().isoformat(),
            'expires_at': expires_at.isoformat(),
            'approved_at': None,
            'rejected_at': None,
        }
        
        self.pending_approvals[operation_id] = approval_request
        return approval_request
    
    def approve(
        self,
        operation_id: str,
        operator_id: str,
        signature: Optional[str] = None
    ) -> bool:
        """
        Approve an operation.
        
        Returns:
            True if approval successful
        """
        if operation_id not in self.pending_approvals:
            return False
        
        approval = self.pending_approvals[operation_id]
        
        # Check expiration
        expires_at = datetime.fromisoformat(approval['expires_at'])
        if datetime.utcnow() > expires_at:
            approval['status'] = ApprovalStatus.EXPIRED.value
            return False
        
        # Approve
        approval['status'] = ApprovalStatus.APPROVED.value
        approval['operator_id'] = operator_id
        approval['approved_at'] = datetime.utcnow().isoformat()
        approval['signature'] = signature
        
        return True
    
    def reject(
        self,
        operation_id: str,
        operator_id: str,
        reason: Optional[str] = None
    ) -> bool:
        """
        Reject an operation.
        
        Returns:
            True if rejection successful
        """
        if operation_id not in self.pending_approvals:
            return False
        
        approval = self.pending_approvals[operation_id]
        approval['status'] = ApprovalStatus.REJECTED.value
        approval['operator_id'] = operator_id
        approval['rejected_at'] = datetime.utcnow().isoformat()
        approval['rejection_reason'] = reason
        
        return True
    
    def check_approval(self, operation_id: str) -> Optional[Dict]:
        """Check approval status for an operation."""
        if operation_id not in self.pending_approvals:
            return None
        
        approval = self.pending_approvals[operation_id]
        
        # Check expiration
        expires_at = datetime.fromisoformat(approval['expires_at'])
        if datetime.utcnow() > expires_at and approval['status'] == ApprovalStatus.PENDING.value:
            approval['status'] = ApprovalStatus.EXPIRED.value
        
        return approval
    
    def requires_approval(self, operation_type: OperationType) -> bool:
        """Check if operation type requires approval."""
        return operation_type in [OperationType.STATE_CHANGE, OperationType.CRITICAL]


# Global operator approval instance
_operator_approval = OperatorApproval()


def get_operator_approval() -> OperatorApproval:
    """Get the global operator approval instance."""
    return _operator_approval

