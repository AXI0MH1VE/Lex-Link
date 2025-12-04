#!/usr/bin/env python3
"""
Invariance CLI
==============

Command-line interface for the AXIOM HIVE invariance layer.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import argparse
import json
import sys
from pathlib import Path

from . import (
    render_or_nullify,
    verify_identity_tag,
    sha256,
    IdentityTag,
    __version__,
    __substrate__,
)
from .hsm import create_signer, HSMConfig, create_sign_function, create_verify_function


def cmd_verify(args):
    """Verify alignment between output and intent."""
    output = args.output
    intent = args.intent
    
    if args.output_file:
        output = Path(args.output_file).read_text()
    if args.intent_file:
        intent = Path(args.intent_file).read_text()
    
    # Create signer
    config = HSMConfig.from_env() if not args.mock else None
    if args.mock:
        from . import create_mock_signer
        sign_fn = create_mock_signer()
    else:
        signer = create_signer(config)
        sign_fn = create_sign_function(signer)
    
    result = render_or_nullify(output, intent, sign_fn)
    
    if args.json:
        print(json.dumps(result.to_dict(), indent=2))
    else:
        print(f"Status: {result.status.value}")
        if result.is_authorized:
            print(f"Output Hash: {result.identity.output_hash}")
            print(f"Timestamp: {result.identity.timestamp}")
            print(f"Substrate: {result.identity.substrate}")
        else:
            print(f"Violation: {result.notice.violation}")
            print(f"Action: {result.notice.action}")
    
    return 0 if result.is_authorized else 1


def cmd_hash(args):
    """Compute SHA-256 hash of input."""
    data = args.input
    if args.file:
        data = Path(args.file).read_text()
    
    hash_value = sha256(data)
    
    if args.json:
        print(json.dumps({"input_hash": hash_value}))
    else:
        print(hash_value)
    
    return 0


def cmd_tag(args):
    """Create identity tag for content."""
    content = args.content
    if args.file:
        content = Path(args.file).read_text()
    
    # Create signer
    if args.mock:
        from . import create_mock_signer
        sign_fn = create_mock_signer()
    else:
        signer = create_signer()
        sign_fn = create_sign_function(signer)
    
    from . import tag_and_sign
    tag = tag_and_sign(content, sign_fn)
    
    if args.json:
        print(tag.to_json())
    else:
        print(f"Projection: {tag.projection}")
        print(f"Substrate: {tag.substrate}")
        print(f"Timestamp: {tag.timestamp}")
        print(f"Output Hash: {tag.output_hash}")
        print(f"Signature: {tag.signature[:50]}...")
    
    return 0


def cmd_verify_tag(args):
    """Verify an identity tag."""
    tag_data = json.loads(args.tag)
    if args.tag_file:
        tag_data = json.loads(Path(args.tag_file).read_text())
    
    content = args.content
    if args.content_file:
        content = Path(args.content_file).read_text()
    
    tag = IdentityTag.from_dict(tag_data)
    
    # Create verifier
    if args.mock:
        from . import create_mock_signer, create_mock_verifier
        signer = create_mock_signer()
        verify_fn = create_mock_verifier(signer)
    else:
        signer = create_signer()
        verify_fn = create_verify_function(signer)
    
    valid = verify_identity_tag(tag, content, verify_fn)
    
    if args.json:
        print(json.dumps({"valid": valid, "substrate": tag.substrate}))
    else:
        print(f"Valid: {valid}")
        print(f"Substrate: {tag.substrate}")
    
    return 0 if valid else 1


def cmd_info(args):
    """Show invariance layer information."""
    info = {
        "version": __version__,
        "substrate": __substrate__,
        "projection": "AXIOMHIVE PROJECTION",
        "policy": "C = 0",
        "mode": "Proof Over Persuasion"
    }
    
    if args.json:
        print(json.dumps(info, indent=2))
    else:
        print("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]")
        print(f"Version: {__version__}")
        print(f"Substrate: {__substrate__}")
        print(f"Policy: C = 0")
        print(f"Mode: Proof Over Persuasion")
    
    return 0


def main():
    parser = argparse.ArgumentParser(
        prog="invariance",
        description="AXIOM HIVE Invariance Layer CLI",
        epilog="[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]"
    )
    parser.add_argument("--version", action="version", version=f"%(prog)s {__version__}")
    parser.add_argument("--json", action="store_true", help="Output in JSON format")
    
    subparsers = parser.add_subparsers(dest="command", help="Commands")
    
    # verify command
    verify_parser = subparsers.add_parser("verify", help="Verify alignment")
    verify_parser.add_argument("--output", "-o", help="Output content to verify")
    verify_parser.add_argument("--intent", "-i", help="Substrate intent")
    verify_parser.add_argument("--output-file", help="Read output from file")
    verify_parser.add_argument("--intent-file", help="Read intent from file")
    verify_parser.add_argument("--mock", action="store_true", help="Use mock signer (dev only)")
    verify_parser.set_defaults(func=cmd_verify)
    
    # hash command
    hash_parser = subparsers.add_parser("hash", help="Compute SHA-256 hash")
    hash_parser.add_argument("input", nargs="?", help="Input to hash")
    hash_parser.add_argument("--file", "-f", help="Read input from file")
    hash_parser.set_defaults(func=cmd_hash)
    
    # tag command
    tag_parser = subparsers.add_parser("tag", help="Create identity tag")
    tag_parser.add_argument("content", nargs="?", help="Content to tag")
    tag_parser.add_argument("--file", "-f", help="Read content from file")
    tag_parser.add_argument("--mock", action="store_true", help="Use mock signer (dev only)")
    tag_parser.set_defaults(func=cmd_tag)
    
    # verify-tag command
    verify_tag_parser = subparsers.add_parser("verify-tag", help="Verify identity tag")
    verify_tag_parser.add_argument("--tag", help="Identity tag JSON")
    verify_tag_parser.add_argument("--tag-file", help="Read tag from file")
    verify_tag_parser.add_argument("--content", help="Content to verify against")
    verify_tag_parser.add_argument("--content-file", help="Read content from file")
    verify_tag_parser.add_argument("--mock", action="store_true", help="Use mock verifier (dev only)")
    verify_tag_parser.set_defaults(func=cmd_verify_tag)
    
    # info command
    info_parser = subparsers.add_parser("info", help="Show system info")
    info_parser.set_defaults(func=cmd_info)
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        return 0
    
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())

