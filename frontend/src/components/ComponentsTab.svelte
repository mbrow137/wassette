<script>
  import { onMount, createEventDispatcher } from 'svelte';
  
  const dispatch = createEventDispatcher();
  
  let components = [];
  let allComponents = [];
  let searchTerm = '';
  let loading = false;
  
  onMount(() => {
    loadComponents();
    
    // Listen for real-time updates
    window.addEventListener('realtimeUpdate', handleRealtimeUpdate);
    
    return () => {
      window.removeEventListener('realtimeUpdate', handleRealtimeUpdate);
    };
  });
  
  function handleRealtimeUpdate(event) {
    const data = event.detail;
    if (data.type === 'component_loaded' || data.type === 'component_unloaded') {
      loadComponents();
    }
  }
  
  async function loadComponents() {
    loading = true;
    try {
      const response = await fetch('/api/components');
      const data = await response.json();
      allComponents = data;
      filterComponents();
    } catch (error) {
      console.error('Failed to load components:', error);
    } finally {
      loading = false;
    }
  }
  
  function filterComponents() {
    if (!searchTerm) {
      components = allComponents;
    } else {
      components = allComponents.filter(component => 
        component.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        component.id.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
  }
  
  async function unloadComponent(id) {
    if (!confirm(`Are you sure you want to unload component "${id}"?`)) {
      return;
    }
    
    try {
      const response = await fetch(`/api/components/${id}/unload`, {
        method: 'DELETE'
      });
      
      if (response.ok) {
        await loadComponents();
      } else {
        alert('Failed to unload component');
      }
    } catch (error) {
      console.error('Failed to unload component:', error);
      alert('Failed to unload component');
    }
  }
  
  function loadComponent() {
    dispatch('loadComponent');
  }
</script>

<div class="components-header">
  <h2>Components</h2>
  <div class="actions">
    <button on:click={loadComponent} class="btn btn-primary">Load Component</button>
    <button on:click={loadComponents} class="btn btn-secondary">Refresh</button>
  </div>
</div>

<div class="search-bar">
  <input 
    type="text" 
    bind:value={searchTerm}
    on:input={filterComponents}
    placeholder="Search components..." 
  />
</div>

<div class="components-grid">
  {#if loading}
    <div class="loading">Loading components...</div>
  {:else if components.length === 0}
    <div class="empty-state">
      <p>No components loaded</p>
      <button on:click={loadComponent} class="btn btn-primary">Load Your First Component</button>
    </div>
  {:else}
    {#each components as component}
      <div class="component-card">
        <div class="component-header">
          <h3>{component.name}</h3>
          <span class="status-badge {component.enabled ? 'enabled' : 'disabled'}">
            {component.enabled ? 'Enabled' : 'Disabled'}
          </span>
        </div>
        
        <div class="component-info">
          <p><strong>ID:</strong> {component.id}</p>
          <p><strong>Tools:</strong> {component.tool_count}</p>
          {#if component.policy_file}
            <p><strong>Policy:</strong> {component.policy_file}</p>
          {/if}
        </div>
        
        <div class="component-actions">
          <button on:click={() => unloadComponent(component.id)} class="btn btn-danger">
            Unload
          </button>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .components-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    background: rgba(255, 255, 255, 0.9);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  }

  .components-header h2 {
    margin: 0;
    color: #333;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .search-bar {
    margin-bottom: 1.5rem;
  }

  .search-bar input {
    width: 100%;
    padding: 12px;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  }

  .search-bar input:focus {
    outline: none;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  }

  .components-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .component-card {
    background: rgba(255, 255, 255, 0.95);
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s ease, box-shadow 0.3s ease;
  }

  .component-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15);
  }

  .component-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .component-header h3 {
    margin: 0;
    color: #333;
    font-size: 1.2rem;
  }

  .status-badge {
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: bold;
    text-transform: uppercase;
  }

  .status-badge.enabled {
    background: #d4edda;
    color: #155724;
  }

  .status-badge.disabled {
    background: #f8d7da;
    color: #721c24;
  }

  .component-info {
    margin-bottom: 1rem;
  }

  .component-info p {
    margin: 0.5rem 0;
    color: #666;
    font-size: 0.9rem;
  }

  .component-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .btn-primary {
    background: #007bff;
    color: white;
  }

  .btn-primary:hover {
    background: #0056b3;
  }

  .btn-secondary {
    background: #6c757d;
    color: white;
  }

  .btn-secondary:hover {
    background: #545b62;
  }

  .btn-danger {
    background: #dc3545;
    color: white;
  }

  .btn-danger:hover {
    background: #c82333;
  }

  .loading {
    text-align: center;
    padding: 2rem;
    color: white;
    font-size: 1.2rem;
  }

  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: 3rem;
    background: rgba(255, 255, 255, 0.9);
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  }

  .empty-state p {
    margin: 0 0 1rem 0;
    color: #666;
    font-size: 1.1rem;
  }
</style>