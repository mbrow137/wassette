// Wassette Management Interface JavaScript

let components = [];
let activityEvents = [];
let allComponents = []; // Store unfiltered components

// Initialize the application
document.addEventListener('DOMContentLoaded', function() {
    initializeApp();
});

async function initializeApp() {
    await loadComponents();
    await loadActivity();
    setupEventListeners();
    startPeriodicRefresh();
}

// Tab management
function showTab(tabName) {
    // Hide all tabs
    document.querySelectorAll('.tab-content').forEach(tab => {
        tab.classList.remove('active');
    });
    
    // Remove active class from all buttons
    document.querySelectorAll('.tab-button').forEach(button => {
        button.classList.remove('active');
    });
    
    // Show selected tab
    document.getElementById(tabName + '-tab').classList.add('active');
    
    // Add active class to clicked button
    event.target.classList.add('active');
    
    // Load data for the tab
    switch(tabName) {
        case 'components':
            loadComponents();
            break;
        case 'activity':
            loadActivity();
            break;
        case 'permissions':
            loadPermissionsOverview();
            break;
    }
}

// Component management
async function loadComponents() {
    try {
        showLoading('components-list');
        const response = await fetch('/api/components');
        if (response.ok) {
            allComponents = await response.json();
            components = [...allComponents];
            renderComponents();
        } else {
            showError('Failed to load components');
        }
    } catch (error) {
        console.error('Error loading components:', error);
        showError('Failed to load components');
    }
}

function renderComponents() {
    const container = document.getElementById('components-list');
    
    if (components.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <h3>No components found</h3>
                <p>Load your first component to get started</p>
                <button onclick="loadComponent()" class="btn btn-primary">Load Component</button>
            </div>
        `;
        return;
    }
    
    container.innerHTML = components.map(component => `
        <div class="component-card" onclick="showComponentDetails('${component.id}')">
            <div class="component-header">
                <div>
                    <div class="component-name">${component.name}</div>
                    <div class="component-id">${component.id}</div>
                </div>
                <div class="component-status ${component.enabled ? 'status-enabled' : 'status-disabled'}">
                    ${component.enabled ? 'Enabled' : 'Disabled'}
                </div>
            </div>
            
            <div class="component-info">
                <div class="info-item">
                    <div class="info-value">${component.tool_count}</div>
                    <div class="info-label">Tools</div>
                </div>
                <div class="info-item">
                    <div class="info-value">${component.policy_file ? '✓' : '✗'}</div>
                    <div class="info-label">Policy</div>
                </div>
            </div>
            
            <div class="component-actions" onclick="event.stopPropagation()">
                <button class="btn btn-small btn-secondary" onclick="showComponentDetails('${component.id}')">
                    Details
                </button>
                <button class="btn btn-small btn-danger" onclick="unloadComponent('${component.id}')">
                    Unload
                </button>
            </div>
        </div>
    `).join('');
}

function filterComponents() {
    const searchTerm = document.getElementById('component-search').value.toLowerCase();
    components = allComponents.filter(component => 
        component.id.toLowerCase().includes(searchTerm) ||
        component.name.toLowerCase().includes(searchTerm)
    );
    renderComponents();
}

async function showComponentDetails(componentId) {
    try {
        const response = await fetch(`/api/components/${componentId}`);
        if (response.ok) {
            const component = await response.json();
            const permissionsResponse = await fetch(`/api/components/${componentId}/permissions`);
            const permissions = permissionsResponse.ok ? await permissionsResponse.json() : null;
            
            document.getElementById('modal-title').textContent = `${component.name} Details`;
            document.getElementById('component-details').innerHTML = `
                <div class="component-detail-section">
                    <h4>Basic Information</h4>
                    <p><strong>ID:</strong> ${component.id}</p>
                    <p><strong>Name:</strong> ${component.name}</p>
                    <p><strong>Tool Count:</strong> ${component.tool_count}</p>
                    <p><strong>Status:</strong> ${component.enabled ? 'Enabled' : 'Disabled'}</p>
                    <p><strong>Policy File:</strong> ${component.policy_file || 'None'}</p>
                </div>
                
                ${permissions ? `
                <div class="component-detail-section">
                    <h4>Permissions</h4>
                    <div class="permission-section">
                        <h5>Network Permissions</h5>
                        ${permissions.network.length > 0 ? 
                            permissions.network.map(p => `<div class="permission-item">${p}</div>`).join('') :
                            '<div class="permission-item">No network permissions</div>'
                        }
                    </div>
                    <div class="permission-section">
                        <h5>Storage Permissions</h5>
                        ${permissions.storage.length > 0 ? 
                            permissions.storage.map(p => `<div class="permission-item">${p}</div>`).join('') :
                            '<div class="permission-item">No storage permissions</div>'
                        }
                    </div>
                    <div class="permission-section">
                        <h5>Environment Permissions</h5>
                        ${permissions.environment.length > 0 ? 
                            permissions.environment.map(p => `<div class="permission-item">${p}</div>`).join('') :
                            '<div class="permission-item">No environment permissions</div>'
                        }
                    </div>
                </div>
                ` : ''}
                
                ${component.metadata ? `
                <div class="component-detail-section">
                    <h4>Metadata</h4>
                    <pre style="background: #f8f9fa; padding: 15px; border-radius: 6px; overflow-x: auto;">
${JSON.stringify(component.metadata, null, 2)}</pre>
                </div>
                ` : ''}
            `;
            
            document.getElementById('component-modal').classList.add('active');
        }
    } catch (error) {
        console.error('Error loading component details:', error);
        showError('Failed to load component details');
    }
}

function loadComponent() {
    document.getElementById('load-modal').classList.add('active');
}

function updateLoadForm() {
    const source = document.getElementById('component-source').value;
    
    // Hide all fields
    document.getElementById('oci-fields').style.display = 'none';
    document.getElementById('file-fields').style.display = 'none';
    document.getElementById('url-fields').style.display = 'none';
    
    // Show relevant field
    switch(source) {
        case 'oci':
            document.getElementById('oci-fields').style.display = 'block';
            break;
        case 'file':
            document.getElementById('file-fields').style.display = 'block';
            break;
        case 'url':
            document.getElementById('url-fields').style.display = 'block';
            break;
    }
}

async function unloadComponent(componentId) {
    if (!confirm(`Are you sure you want to unload component "${componentId}"?`)) {
        return;
    }
    
    try {
        const response = await fetch(`/api/components/${componentId}/unload`, {
            method: 'DELETE'
        });
        
        if (response.ok) {
            showSuccess('Component unloaded successfully');
            await loadComponents();
            await loadActivity();
        } else {
            showError('Failed to unload component');
        }
    } catch (error) {
        console.error('Error unloading component:', error);
        showError('Failed to unload component');
    }
}

// Activity feed
async function loadActivity() {
    try {
        const response = await fetch('/api/events');
        if (response.ok) {
            activityEvents = await response.json();
            renderActivity();
        }
    } catch (error) {
        console.error('Error loading activity:', error);
    }
}

function renderActivity() {
    const container = document.getElementById('activity-feed');
    
    if (activityEvents.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <h3>No activity yet</h3>
                <p>Component operations will appear here</p>
            </div>
        `;
        return;
    }
    
    // Sort events by timestamp (newest first)
    const sortedEvents = [...activityEvents].sort((a, b) => 
        new Date(b.timestamp) - new Date(a.timestamp)
    );
    
    container.innerHTML = sortedEvents.map(event => `
        <div class="activity-item ${event.success ? 'success' : 'error'}">
            <div class="activity-icon ${event.success ? 'success' : 'error'}">
                ${getActivityIcon(event.event_type, event.success)}
            </div>
            <div class="activity-content">
                <div class="activity-title">${event.event_type.replace('_', ' ')}</div>
                <div class="activity-description">${event.description}</div>
                ${event.component_id ? `<div class="activity-component">Component: ${event.component_id}</div>` : ''}
                <div class="activity-time">${formatTime(event.timestamp)}</div>
            </div>
        </div>
    `).join('');
}

function getActivityIcon(eventType, success) {
    if (!success) return '❌';
    
    switch(eventType) {
        case 'component_load': return '📦';
        case 'component_unload': return '🗑️';
        case 'permission_grant': return '🔓';
        case 'permission_revoke': return '🔒';
        case 'tool_execution': return '⚙️';
        default: return '📝';
    }
}

function filterActivity() {
    const eventType = document.getElementById('event-type-filter').value;
    renderActivity(); // For now, just re-render all events
}

// Permissions overview
async function loadPermissionsOverview() {
    const container = document.getElementById('permissions-overview');
    container.innerHTML = `
        <div class="loading">Loading permissions overview...</div>
    `;
    
    // This would aggregate permissions across all components
    setTimeout(() => {
        container.innerHTML = `
            <div class="empty-state">
                <h3>Permissions Overview</h3>
                <p>Component-specific permissions can be viewed in the component details</p>
            </div>
        `;
    }, 1000);
}

// Event listeners
function setupEventListeners() {
    // Load component form
    document.getElementById('load-component-form').addEventListener('submit', async function(e) {
        e.preventDefault();
        
        const source = document.getElementById('component-source').value;
        const componentId = document.getElementById('component-id').value;
        
        let payload = {};
        
        switch(source) {
            case 'oci':
                payload.source = document.getElementById('oci-reference').value;
                break;
            case 'file':
                payload.source = 'file://' + document.getElementById('file-path').value;
                break;
            case 'url':
                payload.source = document.getElementById('component-url').value;
                break;
        }
        
        if (componentId) {
            payload.component_id = componentId;
        }
        
        try {
            const response = await fetch(`/api/components/${componentId || 'new'}/load`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(payload)
            });
            
            if (response.ok) {
                showSuccess('Component loaded successfully');
                closeLoadModal();
                await loadComponents();
                await loadActivity();
            } else {
                showError('Failed to load component');
            }
        } catch (error) {
            console.error('Error loading component:', error);
            showError('Failed to load component');
        }
    });
}

// Modal management
function closeModal() {
    document.getElementById('component-modal').classList.remove('active');
}

function closeLoadModal() {
    document.getElementById('load-modal').classList.remove('active');
    // Reset form
    document.getElementById('load-component-form').reset();
    updateLoadForm();
}

// Click outside modal to close
window.onclick = function(event) {
    const componentModal = document.getElementById('component-modal');
    const loadModal = document.getElementById('load-modal');
    
    if (event.target === componentModal) {
        closeModal();
    }
    
    if (event.target === loadModal) {
        closeLoadModal();
    }
}

// Utility functions
function showLoading(containerId) {
    document.getElementById(containerId).innerHTML = `
        <div class="loading">Loading...</div>
    `;
}

function showError(message) {
    // Simple alert for now - could be replaced with toast notifications
    alert('Error: ' + message);
}

function showSuccess(message) {
    // Simple alert for now - could be replaced with toast notifications
    alert('Success: ' + message);
}

function formatTime(timestamp) {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now - date;
    
    if (diff < 60000) { // Less than 1 minute
        return 'Just now';
    } else if (diff < 3600000) { // Less than 1 hour
        return `${Math.floor(diff / 60000)} minutes ago`;
    } else if (diff < 86400000) { // Less than 1 day
        return `${Math.floor(diff / 3600000)} hours ago`;
    } else {
        return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
    }
}

async function refreshComponents() {
    await loadComponents();
}

async function refreshActivity() {
    await loadActivity();
}

function startPeriodicRefresh() {
    // Refresh data every 30 seconds
    setInterval(async () => {
        if (document.getElementById('components-tab').classList.contains('active')) {
            await loadComponents();
        }
        
        if (document.getElementById('activity-tab').classList.contains('active')) {
            await loadActivity();
        }
    }, 30000);
}