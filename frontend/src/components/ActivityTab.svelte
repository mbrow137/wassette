<script>
  import { onMount } from 'svelte';
  
  let events = [];
  let filteredEvents = [];
  let eventTypeFilter = '';
  let loading = false;
  
  onMount(() => {
    loadActivity();
    
    // Listen for real-time updates
    window.addEventListener('realtimeUpdate', handleRealtimeUpdate);
    
    return () => {
      window.removeEventListener('realtimeUpdate', handleRealtimeUpdate);
    };
  });
  
  function handleRealtimeUpdate(event) {
    const data = event.detail;
    // Add new event to the beginning of the list
    events = [data, ...events];
    // Keep only last 50 events
    if (events.length > 50) {
      events = events.slice(0, 50);
    }
    filterActivity();
  }
  
  async function loadActivity() {
    loading = true;
    try {
      const response = await fetch('/api/events');
      const data = await response.json();
      events = data.reverse(); // Show newest first
      filterActivity();
    } catch (error) {
      console.error('Failed to load activity:', error);
    } finally {
      loading = false;
    }
  }
  
  function filterActivity() {
    if (!eventTypeFilter) {
      filteredEvents = events;
    } else {
      filteredEvents = events.filter(event => event.event_type === eventTypeFilter);
    }
  }
  
  function formatTimestamp(timestamp) {
    try {
      const date = new Date(timestamp);
      return date.toLocaleString();
    } catch {
      return timestamp;
    }
  }
  
  function getEventIcon(eventType) {
    switch (eventType) {
      case 'component_load':
      case 'component_loaded':
        return '📦';
      case 'component_unload':
      case 'component_unloaded':
        return '📤';
      case 'permission_change':
        return '🔒';
      case 'error':
        return '❌';
      default:
        return '📋';
    }
  }
  
  function getEventClass(event) {
    if (!event.success) return 'error';
    switch (event.event_type) {
      case 'component_load':
      case 'component_loaded':
        return 'success';
      case 'component_unload':
      case 'component_unloaded':
        return 'warning';
      case 'permission_change':
        return 'info';
      default:
        return 'default';
    }
  }
</script>

<div class="activity-header">
  <h2>Activity Feed</h2>
  <button on:click={loadActivity} class="btn btn-secondary">Refresh</button>
</div>

<div class="activity-filters">
  <select bind:value={eventTypeFilter} on:change={filterActivity}>
    <option value="">All Events</option>
    <option value="component_load">Component Load</option>
    <option value="component_unload">Component Unload</option>
    <option value="permission_change">Permission Changes</option>
    <option value="error">Errors</option>
  </select>
</div>

<div class="activity-feed">
  {#if loading}
    <div class="loading">Loading activity...</div>
  {:else if filteredEvents.length === 0}
    <div class="empty-state">
      <p>No activity events found</p>
      <p class="subtitle">Events will appear here as you interact with components</p>
    </div>
  {:else}
    {#each filteredEvents as event}
      <div class="activity-item {getEventClass(event)}">
        <div class="event-icon">
          {getEventIcon(event.event_type)}
        </div>
        
        <div class="event-content">
          <div class="event-header">
            <span class="event-type">{event.event_type.replace(/_/g, ' ')}</span>
            <span class="event-time">{formatTimestamp(event.timestamp)}</span>
          </div>
          
          <div class="event-description">
            {event.description}
          </div>
          
          {#if event.component_id}
            <div class="event-component">
              Component: <strong>{event.component_id}</strong>
            </div>
          {/if}
          
          {#if event.details}
            <details class="event-details">
              <summary>Details</summary>
              <pre>{JSON.stringify(event.details, null, 2)}</pre>
            </details>
          {/if}
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .activity-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    background: rgba(255, 255, 255, 0.9);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  }

  .activity-header h2 {
    margin: 0;
    color: #333;
  }

  .activity-filters {
    margin-bottom: 1.5rem;
  }

  .activity-filters select {
    padding: 10px;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  }

  .activity-feed {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .activity-item {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    padding: 1.5rem;
    background: rgba(255, 255, 255, 0.95);
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #ccc;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
  }

  .activity-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 25px rgba(0, 0, 0, 0.15);
  }

  .activity-item.success {
    border-left-color: #28a745;
  }

  .activity-item.warning {
    border-left-color: #ffc107;
  }

  .activity-item.error {
    border-left-color: #dc3545;
  }

  .activity-item.info {
    border-left-color: #17a2b8;
  }

  .event-icon {
    font-size: 1.5rem;
    width: 2rem;
    text-align: center;
  }

  .event-content {
    flex: 1;
  }

  .event-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .event-type {
    font-weight: bold;
    color: #333;
    text-transform: capitalize;
  }

  .event-time {
    font-size: 0.9rem;
    color: #666;
  }

  .event-description {
    color: #555;
    margin-bottom: 0.5rem;
  }

  .event-component {
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 0.5rem;
  }

  .event-details {
    margin-top: 0.5rem;
  }

  .event-details summary {
    cursor: pointer;
    font-size: 0.9rem;
    color: #007bff;
  }

  .event-details pre {
    background: #f8f9fa;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    overflow-x: auto;
    margin-top: 0.5rem;
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
    background: rgba(255, 255, 255, 0.9);
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
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