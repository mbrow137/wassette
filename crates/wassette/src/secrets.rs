// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! Secret management for Wassette components
//!
//! This module provides functionality to manage per-component secrets that are:
//! - Stored as YAML files in a dedicated directory
//! - Lazily loaded at component invocation
//! - Cached with mtime-based invalidation
//! - Injected as environment variables

use std::collections::HashMap;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::{Context, Result};
use etcetera::BaseStrategy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{debug, instrument, warn};

/// A flat map of secret key-value pairs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecretMap(HashMap<String, String>);

impl SecretMap {
    /// Create a new empty secret map
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    /// Remove a key
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    /// Get all keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// Get all key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the number of secrets
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Convert to HashMap for environment variable injection
    pub fn into_env_vars(self) -> HashMap<String, String> {
        self.0
    }
}

impl Default for SecretMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Cached secret data with metadata
#[derive(Debug, Clone)]
struct CachedSecrets {
    secrets: SecretMap,
    last_mtime: SystemTime,
}

/// Manager for component secrets
#[derive(Debug)]
pub struct SecretManager {
    secrets_dir: PathBuf,
    cache: Arc<Mutex<HashMap<String, CachedSecrets>>>,
}

impl SecretManager {
    /// Create a new SecretManager with the specified secrets directory
    pub fn new(secrets_dir: impl Into<PathBuf>) -> Self {
        Self {
            secrets_dir: secrets_dir.into(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new SecretManager with the default secrets directory
    pub fn with_default_dir() -> Result<Self> {
        let dir_strategy = etcetera::choose_base_strategy()
            .context("Unable to get home directory")?;
        let secrets_dir = dir_strategy.config_dir().join("wassette").join("secrets");
        Ok(Self::new(secrets_dir))
    }

    /// Get the path to a component's secret file
    fn get_secret_file_path(&self, component_id: &str) -> PathBuf {
        let sanitized_id = sanitize_component_id(component_id);
        self.secrets_dir.join(format!("{}.yaml", sanitized_id))
    }

    /// Ensure the secrets directory exists with proper permissions
    #[instrument(skip(self))]
    pub async fn ensure_secrets_dir(&self) -> Result<()> {
        if !self.secrets_dir.exists() {
            debug!("Creating secrets directory: {:?}", self.secrets_dir);
            tokio::fs::create_dir_all(&self.secrets_dir).await
                .context("Failed to create secrets directory")?;
        }

        // Set permissions to 0700 (user-only) on Unix systems
        #[cfg(unix)]
        {
            let metadata = tokio::fs::metadata(&self.secrets_dir).await
                .context("Failed to get secrets directory metadata")?;
            let permissions = metadata.permissions();
            
            if permissions.mode() & 0o777 != 0o700 {
                warn!("Secrets directory has incorrect permissions, fixing...");
                let new_permissions = Permissions::from_mode(0o700);
                tokio::fs::set_permissions(&self.secrets_dir, new_permissions).await
                    .context("Failed to set secrets directory permissions")?;
            }
        }

        Ok(())
    }

    /// Load secrets for a component from disk
    #[instrument(skip(self))]
    async fn load_secrets_from_disk(&self, component_id: &str) -> Result<(SecretMap, SystemTime)> {
        let file_path = self.get_secret_file_path(component_id);
        
        if !file_path.exists() {
            debug!("No secrets file found for component: {}", component_id);
            return Ok((SecretMap::new(), SystemTime::UNIX_EPOCH));
        }

        let metadata = tokio::fs::metadata(&file_path).await
            .context("Failed to get secret file metadata")?;
        let mtime = metadata.modified()
            .context("Failed to get file modification time")?;

        let content = tokio::fs::read_to_string(&file_path).await
            .context("Failed to read secrets file")?;

        let secrets: SecretMap = serde_yaml::from_str(&content)
            .context("Failed to parse secrets YAML")?;

        debug!("Loaded {} secrets for component: {}", secrets.len(), component_id);
        Ok((secrets, mtime))
    }

    /// Get secrets for a component, using cache if available and up-to-date
    #[instrument(skip(self))]
    pub async fn get_secrets(&self, component_id: &str) -> Result<SecretMap> {
        let file_path = self.get_secret_file_path(component_id);
        
        // Check if file exists first
        if !file_path.exists() {
            return Ok(SecretMap::new());
        }

        // Get current file mtime
        let metadata = tokio::fs::metadata(&file_path).await
            .context("Failed to get secret file metadata")?;
        let current_mtime = metadata.modified()
            .context("Failed to get file modification time")?;

        // Check cache
        {
            let cache = self.cache.lock().await;
            if let Some(cached) = cache.get(component_id) {
                if cached.last_mtime >= current_mtime {
                    debug!("Using cached secrets for component: {}", component_id);
                    return Ok(cached.secrets.clone());
                }
            }
        }

        // Load from disk and update cache
        let (secrets, mtime) = self.load_secrets_from_disk(component_id).await?;
        
        {
            let mut cache = self.cache.lock().await;
            cache.insert(component_id.to_string(), CachedSecrets {
                secrets: secrets.clone(),
                last_mtime: mtime,
            });
        }

        Ok(secrets)
    }

    /// Set secrets for a component (replace all existing secrets)
    #[instrument(skip(self, secrets))]
    pub async fn set_secrets(&self, component_id: &str, secrets: SecretMap) -> Result<()> {
        self.ensure_secrets_dir().await?;

        let file_path = self.get_secret_file_path(component_id);
        let content = serde_yaml::to_string(&secrets)
            .context("Failed to serialize secrets to YAML")?;

        // Atomic write: write to temp file, then rename
        let temp_path = file_path.with_extension("tmp");
        
        tokio::fs::write(&temp_path, &content).await
            .context("Failed to write temporary secrets file")?;

        // Set file permissions to 0600 (user read/write only)
        #[cfg(unix)]
        {
            let permissions = Permissions::from_mode(0o600);
            tokio::fs::set_permissions(&temp_path, permissions).await
                .context("Failed to set secret file permissions")?;
        }

        tokio::fs::rename(&temp_path, &file_path).await
            .context("Failed to rename temporary secrets file")?;

        // Invalidate cache
        {
            let mut cache = self.cache.lock().await;
            cache.remove(component_id);
        }

        debug!("Saved {} secrets for component: {}", secrets.len(), component_id);
        Ok(())
    }

    /// Update specific secrets for a component (merge with existing)
    #[instrument(skip(self, updates))]
    pub async fn update_secrets(&self, component_id: &str, updates: HashMap<String, String>) -> Result<()> {
        let mut secrets = self.get_secrets(component_id).await?;
        
        for (key, value) in updates {
            secrets.insert(key, value);
        }

        self.set_secrets(component_id, secrets).await
    }

    /// Delete specific secret keys for a component
    #[instrument(skip(self))]
    pub async fn delete_secret_keys(&self, component_id: &str, keys: &[String]) -> Result<Vec<String>> {
        let mut secrets = self.get_secrets(component_id).await?;
        let mut deleted_keys = Vec::new();

        for key in keys {
            if secrets.remove(key).is_some() {
                deleted_keys.push(key.clone());
            }
        }

        if !deleted_keys.is_empty() {
            if secrets.is_empty() {
                // If no secrets left, remove the file
                self.delete_secrets_file(component_id).await?;
            } else {
                self.set_secrets(component_id, secrets).await?;
            }
        }

        Ok(deleted_keys)
    }

    /// Delete the entire secrets file for a component
    #[instrument(skip(self))]
    pub async fn delete_secrets_file(&self, component_id: &str) -> Result<()> {
        let file_path = self.get_secret_file_path(component_id);
        
        if file_path.exists() {
            tokio::fs::remove_file(&file_path).await
                .context("Failed to remove secrets file")?;
            
            // Invalidate cache
            {
                let mut cache = self.cache.lock().await;
                cache.remove(component_id);
            }
            
            debug!("Deleted secrets file for component: {}", component_id);
        }

        Ok(())
    }

    /// List all component IDs that have secrets
    #[instrument(skip(self))]
    pub async fn list_components_with_secrets(&self) -> Result<Vec<String>> {
        if !self.secrets_dir.exists() {
            return Ok(Vec::new());
        }

        let mut components = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.secrets_dir).await
            .context("Failed to read secrets directory")?;

        while let Some(entry) = entries.next_entry().await
            .context("Failed to read directory entry")? {
            
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    components.push(stem.to_string());
                }
            }
        }

        Ok(components)
    }
}

/// Sanitize a component ID to create a safe filename
fn sanitize_component_id(component_id: &str) -> String {
    let mut sanitized = component_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();

    // Collapse multiple underscores
    while sanitized.contains("__") {
        sanitized = sanitized.replace("__", "_");
    }

    // Trim to 128 bytes (UTF-8 safe)
    if sanitized.len() > 128 {
        sanitized.truncate(128);
        // Ensure we don't cut in the middle of a UTF-8 character
        while !sanitized.is_char_boundary(sanitized.len()) {
            sanitized.pop();
        }
    }

    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_sanitize_component_id() {
        assert_eq!(sanitize_component_id("simple"), "simple");
        assert_eq!(sanitize_component_id("my-component.v1"), "my-component.v1");
        assert_eq!(sanitize_component_id("invalid/chars:here"), "invalid_chars_here");
        assert_eq!(sanitize_component_id("multiple___underscores"), "multiple_underscores");
        
        // Test truncation
        let long_id = "a".repeat(200);
        let sanitized = sanitize_component_id(&long_id);
        assert!(sanitized.len() <= 128);
        assert_eq!(sanitized, "a".repeat(128));
    }

    #[tokio::test]
    async fn test_secret_map_operations() {
        let mut secrets = SecretMap::new();
        assert!(secrets.is_empty());
        assert_eq!(secrets.len(), 0);

        secrets.insert("key1".to_string(), "value1".to_string());
        secrets.insert("key2".to_string(), "value2".to_string());
        
        assert!(!secrets.is_empty());
        assert_eq!(secrets.len(), 2);
        assert_eq!(secrets.get("key1"), Some(&"value1".to_string()));
        assert_eq!(secrets.get("key2"), Some(&"value2".to_string()));
        assert_eq!(secrets.get("nonexistent"), None);

        assert_eq!(secrets.remove("key1"), Some("value1".to_string()));
        assert_eq!(secrets.len(), 1);
        assert_eq!(secrets.remove("nonexistent"), None);
    }

    #[tokio::test]
    async fn test_secret_manager_basic_operations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = SecretManager::new(temp_dir.path());

        let component_id = "test-component";
        
        // Initially no secrets
        let secrets = manager.get_secrets(component_id).await?;
        assert!(secrets.is_empty());

        // Set some secrets
        let mut new_secrets = SecretMap::new();
        new_secrets.insert("API_KEY".to_string(), "secret123".to_string());
        new_secrets.insert("REGION".to_string(), "us-west-2".to_string());
        
        manager.set_secrets(component_id, new_secrets).await?;

        // Retrieve secrets
        let retrieved = manager.get_secrets(component_id).await?;
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved.get("API_KEY"), Some(&"secret123".to_string()));
        assert_eq!(retrieved.get("REGION"), Some(&"us-west-2".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_secret_manager_update_and_delete() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = SecretManager::new(temp_dir.path());

        let component_id = "test-component";
        
        // Set initial secrets
        let mut initial = SecretMap::new();
        initial.insert("KEY1".to_string(), "value1".to_string());
        initial.insert("KEY2".to_string(), "value2".to_string());
        manager.set_secrets(component_id, initial).await?;

        // Update some secrets
        let mut updates = HashMap::new();
        updates.insert("KEY2".to_string(), "updated_value2".to_string());
        updates.insert("KEY3".to_string(), "value3".to_string());
        
        manager.update_secrets(component_id, updates).await?;

        let secrets = manager.get_secrets(component_id).await?;
        assert_eq!(secrets.len(), 3);
        assert_eq!(secrets.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(secrets.get("KEY2"), Some(&"updated_value2".to_string()));
        assert_eq!(secrets.get("KEY3"), Some(&"value3".to_string()));

        // Delete some keys
        let deleted = manager.delete_secret_keys(component_id, &["KEY1".to_string(), "NONEXISTENT".to_string()]).await?;
        assert_eq!(deleted, vec!["KEY1"]);

        let secrets = manager.get_secrets(component_id).await?;
        assert_eq!(secrets.len(), 2);
        assert_eq!(secrets.get("KEY1"), None);
        assert_eq!(secrets.get("KEY2"), Some(&"updated_value2".to_string()));
        assert_eq!(secrets.get("KEY3"), Some(&"value3".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_secret_manager_caching() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = SecretManager::new(temp_dir.path());

        let component_id = "test-component";
        
        // Set initial secrets
        let mut secrets = SecretMap::new();
        secrets.insert("KEY1".to_string(), "value1".to_string());
        manager.set_secrets(component_id, secrets).await?;

        // First read should load from disk
        let secrets1 = manager.get_secrets(component_id).await?;
        assert_eq!(secrets1.len(), 1);

        // Second read should use cache
        let secrets2 = manager.get_secrets(component_id).await?;
        assert_eq!(secrets2.len(), 1);

        // Update should invalidate cache
        let mut updates = HashMap::new();
        updates.insert("KEY2".to_string(), "value2".to_string());
        manager.update_secrets(component_id, updates).await?;

        let secrets3 = manager.get_secrets(component_id).await?;
        assert_eq!(secrets3.len(), 2);

        Ok(())
    }
}