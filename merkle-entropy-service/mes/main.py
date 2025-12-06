"""
Merkle Entropy Service API - Flask application.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
Policy: C = 0 | Human-in-the-Loop | Operator Control
"""

from flask import Flask, jsonify, request, abort
from mes.merkle import MerkleTree
from mes.entropy import calculate_entropy
from mes.safety import InputValidator, get_audit_logger, SafetyConfig
from mes.operator_control import (
    OperationType, get_operator_approval, ApprovalStatus
)
from typing import List, Optional
import json
import hashlib
import uuid

# Flask application initialization
app = Flask(__name__)

# Initialize safety components
audit_logger = get_audit_logger()
operator_approval = get_operator_approval()


@app.route('/health', methods=['GET'])
def health_check():
    """
    Operational readiness check for orchestrators.
    
    This endpoint is read-only and does not require operator approval.
    """
    return jsonify({
        'status': 'healthy',
        'service': 'merkle-entropy-service',
        'version': '1.0.0',
        'policy': 'C = 0',
        'human_in_loop': True,
        'operator_control': True
    }), 200


def _validate_data_input(data: dict) -> List[str]:
    """
    Helper to ensure input is a valid list of strings.
    
    Includes safety validation for size limits and DoS protection.
    """
    if not isinstance(data, dict) or 'data_blocks' not in data:
        abort(400, description="Missing 'data_blocks' key in JSON payload.")
    
    data_blocks = data['data_blocks']
    
    if not isinstance(data_blocks, list):
        abort(400, description="'data_blocks' must be a list.")
        
    for item in data_blocks:
        if not isinstance(item, str):
            abort(400, description="All items in 'data_blocks' must be strings.")
    
    # Safety validation
    is_valid, error_msg = InputValidator.validate_data_blocks(data_blocks)
    if not is_valid:
        abort(400, description=f"Input validation failed: {error_msg}")
            
    return data_blocks


def _compute_input_hash(data_blocks: List[str]) -> str:
    """Compute cryptographic hash of input for audit trail."""
    input_json = json.dumps(data_blocks, sort_keys=True)
    return hashlib.sha256(input_json.encode('utf-8')).hexdigest()


@app.route('/merkle_root', methods=['POST'])
def get_merkle_root():
    """
    Calculates and returns the Merkle Root for the provided data blocks.
    
    This is a READ-ONLY operation (no state changes) and does not require
    operator approval. All operations are logged for audit.
    """
    try:
        data = request.get_json()
    except json.JSONDecodeError:
        abort(400, description="Invalid JSON format.")

    data_blocks = _validate_data_input(data)
    
    # Compute input hash for audit
    input_hash = _compute_input_hash(data_blocks)
    operation_id = str(uuid.uuid4())

    # Perform computation (read-only, deterministic)
    tree = MerkleTree(data_blocks)
    root_hash = tree.get_root_hash()

    if root_hash is None:
        result = {'merkle_root': None, 'message': 'No data provided'}
        output_hash = hashlib.sha256(json.dumps(result, sort_keys=True).encode('utf-8')).hexdigest()
        
        # Log operation
        audit_logger.log_operation(
            operation='merkle_root',
            input_hash=input_hash,
            output_hash=output_hash,
            operator_id=request.headers.get('X-Operator-ID'),
            approved=True,  # Read-only operations auto-approved
            metadata={'operation_id': operation_id, 'empty_input': True}
        )
        
        return jsonify(result), 200

    result = {'merkle_root': root_hash, 'operation_id': operation_id}
    output_hash = hashlib.sha256(json.dumps(result, sort_keys=True).encode('utf-8')).hexdigest()
    
    # Log operation for audit trail
    audit_logger.log_operation(
        operation='merkle_root',
        input_hash=input_hash,
        output_hash=output_hash,
        operator_id=request.headers.get('X-Operator-ID'),
        approved=True,  # Read-only operations auto-approved
        metadata={'operation_id': operation_id, 'root_hash': root_hash}
    )

    return jsonify(result), 200


@app.route('/merkle_entropy', methods=['POST'])
def get_merkle_entropy():
    """
    Calculates and returns the Shannon Entropy of the provided data blocks.
    
    This is a READ-ONLY operation (no state changes) and does not require
    operator approval. All operations are logged for audit.
    """
    try:
        data = request.get_json()
    except json.JSONDecodeError:
        abort(400, description="Invalid JSON format.")

    data_blocks = _validate_data_input(data)
    
    # Compute input hash for audit
    input_hash = _compute_input_hash(data_blocks)
    operation_id = str(uuid.uuid4())

    # Perform computation (read-only, deterministic)
    entropy_value = calculate_entropy(data_blocks)
    
    result = {
        'shannon_entropy': entropy_value,
        'operation_id': operation_id
    }
    output_hash = hashlib.sha256(json.dumps(result, sort_keys=True).encode('utf-8')).hexdigest()
    
    # Log operation for audit trail
    audit_logger.log_operation(
        operation='merkle_entropy',
        input_hash=input_hash,
        output_hash=output_hash,
        operator_id=request.headers.get('X-Operator-ID'),
        approved=True,  # Read-only operations auto-approved
        metadata={'operation_id': operation_id, 'entropy_value': entropy_value}
    )
    
    return jsonify(result), 200


@app.route('/audit/trail', methods=['GET'])
def get_audit_trail():
    """
    Get complete audit trail.
    
    Requires operator authentication (X-Operator-ID header).
    This is a READ-ONLY operation.
    """
    operator_id = request.headers.get('X-Operator-ID')
    if not operator_id:
        abort(401, description="Operator ID required (X-Operator-ID header)")
    
    trail = audit_logger.get_audit_trail()
    integrity_ok = audit_logger.verify_integrity()
    
    return jsonify({
        'audit_trail': trail,
        'integrity_verified': integrity_ok,
        'entry_count': len(trail)
    }), 200


@app.route('/operator/approval/<operation_id>', methods=['POST'])
def approve_operation(operation_id: str):
    """
    Operator approval endpoint for operations requiring approval.
    
    This endpoint allows operators to approve or reject operations.
    """
    operator_id = request.headers.get('X-Operator-ID')
    if not operator_id:
        abort(401, description="Operator ID required (X-Operator-ID header)")
    
    try:
        data = request.get_json() or {}
        action = data.get('action', 'approve')  # 'approve' or 'reject'
        signature = data.get('signature')
        reason = data.get('reason')
    except json.JSONDecodeError:
        abort(400, description="Invalid JSON format.")
    
    if action == 'approve':
        success = operator_approval.approve(operation_id, operator_id, signature)
        if not success:
            abort(400, description="Approval failed - operation not found or expired")
        return jsonify({'status': 'approved', 'operation_id': operation_id}), 200
    elif action == 'reject':
        success = operator_approval.reject(operation_id, operator_id, reason)
        if not success:
            abort(400, description="Rejection failed - operation not found")
        return jsonify({'status': 'rejected', 'operation_id': operation_id}), 200
    else:
        abort(400, description="Invalid action - must be 'approve' or 'reject'")


@app.route('/operator/approval/<operation_id>', methods=['GET'])
def check_approval_status(operation_id: str):
    """Check approval status for an operation."""
    operator_id = request.headers.get('X-Operator-ID')
    if not operator_id:
        abort(401, description="Operator ID required (X-Operator-ID header)")
    
    approval = operator_approval.check_approval(operation_id)
    if not approval:
        abort(404, description="Operation not found")
    
    return jsonify(approval), 200


@app.route('/safety/config', methods=['GET'])
def get_safety_config():
    """
    Get current safety configuration.
    
    This is a READ-ONLY operation showing operator-visible safety settings.
    """
    return jsonify({
        'max_data_blocks': SafetyConfig.MAX_DATA_BLOCKS,
        'max_block_size': SafetyConfig.MAX_BLOCK_SIZE,
        'max_total_size': SafetyConfig.MAX_TOTAL_SIZE,
        'max_requests_per_minute': SafetyConfig.MAX_REQUESTS_PER_MINUTE,
        'requires_approval': SafetyConfig.REQUIRES_APPROVAL,
        'approval_timeout_seconds': SafetyConfig.APPROVAL_TIMEOUT_SECONDS,
        'human_in_loop': True,
        'operator_control': True,
        'policy': 'C = 0'
    }), 200


if __name__ == '__main__':
    # This block is for local development only. Production relies on Gunicorn.
    # WARNING: debug=True should NEVER be used in production
    app.run(host='0.0.0.0', port=8000, debug=False)  # Changed to False for safety

