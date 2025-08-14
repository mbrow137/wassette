# OCI Signature Verification Framework

Wassette includes a configurable framework for OCI signature verification that will support cosign/sigstore to ensure the integrity and authenticity of loaded components.

## Current Status

**This is a framework implementation** - The configuration and integration points are fully implemented, but the actual cryptographic verification using cosign/sigstore will be added in a future release. Currently:

- Configuration is fully functional and validated
- OCI loading integration is complete and enforces verification policies
- Trust root configuration is validated
- Error handling and user feedback is implemented
- The system acts as a secure placeholder that can be enhanced with full verification

## Security Model

**By default, signature verification enforcement is ENABLED.** This means:
- When enforcement is enabled with proper trust configuration, components will be allowed (placeholder behavior)
- When enforcement is enabled without trust configuration, loading will fail securely
- When enforcement is disabled, all OCI artifacts are allowed
- Local files (`file://`) and HTTPS downloads are not subject to signature verification

## Configuration

Signature verification is configured through the `signature_verification` section in your config file:

```toml
[signature_verification]
# Whether to enforce signature verification (default: true)
enforce = true

# Trusted public keys in PEM format for signature verification
trusted_keys = [
    "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...\n-----END PUBLIC KEY-----"
]

# Paths to trusted certificate files
trusted_certs = [
    "/path/to/trusted-cert.pem"
]

# Allow Fulcio-issued certificates from the public sigstore instance (default: false)
allow_fulcio = false
```

## Environment Variables

You can also configure signature verification using environment variables:

```bash
# Disable signature verification (NOT RECOMMENDED for production)
export WASETTE_SIGNATURE_VERIFICATION_ENFORCE=false

# Enable Fulcio support
export WASETTE_SIGNATURE_VERIFICATION_ALLOW_FULCIO=true
```

## Trust Configuration

### Using Public Keys

To trust specific public keys, add them to the `trusted_keys` array in PEM format:

```toml
[signature_verification]
trusted_keys = [
    "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...\n-----END PUBLIC KEY-----"
]
```

### Using Certificates

To trust certificates, specify their file paths:

```toml
[signature_verification]
trusted_certs = [
    "/etc/wasette/trusted-certs/my-org.pem"
]
```

### Using Fulcio (Public Sigstore)

To allow components signed with Fulcio certificates from the public sigstore instance:

```toml
[signature_verification]
allow_fulcio = true
```

**Warning:** Only enable Fulcio in trusted environments, as it allows any valid sigstore signature.

## Disabling Verification

For environments where signature verification is not required, you can disable enforcement:

```toml
[signature_verification]
enforce = false
```

**Note:** When enforcement is disabled, all OCI artifacts will be allowed regardless of signature status.

## Future Implementation

The full cosign/sigstore integration will include:

1. **Cryptographic Verification**: Actual signature validation using cosign/sigstore
2. **Multiple Signature Formats**: Support for various signing formats
3. **Certificate Chain Validation**: Full certificate path validation for Fulcio
4. **Policy Constraints**: Advanced verification policies and constraints
5. **Performance Optimization**: Caching and efficient verification processes

## Error Messages

Current error messages you may encounter:

- `No trust roots configured and Fulcio is disabled` - No trusted keys/certificates configured when enforcement is enabled
- `Signature verification placeholder: allowing [reference]` - Indicates the framework is active but full verification is not yet implemented
- `Signature verification is enabled but no trust configuration provided` - Trust configuration is required when enforcement is enabled

## Best Practices

1. **Always keep verification enabled** in production environments
2. **Use specific trusted keys** rather than allowing all Fulcio certificates
3. **Rotate signing keys** regularly and update trusted_keys configuration
4. **Monitor verification logs** for any failed attempts
5. **Test signature verification** in staging environments before deploying to production