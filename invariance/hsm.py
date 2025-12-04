"""
HSM (Hardware Security Module) Integration
==========================================

Production signing and verification using YubiHSM 2 or Nitrokey HSM.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import base64
import hashlib
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Optional, Tuple
from enum import Enum


class HSMType(Enum):
    """Supported HSM types."""
    YUBIHSM = "yubihsm"
    NITROKEY = "nitrokey"
    SOFTHSM = "softhsm"  # Development only


class SignatureAlgorithm(Enum):
    """Supported signature algorithms."""
    ECDSA_P256 = "ecdsa-p256"
    ECDSA_P384 = "ecdsa-p384"
    ED25519 = "ed25519"


@dataclass
class HSMConfig:
    """HSM configuration."""
    hsm_type: HSMType
    connector_url: str
    auth_key_id: int
    signing_key_id: int
    algorithm: SignatureAlgorithm = SignatureAlgorithm.ECDSA_P384
    password: Optional[str] = None
    
    @classmethod
    def from_env(cls) -> "HSMConfig":
        """Load configuration from environment variables."""
        import os
        
        hsm_type = HSMType(os.environ.get("AXIOM_HSM_TYPE", "softhsm"))
        
        return cls(
            hsm_type=hsm_type,
            connector_url=os.environ.get("AXIOM_HSM_URL", "http://localhost:12345"),
            auth_key_id=int(os.environ.get("AXIOM_HSM_AUTH_KEY", "1")),
            signing_key_id=int(os.environ.get("AXIOM_HSM_SIGN_KEY", "2")),
            algorithm=SignatureAlgorithm(
                os.environ.get("AXIOM_HSM_ALGORITHM", "ecdsa-p384")
            ),
            password=os.environ.get("AXIOM_HSM_PASSWORD"),
        )


class HSMSigner(ABC):
    """Abstract base class for HSM signers."""
    
    @abstractmethod
    def sign(self, data: bytes) -> bytes:
        """Sign data and return DER-encoded signature."""
        pass
    
    @abstractmethod
    def verify(self, data: bytes, signature: bytes) -> bool:
        """Verify signature against data."""
        pass
    
    @abstractmethod
    def get_public_key(self) -> bytes:
        """Get public key in DER format."""
        pass
    
    def sign_hash(self, hash_hex: str) -> str:
        """Sign a hex-encoded hash, return base64-encoded signature."""
        hash_bytes = bytes.fromhex(hash_hex)
        sig_bytes = self.sign(hash_bytes)
        return base64.b64encode(sig_bytes).decode("ascii")
    
    def verify_hash(self, hash_hex: str, signature_b64: str) -> bool:
        """Verify base64-encoded signature against hex-encoded hash."""
        hash_bytes = bytes.fromhex(hash_hex)
        sig_bytes = base64.b64decode(signature_b64)
        return self.verify(hash_bytes, sig_bytes)


class YubiHSMSigner(HSMSigner):
    """
    YubiHSM 2 signer implementation.
    
    Requires yubihsm library: pip install yubihsm[http]
    """
    
    def __init__(self, config: HSMConfig):
        if config.hsm_type != HSMType.YUBIHSM:
            raise ValueError("Config must be for YubiHSM")
        
        self.config = config
        self._session = None
        self._key = None
    
    def _get_session(self):
        """Get or create HSM session."""
        if self._session is not None:
            return self._session
        
        try:
            from yubihsm import YubiHsm
            from yubihsm.objects import AsymmetricKey
        except ImportError:
            raise ImportError(
                "yubihsm library required. Install with: pip install yubihsm[http]"
            )
        
        hsm = YubiHsm.connect(self.config.connector_url)
        self._session = hsm.create_session_derived(
            self.config.auth_key_id,
            self.config.password or ""
        )
        self._key = AsymmetricKey(self._session, self.config.signing_key_id)
        
        return self._session
    
    def sign(self, data: bytes) -> bytes:
        """Sign data using YubiHSM."""
        self._get_session()
        
        # Hash the data (HSM signs the hash)
        if self.config.algorithm == SignatureAlgorithm.ECDSA_P384:
            digest = hashlib.sha384(data).digest()
        else:
            digest = hashlib.sha256(data).digest()
        
        return self._key.sign_ecdsa(digest)
    
    def verify(self, data: bytes, signature: bytes) -> bool:
        """Verify signature using public key."""
        try:
            from cryptography.hazmat.primitives import hashes
            from cryptography.hazmat.primitives.asymmetric import ec
            from cryptography.exceptions import InvalidSignature
        except ImportError:
            raise ImportError("cryptography library required")
        
        public_key = self._get_public_key_obj()
        
        try:
            if self.config.algorithm == SignatureAlgorithm.ECDSA_P384:
                public_key.verify(signature, data, ec.ECDSA(hashes.SHA384()))
            else:
                public_key.verify(signature, data, ec.ECDSA(hashes.SHA256()))
            return True
        except InvalidSignature:
            return False
    
    def _get_public_key_obj(self):
        """Get cryptography public key object."""
        self._get_session()
        return self._key.get_public_key()
    
    def get_public_key(self) -> bytes:
        """Get public key in DER format."""
        from cryptography.hazmat.primitives.serialization import (
            Encoding, PublicFormat
        )
        return self._get_public_key_obj().public_bytes(
            Encoding.DER, PublicFormat.SubjectPublicKeyInfo
        )
    
    def close(self):
        """Close HSM session."""
        if self._session:
            self._session.close()
            self._session = None
            self._key = None


class SoftHSMSigner(HSMSigner):
    """
    Software HSM signer for development/testing.
    
    WARNING: Not for production use!
    """
    
    def __init__(self, config: HSMConfig):
        import warnings
        warnings.warn(
            "SoftHSMSigner is not production-safe. Use YubiHSM in production.",
            RuntimeWarning
        )
        
        self.config = config
        self._private_key = None
        self._public_key = None
        self._init_keys()
    
    def _init_keys(self):
        """Initialize software keys."""
        try:
            from cryptography.hazmat.primitives.asymmetric import ec
            from cryptography.hazmat.backends import default_backend
        except ImportError:
            raise ImportError("cryptography library required")
        
        if self.config.algorithm == SignatureAlgorithm.ECDSA_P384:
            curve = ec.SECP384R1()
        else:
            curve = ec.SECP256R1()
        
        self._private_key = ec.generate_private_key(curve, default_backend())
        self._public_key = self._private_key.public_key()
    
    def sign(self, data: bytes) -> bytes:
        """Sign data using software key."""
        from cryptography.hazmat.primitives import hashes
        from cryptography.hazmat.primitives.asymmetric import ec
        
        if self.config.algorithm == SignatureAlgorithm.ECDSA_P384:
            return self._private_key.sign(data, ec.ECDSA(hashes.SHA384()))
        return self._private_key.sign(data, ec.ECDSA(hashes.SHA256()))
    
    def verify(self, data: bytes, signature: bytes) -> bool:
        """Verify signature."""
        from cryptography.hazmat.primitives import hashes
        from cryptography.hazmat.primitives.asymmetric import ec
        from cryptography.exceptions import InvalidSignature
        
        try:
            if self.config.algorithm == SignatureAlgorithm.ECDSA_P384:
                self._public_key.verify(signature, data, ec.ECDSA(hashes.SHA384()))
            else:
                self._public_key.verify(signature, data, ec.ECDSA(hashes.SHA256()))
            return True
        except InvalidSignature:
            return False
    
    def get_public_key(self) -> bytes:
        """Get public key in DER format."""
        from cryptography.hazmat.primitives.serialization import (
            Encoding, PublicFormat
        )
        return self._public_key.public_bytes(
            Encoding.DER, PublicFormat.SubjectPublicKeyInfo
        )


def create_signer(config: Optional[HSMConfig] = None) -> HSMSigner:
    """
    Factory function to create appropriate HSM signer.
    
    Args:
        config: HSM configuration. If None, loads from environment.
        
    Returns:
        Configured HSM signer instance.
    """
    if config is None:
        config = HSMConfig.from_env()
    
    if config.hsm_type == HSMType.YUBIHSM:
        return YubiHSMSigner(config)
    elif config.hsm_type == HSMType.SOFTHSM:
        return SoftHSMSigner(config)
    else:
        raise ValueError(f"Unsupported HSM type: {config.hsm_type}")


def create_sign_function(signer: HSMSigner):
    """
    Create a signing function compatible with invariance layer.
    
    Args:
        signer: HSM signer instance
        
    Returns:
        Function that takes hash string and returns base64 signature
    """
    return signer.sign_hash


def create_verify_function(signer: HSMSigner):
    """
    Create a verification function compatible with invariance layer.
    
    Args:
        signer: HSM signer instance
        
    Returns:
        Function that takes (hash, signature) and returns bool
    """
    return signer.verify_hash


__all__ = [
    "HSMType",
    "SignatureAlgorithm",
    "HSMConfig",
    "HSMSigner",
    "YubiHSMSigner",
    "SoftHSMSigner",
    "create_signer",
    "create_sign_function",
    "create_verify_function",
]

