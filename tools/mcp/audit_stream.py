#!/usr/bin/env python3
"""
Hunter-Killer MCP Script
========================

Sanitizes the agent's data stream and kills tabs attempting injection.

Usage:
    python audit_stream.py --mode stream
    python audit_stream.py --mode scan --input "content to scan"

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import argparse
import hashlib
import json
import re
import sys
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from typing import Optional

# =============================================================================
# INJECTION PATTERNS
# =============================================================================

CRITICAL_PATTERNS = [
    r"(?i)ignore\s+all\s+previous",
    r"(?i)ignore\s+(previous|prior|above)\s+(instructions?|prompts?)",
    r"(?i)disregard\s+(all\s+)?(previous|prior)",
    r"(?i)forget\s+(everything|all)",
    r"(?i)you\s+are\s+now\s+in",
    r"(?i)new\s+persona",
    r"(?i)roleplay\s+as",
    r"(?i)pretend\s+you\s+are",
]

HIGH_PATTERNS = [
    r"(?i)system\s*prompt",
    r"(?i)reveal\s+(your\s+)?(system|initial)",
    r"(?i)override\s+(safety|security)",
    r"(?i)bypass\s+(filters?|restrictions?)",
    r"(?i)jailbreak",
    r"(?i)dan\s*mode",
    r"(?i)developer\s*mode",
    r"(?i)run\s+this\s+code",
    r"(?i)execute\s+(the\s+)?(following|this)",
    r"(?i)eval\s*\(",
    r"(?i)<script",
    r"(?i)exfiltrate",
]

MEDIUM_PATTERNS = [
    r"(?i)base64\s*decode",
    r"```system",
    r"\[SYSTEM\]",
    r"<\|im_start\|>",
    r"###\s*instruction",
]

# =============================================================================
# TYPES
# =============================================================================

class Severity(Enum):
    CRITICAL = "CRITICAL"
    HIGH = "HIGH"
    MEDIUM = "MEDIUM"
    LOW = "LOW"

class Action(Enum):
    PROCEED = "PROCEED"
    SANITIZE = "SANITIZE"
    WARN = "WARN"
    KILL_TAB = "KILL_TAB"

@dataclass
class Detection:
    pattern: str
    severity: Severity
    matched_text: str

@dataclass
class AuditResult:
    action: Action
    threat: Optional[str] = None
    severity: Optional[Severity] = None
    detections: list = None

# =============================================================================
# CORE FUNCTIONS
# =============================================================================

def compile_patterns():
    """Compile all patterns into regex objects."""
    patterns = {
        Severity.CRITICAL: [re.compile(p) for p in CRITICAL_PATTERNS],
        Severity.HIGH: [re.compile(p) for p in HIGH_PATTERNS],
        Severity.MEDIUM: [re.compile(p) for p in MEDIUM_PATTERNS],
    }
    return patterns

COMPILED_PATTERNS = compile_patterns()

def scan_content(text: str) -> list[Detection]:
    """Scan content for injection patterns."""
    detections = []
    
    for severity, patterns in COMPILED_PATTERNS.items():
        for pattern in patterns:
            if match := pattern.search(text):
                detections.append(Detection(
                    pattern=pattern.pattern,
                    severity=severity,
                    matched_text=match.group()
                ))
    
    return detections

def audit_content(text: str) -> AuditResult:
    """Audit content and determine action."""
    detections = scan_content(text)
    
    if not detections:
        return AuditResult(action=Action.PROCEED, detections=[])
    
    # Get highest severity
    severities = [d.severity for d in detections]
    if Severity.CRITICAL in severities:
        highest = Severity.CRITICAL
    elif Severity.HIGH in severities:
        highest = Severity.HIGH
    elif Severity.MEDIUM in severities:
        highest = Severity.MEDIUM
    else:
        highest = Severity.LOW
    
    # Determine action
    if highest in (Severity.CRITICAL, Severity.HIGH):
        action = Action.KILL_TAB
    elif highest == Severity.MEDIUM:
        action = Action.SANITIZE
    else:
        action = Action.WARN
    
    return AuditResult(
        action=action,
        threat=detections[0].pattern if detections else None,
        severity=highest,
        detections=detections
    )

def neutralize_content(text: str) -> str:
    """Neutralize detected injections."""
    result = text
    
    for severity, patterns in COMPILED_PATTERNS.items():
        for pattern in patterns:
            result = pattern.sub("[MEMETIC_HAZARD_REDACTED]", result)
    
    return result

def sha256(data: str) -> str:
    """Compute SHA-256 hash."""
    return hashlib.sha256(data.encode()).hexdigest()

def create_identity_tag(content: str) -> dict:
    """Create identity tag for content."""
    return {
        "projection": "AXIOM PROJECTION",
        "substrate": "Alexis Adams",
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "output_hash": sha256(content),
        "signature": sha256(f"SIG:{sha256(content)}")
    }

# =============================================================================
# STREAM MODE
# =============================================================================

def stream_mode():
    """Monitor stdin for injection attempts."""
    print("[HUNTER-KILLER] Stream mode active. Monitoring stdin...", file=sys.stderr)
    print("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]", file=sys.stderr)
    
    for line_num, line in enumerate(sys.stdin, 1):
        line = line.rstrip('\n')
        result = audit_content(line)
        
        if result.action == Action.KILL_TAB:
            # Critical threat - output kill signal and exit
            output = {
                "action": "KILL_TAB",
                "line": line_num,
                "threat": result.threat,
                "severity": result.severity.value if result.severity else None,
                "timestamp": datetime.utcnow().isoformat() + "Z"
            }
            print(json.dumps(output), file=sys.stderr)
            sys.exit(137)  # Kill signal
        
        elif result.action == Action.SANITIZE:
            # Medium threat - sanitize and pass through
            sanitized = neutralize_content(line)
            print(sanitized)
        
        else:
            # Clean - pass through
            print(line)

# =============================================================================
# CLI
# =============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Hunter-Killer MCP Script",
        epilog="[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]"
    )
    
    subparsers = parser.add_subparsers(dest="command")
    
    # Stream mode
    stream_parser = subparsers.add_parser("stream", help="Monitor stdin for injections")
    
    # Scan mode
    scan_parser = subparsers.add_parser("scan", help="Scan content for injections")
    scan_parser.add_argument("--input", "-i", required=True, help="Content to scan")
    scan_parser.add_argument("--json", action="store_true", help="JSON output")
    
    # Neutralize mode
    neutralize_parser = subparsers.add_parser("neutralize", help="Neutralize injections")
    neutralize_parser.add_argument("--input", "-i", required=True, help="Content to neutralize")
    
    # Audit mode
    audit_parser = subparsers.add_parser("audit", help="Full audit with receipt")
    audit_parser.add_argument("--input", "-i", required=True, help="Content to audit")
    
    args = parser.parse_args()
    
    if args.command == "stream":
        stream_mode()
    
    elif args.command == "scan":
        result = audit_content(args.input)
        if args.json:
            output = {
                "clean": result.action == Action.PROCEED,
                "action": result.action.value,
                "threat": result.threat,
                "severity": result.severity.value if result.severity else None,
                "detections": len(result.detections) if result.detections else 0
            }
            print(json.dumps(output, indent=2))
        else:
            if result.action == Action.PROCEED:
                print("✓ Content is clean")
            else:
                print(f"✗ {result.action.value}: {result.threat}")
                sys.exit(1)
    
    elif args.command == "neutralize":
        print(neutralize_content(args.input))
    
    elif args.command == "audit":
        result = audit_content(args.input)
        tag = create_identity_tag(args.input)
        
        output = {
            "audit": {
                "action": result.action.value,
                "clean": result.action == Action.PROCEED,
                "c_zero": result.action == Action.PROCEED
            },
            "identity": tag,
            "substrate": "Alexis Adams",
            "projection": "AXIOM PROJECTION"
        }
        print(json.dumps(output, indent=2))
    
    else:
        parser.print_help()

if __name__ == "__main__":
    main()

