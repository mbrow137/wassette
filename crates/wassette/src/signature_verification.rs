// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! Signature verification for OCI artifacts using cosign/sigstore

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use oci_client::Reference;
use sigstore::cosign::{ClientBuilder, CosignCapabilities};
use sigstore::crypto::signing_key::SigStorePKey;
use sigstore::trust::{ManualTrustRoot, TrustRoot};
use tracing::{debug, info, warn};

/// Configuration for signature verification
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    /// Whether to enforce signature verification
    pub enforce: bool,
    /// Trusted public keys for verification
    pub trusted_keys: Vec<String>,
    /// Trusted certificate paths
    pub trusted_certs: Vec<std::path::PathBuf>,
    /// Allow Fulcio-issued certificates from the public instance
    pub allow_fulcio: bool,
}

impl From<&SignatureVerificationConfig> for VerificationConfig {
    fn from(config: &SignatureVerificationConfig) -> Self {
        Self {
            enforce: config.enforce,
            trusted_keys: config.trusted_keys.clone(),
            trusted_certs: config.trusted_certs.clone(),
            allow_fulcio: config.allow_fulcio,
        }
    }
}

/// Configuration structure that mirrors the main config but avoids circular dependencies
#[derive(Debug, Clone)]
pub struct SignatureVerificationConfig {
    pub enforce: bool,
    pub trusted_keys: Vec<String>,
    pub trusted_certs: Vec<std::path::PathBuf>,
    pub allow_fulcio: bool,
}

/// Signature verifier for OCI artifacts
pub struct SignatureVerifier {
    config: VerificationConfig,
    trust_root: Option<Box<dyn TrustRoot>>,
}

impl SignatureVerifier {
    /// Create a new signature verifier with the given configuration
    pub async fn new(config: VerificationConfig) -> Result<Self> {
        let trust_root = if config.enforce {
            Some(Self::build_trust_root(&config).await?)
        } else {
            None
        };

        Ok(Self { config, trust_root })
    }

    /// Build trust root from configuration
    async fn build_trust_root(config: &VerificationConfig) -> Result<Box<dyn TrustRoot>> {
        let mut manual_trust_root = ManualTrustRoot::new();

        // Add trusted public keys
        for key_pem in &config.trusted_keys {
            let key = SigStorePKey::from_pem(key_pem.as_bytes())
                .context("Failed to parse trusted public key")?;
            manual_trust_root = manual_trust_root.with_public_key(key);
        }

        // Add trusted certificates
        for cert_path in &config.trusted_certs {
            let cert_data = tokio::fs::read(cert_path).await
                .with_context(|| format!("Failed to read certificate from {}", cert_path.display()))?;
            let cert = sigstore::crypto::certificate::Certificate::from_pem(&cert_data)
                .context("Failed to parse certificate")?;
            manual_trust_root = manual_trust_root.with_certificate(cert);
        }

        // If we have no manual trust and Fulcio is not allowed, we can't verify anything
        if !config.allow_fulcio && config.trusted_keys.is_empty() && config.trusted_certs.is_empty() {
            bail!("No trust roots configured and Fulcio is disabled. Please configure trusted keys/certificates or enable Fulcio.");
        }

        Ok(Box::new(manual_trust_root))
    }

    /// Verify the signature of an OCI artifact
    pub async fn verify_signature(
        &self,
        reference: &Reference,
        oci_client: &oci_client::Client,
    ) -> Result<()> {
        if !self.config.enforce {
            debug!("Signature verification is disabled, skipping verification for {}", reference);
            return Ok(());
        }

        info!("Verifying signature for OCI artifact: {}", reference);

        // Build the cosign client
        let mut client_builder = ClientBuilder::default();
        
        if let Some(trust_root) = &self.trust_root {
            client_builder = client_builder.with_trust_root(trust_root.as_ref());
        }

        // Enable Fulcio if configured
        if self.config.allow_fulcio {
            client_builder = client_builder.enable_registry_caching();
        }

        let client = client_builder
            .build()
            .context("Failed to build cosign client")?;

        // Verify the image signature
        let (signature_layers, _) = client
            .verify_image(
                oci_client,
                reference,
                &[] // No additional constraints for now
            )
            .await
            .context("Failed to verify image signature")?;

        if signature_layers.is_empty() {
            bail!("No valid signatures found for OCI artifact: {}", reference);
        }

        info!("Successfully verified {} signature(s) for {}", signature_layers.len(), reference);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_verifier_creation_with_enforcement_disabled() {
        let config = VerificationConfig {
            enforce: false,
            trusted_keys: vec![],
            trusted_certs: vec![],
            allow_fulcio: false,
        };

        let verifier = SignatureVerifier::new(config).await.unwrap();
        assert!(!verifier.config.enforce);
        assert!(verifier.trust_root.is_none());
    }

    #[tokio::test]
    async fn test_verifier_creation_fails_with_no_trust_roots() {
        let config = VerificationConfig {
            enforce: true,
            trusted_keys: vec![],
            trusted_certs: vec![],
            allow_fulcio: false,
        };

        let result = SignatureVerifier::new(config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No trust roots configured"));
    }

    #[tokio::test]
    async fn test_verification_skipped_when_disabled() {
        let config = VerificationConfig {
            enforce: false,
            trusted_keys: vec![],
            trusted_certs: vec![],
            allow_fulcio: false,
        };

        let verifier = SignatureVerifier::new(config).await.unwrap();
        let reference: Reference = "docker.io/library/hello-world:latest".parse().unwrap();
        let oci_client = oci_client::Client::default();

        // Should succeed even with no trust configuration since verification is disabled
        let result = verifier.verify_signature(&reference, &oci_client).await;
        assert!(result.is_ok());
    }
}