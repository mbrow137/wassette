// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use anyhow::{Context, Result};
use tempfile::TempDir;
use wassette::LifecycleManager;

mod common;
use common::build_html_to_markdown_component;

async fn setup_lifecycle_manager() -> Result<(LifecycleManager, TempDir)> {
    let tempdir = tempfile::tempdir().context("Failed to create temporary directory")?;
    let manager = LifecycleManager::new(&tempdir).await?;
    Ok((manager, tempdir))
}

#[tokio::test]
async fn test_html_to_markdown_basic_conversion() -> Result<()> {
    let (manager, _tempdir) = setup_lifecycle_manager().await?;
    let component_path = build_html_to_markdown_component().await?;

    let (component_id, _) = manager
        .load_component(&format!("file://{}", component_path.to_str().unwrap()))
        .await?;

    let html_input = r#"<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>"#;

    let result = manager
        .execute_component_call(
            &component_id,
            "convert",
            &serde_json::json!({"html": html_input}).to_string(),
        )
        .await?;

    println!("Component response: {result}");

    // Parse the response
    let response: serde_json::Value = serde_json::from_str(&result)?;
    let markdown = response["ok"]
        .as_str()
        .context("Expected ok field in response")?;

    assert!(markdown.contains("# Hello World"));
    assert!(markdown.contains("**test**"));

    Ok(())
}

#[tokio::test]
async fn test_html_to_markdown_complex_html() -> Result<()> {
    let (manager, _tempdir) = setup_lifecycle_manager().await?;
    let component_path = build_html_to_markdown_component().await?;

    let (component_id, _) = manager
        .load_component(&format!("file://{}", component_path.to_str().unwrap()))
        .await?;

    let html_input = r#"
        <h1>Documentation</h1>
        <p>This is an example with multiple elements:</p>
        <ul>
            <li>First item</li>
            <li>Second item</li>
        </ul>
        <blockquote>This is a quote</blockquote>
        <pre>let x = 5;</pre>
        <p>Link: <a href="https://example.com">Example</a></p>
    "#;

    let result = manager
        .execute_component_call(
            &component_id,
            "convert",
            &serde_json::json!({"html": html_input}).to_string(),
        )
        .await?;

    println!("Component response: {result}");

    // Parse the response
    let response: serde_json::Value = serde_json::from_str(&result)?;
    let markdown = response["ok"]
        .as_str()
        .context("Expected ok field in response")?;

    assert!(markdown.contains("# Documentation"));
    assert!(markdown.contains("- First item"));
    assert!(markdown.contains("- Second item"));
    assert!(markdown.contains("> This is a quote"));
    assert!(markdown.contains("```"));
    assert!(markdown.contains("let x = 5;"));
    assert!(markdown.contains("[Example](https://example.com)"));

    Ok(())
}

#[tokio::test]
async fn test_html_to_markdown_empty_input() -> Result<()> {
    let (manager, _tempdir) = setup_lifecycle_manager().await?;
    let component_path = build_html_to_markdown_component().await?;

    let (component_id, _) = manager
        .load_component(&format!("file://{}", component_path.to_str().unwrap()))
        .await?;

    let html_input = "";

    let result = manager
        .execute_component_call(
            &component_id,
            "convert",
            &serde_json::json!({"html": html_input}).to_string(),
        )
        .await?;

    println!("Component response: {result}");

    // Parse the response
    let response: serde_json::Value = serde_json::from_str(&result)?;
    let markdown = response["ok"]
        .as_str()
        .context("Expected ok field in response")?;

    assert_eq!(markdown, "");

    Ok(())
}

#[tokio::test]
async fn test_html_to_markdown_lists() -> Result<()> {
    let (manager, _tempdir) = setup_lifecycle_manager().await?;
    let component_path = build_html_to_markdown_component().await?;

    let (component_id, _) = manager
        .load_component(&format!("file://{}", component_path.to_str().unwrap()))
        .await?;

    let html_input = r#"
        <ol>
            <li>First ordered item</li>
            <li>Second ordered item</li>
        </ol>
        <ul>
            <li>First unordered item</li>
            <li>Second unordered item</li>
        </ul>
    "#;

    let result = manager
        .execute_component_call(
            &component_id,
            "convert",
            &serde_json::json!({"html": html_input}).to_string(),
        )
        .await?;

    println!("Component response: {result}");

    // Parse the response
    let response: serde_json::Value = serde_json::from_str(&result)?;
    let markdown = response["ok"]
        .as_str()
        .context("Expected ok field in response")?;

    assert!(markdown.contains("1. First ordered item"));
    assert!(markdown.contains("2. Second ordered item"));
    assert!(markdown.contains("- First unordered item"));
    assert!(markdown.contains("- Second unordered item"));

    Ok(())
}

#[tokio::test]
async fn test_html_to_markdown_code_elements() -> Result<()> {
    let (manager, _tempdir) = setup_lifecycle_manager().await?;
    let component_path = build_html_to_markdown_component().await?;

    let (component_id, _) = manager
        .load_component(&format!("file://{}", component_path.to_str().unwrap()))
        .await?;

    let html_input = r#"
        <p>Use the <code>print</code> function.</p>
        <pre>def hello():
    print("Hello, world!")</pre>
    "#;

    let result = manager
        .execute_component_call(
            &component_id,
            "convert",
            &serde_json::json!({"html": html_input}).to_string(),
        )
        .await?;

    println!("Component response: {result}");

    // Parse the response
    let response: serde_json::Value = serde_json::from_str(&result)?;
    let markdown = response["ok"]
        .as_str()
        .context("Expected ok field in response")?;

    assert!(markdown.contains("`print`"));
    assert!(markdown.contains("```"));
    assert!(markdown.contains("def hello():"));
    assert!(markdown.contains("print(\"Hello, world!\")"));

    Ok(())
}
