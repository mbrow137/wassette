// Simple test to check signature verification compilation
#[cfg(test)]
mod signature_verification_tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_signature_verification_config() {
        let config = crate::signature_verification::SignatureVerificationConfig {
            enforce: true,
            trusted_keys: vec!["test-key".to_string()],
            trusted_certs: vec![],
            allow_fulcio: false,
        };

        let verification_config = crate::signature_verification::VerificationConfig::from(&config);
        assert_eq!(verification_config.enforce, true);
        assert_eq!(verification_config.trusted_keys.len(), 1);
    }

    #[tokio::test]
    async fn test_lifecycle_manager_creation_with_verification_disabled() {
        let temp_dir = TempDir::new().unwrap();
        
        let config = crate::config::SignatureVerificationConfig {
            enforce: false,
            trusted_keys: vec![],
            trusted_certs: vec![],
            allow_fulcio: false,
        };

        // This should succeed even with no trust configuration since verification is disabled
        let result = LifecycleManager::new_with_config(temp_dir.path(), &config).await;
        assert!(result.is_ok());
    }
}