// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use anyhow::Result;
use tempfile::TempDir;
use wassette::{LifecycleManager, SecretMap};

#[tokio::test]
async fn test_secret_integration_with_lifecycle_manager() -> Result<()> {
    let plugin_dir = TempDir::new()?;
    let secrets_dir = TempDir::new()?;
    
    // Create lifecycle manager with custom secrets dir
    let manager = LifecycleManager::new_with_secrets_dir(&plugin_dir.path(), &secrets_dir.path()).await?;
    
    // Create a dummy component ID (in real usage, this would be from a loaded component)
    let component_id = "test-component";
    
    // Test setting secrets
    let mut secrets = SecretMap::new();
    secrets.insert("API_KEY".to_string(), "secret123".to_string());
    secrets.insert("REGION".to_string(), "us-west-2".to_string());
    
    // Set secrets using the secret manager
    manager.secret_manager().set_secrets(component_id, secrets.clone()).await?;
    
    // Test getting secrets
    let retrieved = manager.secret_manager().get_secrets(component_id).await?;
    assert_eq!(retrieved.len(), 2);
    assert_eq!(retrieved.get("API_KEY"), Some(&"secret123".to_string()));
    assert_eq!(retrieved.get("REGION"), Some(&"us-west-2".to_string()));
    
    // Test updating secrets
    let mut updates = std::collections::HashMap::new();
    updates.insert("API_KEY".to_string(), "updated_secret".to_string());
    updates.insert("NEW_VAR".to_string(), "new_value".to_string());
    
    manager.secret_manager().update_secrets(component_id, updates).await?;
    
    let updated = manager.secret_manager().get_secrets(component_id).await?;
    assert_eq!(updated.len(), 3);
    assert_eq!(updated.get("API_KEY"), Some(&"updated_secret".to_string()));
    assert_eq!(updated.get("REGION"), Some(&"us-west-2".to_string()));
    assert_eq!(updated.get("NEW_VAR"), Some(&"new_value".to_string()));
    
    // Test deleting keys
    let deleted = manager.secret_manager().delete_secret_keys(component_id, &["REGION".to_string()]).await?;
    assert_eq!(deleted, vec!["REGION"]);
    
    let final_secrets = manager.secret_manager().get_secrets(component_id).await?;
    assert_eq!(final_secrets.len(), 2);
    assert_eq!(final_secrets.get("API_KEY"), Some(&"updated_secret".to_string()));
    assert_eq!(final_secrets.get("NEW_VAR"), Some(&"new_value".to_string()));
    assert_eq!(final_secrets.get("REGION"), None);
    
    Ok(())
}

#[tokio::test]
async fn test_secret_file_persistence() -> Result<()> {
    let plugin_dir = TempDir::new()?;
    let secrets_dir = TempDir::new()?;
    
    let component_id = "persistent-test";
    
    // Create first manager and set secrets
    {
        let manager = LifecycleManager::new_with_secrets_dir(&plugin_dir.path(), &secrets_dir.path()).await?;
        
        let mut secrets = SecretMap::new();
        secrets.insert("PERSISTENT_KEY".to_string(), "persistent_value".to_string());
        
        manager.secret_manager().set_secrets(component_id, secrets).await?;
    }
    
    // Create second manager and verify secrets persist
    {
        let manager = LifecycleManager::new_with_secrets_dir(&plugin_dir.path(), &secrets_dir.path()).await?;
        
        let retrieved = manager.secret_manager().get_secrets(component_id).await?;
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved.get("PERSISTENT_KEY"), Some(&"persistent_value".to_string()));
    }
    
    Ok(())
}

#[tokio::test]
async fn test_secret_directory_creation() -> Result<()> {
    let plugin_dir = TempDir::new()?;
    let secrets_dir = plugin_dir.path().join("custom_secrets");
    
    // Ensure the secrets directory doesn't exist initially
    assert!(!secrets_dir.exists());
    
    let manager = LifecycleManager::new_with_secrets_dir(&plugin_dir.path(), &secrets_dir).await?;
    
    let component_id = "dir-test";
    let mut secrets = SecretMap::new();
    secrets.insert("TEST_KEY".to_string(), "test_value".to_string());
    
    // This should create the directory
    manager.secret_manager().set_secrets(component_id, secrets).await?;
    
    // Verify the directory was created
    assert!(secrets_dir.exists());
    
    // Verify the file was created with correct content
    let secret_file = secrets_dir.join("dir-test.yaml");
    assert!(secret_file.exists());
    
    let content = tokio::fs::read_to_string(&secret_file).await?;
    assert!(content.contains("TEST_KEY"));
    assert!(content.contains("test_value"));
    
    Ok(())
}

#[tokio::test]
async fn test_secret_sanitization() -> Result<()> {
    let plugin_dir = TempDir::new()?;
    let secrets_dir = TempDir::new()?;
    
    let manager = LifecycleManager::new_with_secrets_dir(&plugin_dir.path(), &secrets_dir.path()).await?;
    
    // Test component ID with special characters
    let component_id = "test/component:with@special#chars!";
    
    let mut secrets = SecretMap::new();
    secrets.insert("TEST_KEY".to_string(), "test_value".to_string());
    
    manager.secret_manager().set_secrets(component_id, secrets).await?;
    
    // Verify the file was created with sanitized name
    let expected_file = secrets_dir.path().join("test_component_with_special_chars_.yaml");
    assert!(expected_file.exists());
    
    // Verify we can still retrieve the secrets using the original ID
    let retrieved = manager.secret_manager().get_secrets(component_id).await?;
    assert_eq!(retrieved.len(), 1);
    assert_eq!(retrieved.get("TEST_KEY"), Some(&"test_value".to_string()));
    
    Ok(())
}