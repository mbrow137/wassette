<script>
  import { onMount } from 'svelte';
  
  let globalPermissions = {
    network: [],
    storage: [],
    environment: []
  };
  let componentPermissions = [];
  let loading = false;
  
  onMount(() => {
    loadPermissionsOverview();
  });
  
  async function loadPermissionsOverview() {
    loading = true;
    try {
      // Load components first
      const componentsResponse = await fetch('/api/components');
      const components = await componentsResponse.json();
      
      // Load permissions for each component
      const permissionsPromises = components.map(async component => {
        try {
          const response = await fetch(`/api/components/${component.id}/permissions`);
          const permissions = await response.json();
          return {
            componentId: component.id,
            componentName: component.name,
            ...permissions
          };
        } catch (error) {
          console.error(`Failed to load permissions for ${component.id}:`, error);
          return {
            componentId: component.id,
            componentName: component.name,
            network: [],
            storage: [],
            environment: []
          };
        }
      });
      
      componentPermissions = await Promise.all(permissionsPromises);
      
      // Aggregate global permissions
      const allNetwork = new Set();
      const allStorage = new Set();
      const allEnvironment = new Set();
      
      componentPermissions.forEach(comp => {
        comp.network.forEach(perm => allNetwork.add(perm));
        comp.storage.forEach(perm => allStorage.add(perm));
        comp.environment.forEach(perm => allEnvironment.add(perm));
      });
      
      globalPermissions = {
        network: Array.from(allNetwork),
        storage: Array.from(allStorage),
        environment: Array.from(allEnvironment)
      };
      
    } catch (error) {
      console.error('Failed to load permissions overview:', error);
    } finally {
      loading = false;
    }
  }
  
  async function togglePermission(componentId, permissionType, permission, currentlyEnabled) {
    try {
      const component = componentPermissions.find(c => c.componentId === componentId);
      if (!component) return;
      
      const updatedPermissions = { ...component };
      
      if (currentlyEnabled) {
        updatedPermissions[permissionType] = updatedPermissions[permissionType].filter(p => p !== permission);
      } else {
        updatedPermissions[permissionType] = [...updatedPermissions[permissionType], permission];
      }
      
      const response = await fetch(`/api/components/${componentId}/permissions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          network: updatedPermissions.network,
          storage: updatedPermissions.storage,
          environment: updatedPermissions.environment
        })
      });
      
      if (response.ok) {
        await loadPermissionsOverview();
      } else {
        alert('Failed to update permissions');
      }
    } catch (error) {
      console.error('Failed to toggle permission:', error);
      alert('Failed to update permissions');
    }
  }
  
  function getPermissionIcon(permissionType) {
    switch (permissionType) {
      case 'network':
        return '🌐';
      case 'storage':
        return '💾';
      case 'environment':
        return '🔧';
      default:
        return '🔒';
    }
  }
</script>

<div class="permissions-header">
  <h2>Permissions Overview</h2>
  <button on:click={loadPermissionsOverview} class="btn btn-secondary">Refresh</button>
</div>

{#if loading}
  <div class="loading">Loading permissions...</div>
{:else}
  <!-- Global Permissions Summary -->
  <div class="global-permissions">
    <h3>Global Permissions Summary</h3>
    <div class="permission-categories">
      <div class="permission-category">
        <h4>🌐 Network</h4>
        <div class="permission-list">
          {#each globalPermissions.network as permission}
            <span class="permission-tag network">{permission}</span>
          {:else}
            <span class="no-permissions">No network permissions granted</span>
          {/each}
        </div>
      </div>
      
      <div class="permission-category">
        <h4>💾 Storage</h4>
        <div class="permission-list">
          {#each globalPermissions.storage as permission}
            <span class="permission-tag storage">{permission}</span>
          {:else}
            <span class="no-permissions">No storage permissions granted</span>
          {/each}
        </div>
      </div>
      
      <div class="permission-category">
        <h4>🔧 Environment</h4>
        <div class="permission-list">
          {#each globalPermissions.environment as permission}
            <span class="permission-tag environment">{permission}</span>
          {:else}
            <span class="no-permissions">No environment permissions granted</span>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Component-specific Permissions -->
  <div class="component-permissions">
    <h3>Component Permissions</h3>
    {#if componentPermissions.length === 0}
      <div class="empty-state">
        <p>No components loaded</p>
        <p class="subtitle">Load components to manage their permissions</p>
      </div>
    {:else}
      <div class="components-grid">
        {#each componentPermissions as component}
          <div class="component-card">
            <h4>{component.componentName}</h4>
            <div class="component-id">ID: {component.componentId}</div>
            
            <div class="permission-sections">
              <div class="permission-section">
                <h5>🌐 Network</h5>
                {#if component.network.length === 0}
                  <span class="no-permissions">No network access</span>
                {:else}
                  <div class="permission-toggles">
                    {#each component.network as permission}
                      <label class="permission-toggle">
                        <input 
                          type="checkbox" 
                          checked={true}
                          on:change={() => togglePermission(component.componentId, 'network', permission, true)}
                        />
                        <span>{permission}</span>
                      </label>
                    {/each}
                  </div>
                {/if}
              </div>
              
              <div class="permission-section">
                <h5>💾 Storage</h5>
                {#if component.storage.length === 0}
                  <span class="no-permissions">No storage access</span>
                {:else}
                  <div class="permission-toggles">
                    {#each component.storage as permission}
                      <label class="permission-toggle">
                        <input 
                          type="checkbox" 
                          checked={true}
                          on:change={() => togglePermission(component.componentId, 'storage', permission, true)}
                        />
                        <span>{permission}</span>
                      </label>
                    {/each}
                  </div>
                {/if}
              </div>
              
              <div class="permission-section">
                <h5>🔧 Environment</h5>
                {#if component.environment.length === 0}
                  <span class="no-permissions">No environment access</span>
                {:else}
                  <div class="permission-toggles">
                    {#each component.environment as permission}
                      <label class="permission-toggle">
                        <input 
                          type="checkbox" 
                          checked={true}
                          on:change={() => togglePermission(component.componentId, 'environment', permission, true)}
                        />
                        <span>{permission}</span>
                      </label>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .permissions-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    background: rgba(255, 255, 255, 0.9);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  }

  .permissions-header h2 {
    margin: 0;
    color: #333;
  }

  .global-permissions {
    background: rgba(255, 255, 255, 0.9);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    margin-bottom: 2rem;
  }

  .global-permissions h3 {
    margin: 0 0 1rem 0;
    color: #333;
  }

  .permission-categories {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .permission-category h4 {
    margin: 0 0 0.5rem 0;
    color: #333;
    font-size: 1rem;
  }

  .permission-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .permission-tag {
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: bold;
  }

  .permission-tag.network {
    background: #d1ecf1;
    color: #0c5460;
  }

  .permission-tag.storage {
    background: #d4edda;
    color: #155724;
  }

  .permission-tag.environment {
    background: #fff3cd;
    color: #856404;
  }

  .no-permissions {
    color: #666;
    font-style: italic;
    font-size: 0.9rem;
  }

  .component-permissions {
    background: rgba(255, 255, 255, 0.9);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  }

  .component-permissions h3 {
    margin: 0 0 1.5rem 0;
    color: #333;
  }

  .components-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1rem;
  }

  .component-card {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 8px;
    border: 1px solid #dee2e6;
  }

  .component-card h4 {
    margin: 0 0 0.5rem 0;
    color: #333;
  }

  .component-id {
    font-size: 0.8rem;
    color: #666;
    margin-bottom: 1rem;
  }

  .permission-sections {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .permission-section h5 {
    margin: 0 0 0.5rem 0;
    color: #333;
    font-size: 0.9rem;
  }

  .permission-toggles {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .permission-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8rem;
    cursor: pointer;
  }

  .permission-toggle input[type="checkbox"] {
    margin: 0;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .btn-secondary {
    background: #6c757d;
    color: white;
  }

  .btn-secondary:hover {
    background: #545b62;
  }

  .loading {
    text-align: center;
    padding: 2rem;
    color: white;
    font-size: 1.2rem;
  }

  .empty-state {
    text-align: center;
    padding: 3rem;
    background: #f8f9fa;
    border-radius: 8px;
    border: 1px solid #dee2e6;
  }

  .empty-state p {
    margin: 0 0 0.5rem 0;
    color: #666;
  }

  .empty-state .subtitle {
    font-size: 0.9rem;
    color: #999;
  }
</style>