# OCI Signature Verification

Wassette supports cryptographic signature verification for OCI artifacts using cosign/sigstore to ensure the integrity and authenticity of loaded components.

## Security Model

**By default, signature verification is ENABLED and mandatory.** This means:
- All OCI artifacts (`oci://` scheme) must be signed to be loaded
- Unsigned components will be rejected
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

## Disabling Verification (NOT RECOMMENDED)

For development or testing purposes only, you can disable signature verification:

```toml
[signature_verification]
enforce = false
```

**Security Warning:** Disabling signature verification allows loading of unsigned, potentially malicious components. Only disable in trusted development environments.

## Signing Components

To sign your components for use with Wassette:

1. **Using cosign CLI:**
   ```bash
   # Sign with a private key
   cosign sign --key cosign.key oci://registry.example.com/my-component:latest
   
   # Sign with keyless (Fulcio)
   cosign sign oci://registry.example.com/my-component:latest
   ```

2. **Using GitHub Actions:**
   ```yaml
   - name: Sign container image
     run: |
       cosign sign --yes oci://registry.example.com/my-component:${{ github.sha }}
   ```

## Error Messages

Common signature verification errors:

- `Signature verification failed: No valid signatures found` - The OCI artifact is not signed
- `No trust roots configured and Fulcio is disabled` - No trusted keys/certificates configured
- `Failed to verify image signature` - Signature exists but doesn't match trusted keys

## Best Practices

1. **Always keep verification enabled** in production environments
2. **Use specific trusted keys** rather than allowing all Fulcio certificates
3. **Rotate signing keys** regularly and update trusted_keys configuration
4. **Monitor verification logs** for any failed attempts
5. **Test signature verification** in staging environments before deploying to production