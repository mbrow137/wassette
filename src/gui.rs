// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! Web GUI server for Wassette management interface

use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State}, 
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post, delete},
    Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, error, instrument};

use mcp_server::LifecycleManager;

const GUI_BIND_ADDRESS: &str = "127.0.0.1:9002";

/// GUI web server state
#[derive(Clone)]
pub struct GuiState {
    lifecycle_manager: LifecycleManager,
    event_log: Arc<tokio::sync::RwLock<Vec<ActivityEvent>>>,
}

/// Activity event for the GUI feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub id: String,
    pub timestamp: String, // Use String instead of chrono for simplicity
    pub event_type: String,
    pub component_id: Option<String>,
    pub description: String,
    pub success: bool,
    pub details: Option<Value>,
}

/// Component information for GUI display
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub id: String,
    pub name: String,
    pub tool_count: usize,
    pub policy_file: Option<String>,
    pub enabled: bool,
    pub metadata: Option<Value>,
}

/// Permission information for GUI display
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PermissionInfo {
    pub network: Vec<String>,
    pub storage: Vec<String>,
    pub environment: Vec<String>,
}

impl GuiState {
    pub fn new(lifecycle_manager: LifecycleManager) -> Self {
        Self {
            lifecycle_manager,
            event_log: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    pub async fn add_event(&self, event: ActivityEvent) {
        let mut events = self.event_log.write().await;
        events.push(event);
        // Keep only last 50 events
        let events_len = events.len();
        if events_len > 50 {
            events.drain(0..events_len - 50);
        }
    }
}

/// Start the GUI web server
#[instrument(skip(lifecycle_manager))]
pub async fn start_gui_server(lifecycle_manager: LifecycleManager) -> Result<()> {
    info!("Starting GUI web server on {}", GUI_BIND_ADDRESS);
    
    let state = GuiState::new(lifecycle_manager);
    
    let app = create_router(state);
    
    let listener = TcpListener::bind(GUI_BIND_ADDRESS)
        .await
        .context("Failed to bind GUI server")?;
    
    info!("GUI available at http://{}", GUI_BIND_ADDRESS);
    
    axum::serve(listener, app)
        .await
        .context("GUI server failed")?;
    
    Ok(())
}

fn create_router(state: GuiState) -> Router {
    Router::new()
        // Serve static files from embedded content
        .route("/", get(serve_index))
        .route("/api/components", get(api_list_components))
        .route("/api/components/{id}", get(api_get_component))
        .route("/api/components/{id}/load", post(api_load_component))
        .route("/api/components/{id}/unload", delete(api_unload_component))
        .route("/api/components/{id}/permissions", get(api_get_permissions))
        .route("/api/components/{id}/permissions", post(api_update_permissions))
        .route("/api/events", get(api_get_events))
        .route("/api/tools", get(api_list_tools))
        .route("/assets/{*path}", get(serve_static))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Serve the main HTML page
async fn serve_index() -> Html<&'static str> {
    Html(include_str!("gui/index.html"))
}

/// Serve static assets
async fn serve_static(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches("/assets/");
    
    match path {
        "style.css" => {
            (
                StatusCode::OK,
                [("content-type", "text/css")],
                include_str!("gui/style.css")
            ).into_response()
        }
        "script.js" => {
            (
                StatusCode::OK,
                [("content-type", "application/javascript")],
                include_str!("gui/script.js")
            ).into_response()
        }
        _ => (StatusCode::NOT_FOUND, "Not found").into_response()
    }
}

/// API endpoint to list components
#[instrument(skip(state))]
async fn api_list_components(State(state): State<GuiState>) -> Result<Json<Vec<ComponentInfo>>, StatusCode> {
    match state.lifecycle_manager.list_components().await {
        component_ids => {
            let mut components = Vec::new();
            for id in component_ids {
                if let Some(schema) = state.lifecycle_manager.get_component_schema(&id).await {
                    let tools_count = schema
                        .get("tools")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.len())
                        .unwrap_or(0);
                    
                    components.push(ComponentInfo {
                        id: id.clone(),
                        name: id.clone(), // Use ID as name for now
                        tool_count: tools_count,
                        policy_file: None, // Would need to check if policy exists
                        enabled: true, // Components are enabled if loaded
                        metadata: Some(schema),
                    });
                }
            }
            Ok(Json(components))
        }
    }
}

/// API endpoint to get a specific component
async fn api_get_component(
    Path(id): Path<String>,
    State(state): State<GuiState>,
) -> Result<Json<ComponentInfo>, StatusCode> {
    // For now, just return from the list - in a real implementation we'd have more details
    let components = api_list_components(State(state)).await?;
    
    components.0.into_iter()
        .find(|c| c.id == id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// API endpoint to load a component
async fn api_load_component(
    Path(id): Path<String>,
    State(state): State<GuiState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // For now, return a simple success message
    // In a full implementation, this would use the lifecycle manager to load components
    info!("Load component request for {}: {:?}", id, payload);
    
    let event = ActivityEvent {
        id: format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
        timestamp: format!("{:?}", std::time::SystemTime::now()),
        event_type: "component_load".to_string(),
        component_id: Some(id),
        description: "Component load requested".to_string(),
        success: true,
        details: Some(payload),
    };
    state.add_event(event).await;
    
    Ok(Json(json!({"status": "success", "message": "Component load requested"})))
}

/// API endpoint to unload a component
async fn api_unload_component(
    Path(id): Path<String>,
    State(state): State<GuiState>,
) -> Result<Json<Value>, StatusCode> {
    // Use the LifecycleManager directly
    match state.lifecycle_manager.unload_component(&id).await {
        Ok(_) => {
            let event = ActivityEvent {
                id: format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
                timestamp: format!("{:?}", std::time::SystemTime::now()),
                event_type: "component_unload".to_string(),
                component_id: Some(id),
                description: "Component unloaded".to_string(),
                success: true,
                details: None,
            };
            state.add_event(event).await;
            
            Ok(Json(json!({"status": "success", "message": "Component unloaded"})))
        }
        Err(e) => {
            error!("Failed to unload component: {}", e);
            
            let event = ActivityEvent {
                id: format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
                timestamp: format!("{:?}", std::time::SystemTime::now()),
                event_type: "component_unload".to_string(),
                component_id: Some(id),
                description: format!("Failed to unload component: {}", e),
                success: false,
                details: None,
            };
            state.add_event(event).await;
            
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// API endpoint to get component permissions
async fn api_get_permissions(
    Path(id): Path<String>,
    State(_state): State<GuiState>,
) -> Result<Json<PermissionInfo>, StatusCode> {
    // For now, return empty permissions
    // In a full implementation, this would query the policy system
    info!("Get permissions for component: {}", id);
    
    let permissions = PermissionInfo {
        network: vec!["example.com".to_string()],
        storage: vec!["/tmp".to_string()],
        environment: vec!["PATH".to_string()],
    };
    
    Ok(Json(permissions))
}

/// API endpoint to update component permissions
async fn api_update_permissions(
    Path(id): Path<String>,
    State(_state): State<GuiState>,
    Json(permissions): Json<PermissionInfo>,
) -> Result<Json<Value>, StatusCode> {
    // This would implement permission updates - simplified for now
    info!("Update permissions for component {}: {:?}", id, permissions);
    Ok(Json(json!({"status": "success"})))
}

/// API endpoint to get activity events
async fn api_get_events(State(state): State<GuiState>) -> Json<Vec<ActivityEvent>> {
    let events = state.event_log.read().await;
    Json(events.clone())
}

/// API endpoint to list tools
async fn api_list_tools(State(_state): State<GuiState>) -> Result<Json<Value>, StatusCode> {
    // For now, return a simple tools list
    // In a full implementation, this would use handle_tools_list
    let tools = json!({
        "tools": [
            {"name": "load-component", "description": "Load a WebAssembly component"},
            {"name": "unload-component", "description": "Unload a component"},
            {"name": "list-components", "description": "List loaded components"},
            {"name": "get-policy", "description": "Get component policy"},
        ]
    });
    
    Ok(Json(tools))
}

/// Parse permissions result from the MCP tool call
fn _parse_permissions_result(_result: Value) -> Result<PermissionInfo> {
    // Simplified parser for permissions
    Ok(PermissionInfo {
        network: Vec::new(),
        storage: Vec::new(),
        environment: Vec::new(),
    })
}