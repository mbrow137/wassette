<script>
  import { onMount, onDestroy } from 'svelte';
  import ComponentsTab from './components/ComponentsTab.svelte';
  import ActivityTab from './components/ActivityTab.svelte';
  import PermissionsTab from './components/PermissionsTab.svelte';
  import LoadComponentModal from './components/LoadComponentModal.svelte';
  
  let activeTab = 'components';
  let showLoadModal = false;
  let eventSource = null;
  
  // Real-time event handling via SSE
  onMount(() => {
    // Connect to SSE endpoint for real-time updates
    eventSource = new EventSource('/api/events/stream');
    
    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      // Dispatch custom events for components to handle
      window.dispatchEvent(new CustomEvent('realtimeUpdate', { detail: data }));
    };
    
    eventSource.onerror = (error) => {
      console.error('SSE connection error:', error);
    };
  });
  
  onDestroy(() => {
    if (eventSource) {
      eventSource.close();
    }
  });
  
  function showTab(tabName) {
    activeTab = tabName;
  }
  
  function openLoadModal() {
    showLoadModal = true;
  }
  
  function closeLoadModal() {
    showLoadModal = false;
  }
</script>

<main class="container">
  <header>
    <h1>🔧 Wassette Management Interface</h1>
    <p>Manage WebAssembly components and permissions</p>
  </header>

  <nav class="tabs">
    <button 
      class="tab-button {activeTab === 'components' ? 'active' : ''}" 
      on:click={() => showTab('components')}
    >
      Components
    </button>
    <button 
      class="tab-button {activeTab === 'activity' ? 'active' : ''}" 
      on:click={() => showTab('activity')}
    >
      Activity Feed
    </button>
    <button 
      class="tab-button {activeTab === 'permissions' ? 'active' : ''}" 
      on:click={() => showTab('permissions')}
    >
      Permissions
    </button>
  </nav>

  {#if activeTab === 'components'}
    <ComponentsTab on:loadComponent={openLoadModal} />
  {:else if activeTab === 'activity'}
    <ActivityTab />
  {:else if activeTab === 'permissions'}
    <PermissionsTab />
  {/if}
  
  {#if showLoadModal}
    <LoadComponentModal on:close={closeLoadModal} />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
    color: #333;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    min-height: 100vh;
  }

  header {
    text-align: center;
    margin-bottom: 2rem;
    color: white;
  }

  header h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
  }

  header p {
    font-size: 1.1rem;
    opacity: 0.9;
    margin: 0;
  }

  .tabs {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    justify-content: center;
  }

  .tab-button {
    padding: 12px 24px;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    border-radius: 8px;
    color: white;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    backdrop-filter: blur(10px);
  }

  .tab-button:hover {
    background: rgba(255, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .tab-button.active {
    background: rgba(255, 255, 255, 0.9);
    color: #333;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  }
</style>