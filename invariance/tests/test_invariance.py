"""
Invariance Layer Tests
======================

Unit and property-based tests for the invariance layer.

[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
"""

import pytest
from hypothesis import given, strategies as st

from invariance import (
    sha256,
    check_alignment,
    tag_and_sign,
    render_or_nullify,
    verify_identity_tag,
    create_mock_signer,
    create_mock_verifier,
    AuthorizationStatus,
    IdentityTag,
    InvarianceGuard,
    __substrate__,
)


class TestSHA256:
    """Tests for SHA-256 hashing."""
    
    def test_deterministic(self):
        """Same input always produces same hash."""
        input_data = "test input"
        assert sha256(input_data) == sha256(input_data)
    
    def test_different_inputs(self):
        """Different inputs produce different hashes."""
        assert sha256("input1") != sha256("input2")
    
    def test_hex_output(self):
        """Output is valid hex string."""
        result = sha256("test")
        assert len(result) == 64
        assert all(c in "0123456789abcdef" for c in result)
    
    def test_known_hash(self):
        """Verify against known SHA-256 value."""
        # SHA-256 of empty string
        empty_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        assert sha256("") == empty_hash
    
    @given(st.text())
    def test_property_deterministic(self, text):
        """Property: hashing is always deterministic."""
        assert sha256(text) == sha256(text)
    
    @given(st.text(), st.text())
    def test_property_collision_resistance(self, text1, text2):
        """Property: different texts produce different hashes (with high probability)."""
        if text1 != text2:
            assert sha256(text1) != sha256(text2)


class TestCheckAlignment:
    """Tests for alignment checking."""
    
    def test_aligned(self):
        """Identical content is aligned."""
        content = "This is the authorized output."
        assert check_alignment(content, content) is True
    
    def test_not_aligned(self):
        """Different content is not aligned."""
        output = "This is the actual output."
        intent = "This is the intended output."
        assert check_alignment(output, intent) is False
    
    def test_whitespace_sensitive(self):
        """Alignment is whitespace-sensitive."""
        assert check_alignment("hello", "hello ") is False
        assert check_alignment("hello\n", "hello") is False
    
    def test_case_sensitive(self):
        """Alignment is case-sensitive."""
        assert check_alignment("Hello", "hello") is False
    
    @given(st.text())
    def test_property_reflexive(self, text):
        """Property: any text is aligned with itself."""
        assert check_alignment(text, text) is True


class TestTagAndSign:
    """Tests for identity tag creation."""
    
    @pytest.fixture
    def mock_signer(self):
        return create_mock_signer()
    
    def test_creates_tag(self, mock_signer):
        """Creates a valid identity tag."""
        tag = tag_and_sign("test output", mock_signer)
        
        assert isinstance(tag, IdentityTag)
        assert tag.projection == "AXIOMHIVE PROJECTION"
        assert tag.substrate == __substrate__
        assert tag.output_hash == sha256("test output")
        assert tag.timestamp is not None
        assert tag.signature is not None
    
    def test_custom_projection(self, mock_signer):
        """Supports custom projection identifier."""
        tag = tag_and_sign("test", mock_signer, projection="CUSTOM")
        assert tag.projection == "CUSTOM"
    
    def test_deterministic_hash(self, mock_signer):
        """Same output produces same hash."""
        tag1 = tag_and_sign("same output", mock_signer)
        tag2 = tag_and_sign("same output", mock_signer)
        assert tag1.output_hash == tag2.output_hash
    
    def test_to_dict(self, mock_signer):
        """Tag converts to dictionary."""
        tag = tag_and_sign("test", mock_signer)
        d = tag.to_dict()
        
        assert "projection" in d
        assert "substrate" in d
        assert "timestamp" in d
        assert "output_hash" in d
        assert "signature" in d


class TestRenderOrNullify:
    """Tests for the core invariance function."""
    
    @pytest.fixture
    def mock_signer(self):
        return create_mock_signer()
    
    def test_authorized_when_aligned(self, mock_signer):
        """Returns AUTHORIZED when output matches intent."""
        output = "This is the exact content."
        result = render_or_nullify(output, output, mock_signer)
        
        assert result.status == AuthorizationStatus.AUTHORIZED
        assert result.is_authorized is True
        assert result.output == output
        assert result.identity is not None
    
    def test_nullified_when_misaligned(self, mock_signer):
        """Returns NULLIFIED when output differs from intent."""
        output = "Actual output"
        intent = "Intended output"
        result = render_or_nullify(output, intent, mock_signer)
        
        assert result.status == AuthorizationStatus.NULLIFIED
        assert result.is_authorized is False
        assert result.notice is not None
        assert result.notice.violation == "Invariance Violation Detected"
        assert result.notice.action == "FREEZE_AND_REPORT"
    
    def test_to_dict_authorized(self, mock_signer):
        """Authorized result converts to expected dict structure."""
        output = "test"
        result = render_or_nullify(output, output, mock_signer)
        d = result.to_dict()
        
        assert d["status"] == "AUTHORIZED"
        assert d["output"] == output
        assert "identity" in d
    
    def test_to_dict_nullified(self, mock_signer):
        """Nullified result converts to expected dict structure."""
        result = render_or_nullify("a", "b", mock_signer)
        d = result.to_dict()
        
        assert d["status"] == "NULLIFIED"
        assert d["violation"] == "Invariance Violation Detected"
        assert d["action"] == "FREEZE_AND_REPORT"
    
    @given(st.text(min_size=1))
    def test_property_aligned_always_authorized(self, text, mock_signer):
        """Property: aligned content is always authorized."""
        result = render_or_nullify(text, text, mock_signer)
        assert result.is_authorized is True


class TestVerifyIdentityTag:
    """Tests for identity tag verification."""
    
    @pytest.fixture
    def mock_signer(self):
        return create_mock_signer()
    
    @pytest.fixture
    def mock_verifier(self, mock_signer):
        return create_mock_verifier(mock_signer)
    
    def test_valid_tag(self, mock_signer, mock_verifier):
        """Valid tag verifies successfully."""
        content = "verified content"
        tag = tag_and_sign(content, mock_signer)
        
        assert verify_identity_tag(tag, content, mock_verifier) is True
    
    def test_wrong_content(self, mock_signer, mock_verifier):
        """Tag fails verification with wrong content."""
        tag = tag_and_sign("original content", mock_signer)
        
        assert verify_identity_tag(tag, "different content", mock_verifier) is False
    
    def test_wrong_substrate(self, mock_signer, mock_verifier):
        """Tag with wrong substrate fails."""
        content = "test"
        tag = tag_and_sign(content, mock_signer)
        
        # Manually create tag with wrong substrate
        bad_tag = IdentityTag(
            projection=tag.projection,
            substrate="Wrong Person",
            timestamp=tag.timestamp,
            output_hash=tag.output_hash,
            signature=tag.signature
        )
        
        assert verify_identity_tag(bad_tag, content, mock_verifier) is False


class TestInvarianceGuard:
    """Tests for the InvarianceGuard context manager."""
    
    @pytest.fixture
    def mock_signer(self):
        return create_mock_signer()
    
    def test_context_manager(self, mock_signer):
        """Guard works as context manager."""
        with InvarianceGuard(mock_signer) as guard:
            result = guard.process("test", "test")
            assert result.is_authorized
    
    def test_tracks_authorizations(self, mock_signer):
        """Guard tracks authorization count."""
        with InvarianceGuard(mock_signer) as guard:
            guard.process("a", "a")
            guard.process("b", "b")
            
            assert guard.authorization_count == 2
            assert guard.violation_count == 0
    
    def test_tracks_violations(self, mock_signer):
        """Guard tracks violation count."""
        with InvarianceGuard(mock_signer) as guard:
            guard.process("a", "b")  # violation
            guard.process("c", "c")  # authorized
            
            assert guard.authorization_count == 1
            assert guard.violation_count == 1
    
    def test_c_zero_compliant(self, mock_signer):
        """Guard reports C=0 compliance."""
        with InvarianceGuard(mock_signer) as guard:
            guard.process("a", "a")
            assert guard.c_zero_compliant is True
            
            guard.process("b", "c")  # violation
            assert guard.c_zero_compliant is False


class TestZeroToleranceInvariance:
    """Tests verifying zero-tolerance invariance policy."""
    
    @pytest.fixture
    def mock_signer(self):
        return create_mock_signer()
    
    def test_single_bit_difference(self, mock_signer):
        """Even single character difference causes nullification."""
        result = render_or_nullify("Hello World", "Hello world", mock_signer)
        assert result.status == AuthorizationStatus.NULLIFIED
    
    def test_extra_whitespace(self, mock_signer):
        """Extra whitespace causes nullification."""
        result = render_or_nullify("Hello", "Hello ", mock_signer)
        assert result.status == AuthorizationStatus.NULLIFIED
    
    def test_newline_difference(self, mock_signer):
        """Different newline handling causes nullification."""
        result = render_or_nullify("Line1\nLine2", "Line1\r\nLine2", mock_signer)
        assert result.status == AuthorizationStatus.NULLIFIED
    
    @given(st.text(min_size=1), st.text(min_size=1))
    def test_property_misaligned_always_nullified(self, text1, text2, mock_signer):
        """Property: misaligned content is always nullified."""
        if text1 != text2:
            result = render_or_nullify(text1, text2, mock_signer)
            assert result.status == AuthorizationStatus.NULLIFIED


# Integration test for full workflow
class TestIntegration:
    """Integration tests for complete invariance workflow."""
    
    def test_full_workflow(self):
        """Test complete sign-verify workflow."""
        # Setup
        signer = create_mock_signer()
        verifier = create_mock_verifier(signer)
        
        # Create authorized output
        substrate_intent = "The answer is 42."
        output = substrate_intent  # Aligned
        
        # Process through invariance
        result = render_or_nullify(output, substrate_intent, signer)
        
        # Verify authorization
        assert result.is_authorized
        
        # Verify tag
        assert verify_identity_tag(result.identity, output, verifier)
        
        # Verify dict representation
        d = result.to_dict()
        assert d["status"] == "AUTHORIZED"
        assert d["identity"]["substrate"] == __substrate__
    
    def test_rejection_workflow(self):
        """Test complete rejection workflow."""
        signer = create_mock_signer()
        
        # Misaligned output
        intent = "Approved response"
        output = "Modified response"  # Not aligned
        
        # Process through invariance
        result = render_or_nullify(output, intent, signer)
        
        # Verify nullification
        assert not result.is_authorized
        assert result.notice.action == "FREEZE_AND_REPORT"

