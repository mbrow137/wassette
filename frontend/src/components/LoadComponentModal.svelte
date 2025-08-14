<script>
  import { createEventDispatcher } from 'svelte';
  
  const dispatch = createEventDispatcher();
  
  let sourceType = 'oci';
  let componentSource = '';
  let componentId = '';
  let loading = false;
  
  function closeModal() {
    dispatch('close');
  }
  
  async function loadComponent() {
    if (!componentSource.trim() || !componentId.trim()) {
      alert('Please provide both component source and ID');
      return;
    }
    
    loading = true;
    try {
      const payload = {
        source: componentSource.trim(),
        source_type: sourceType,
        component_id: componentId.trim()
      };
      
      const response = await fetch(`/api/components/${componentId}/load`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      });
      
      if (response.ok) {
        // Close modal and let parent components refresh
        closeModal();
      } else {
        const error = await response.text();
        alert(`Failed to load component: ${error}`);
      }
    } catch (error) {
      console.error('Failed to load component:', error);
      alert('Failed to load component');
    } finally {
      loading = false;
    }
  }
  
  function handleKeydown(event) {
    if (event.key === 'Escape') {
      closeModal();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-overlay" on:click={closeModal}>
  <div class="modal-content" on:click|stopPropagation>
    <div class="modal-header">
      <h3>Load Component</h3>
      <button class="close-button" on:click={closeModal}>&times;</button>
    </div>
    
    <form on:submit|preventDefault={loadComponent}>
      <div class="form-group">
        <label for="source-type">Source Type:</label>
        <select id="source-type" bind:value={sourceType}>
          <option value="oci">OCI Registry</option>
          <option value="file">Local File</option>
          <option value="url">URL</option>
        </select>
      </div>
      
      <div class="form-group">
        <label for="component-source">
          {#if sourceType === 'oci'}
            OCI Reference (e.g., ghcr.io/owner/component:latest):
          {:else if sourceType === 'file'}
            File Path:
          {:else if sourceType === 'url'}
            URL:
          {/if}
        </label>
        <input 
          id="component-source"
          type="text" 
          bind:value={componentSource}
          placeholder={sourceType === 'oci' ? 'ghcr.io/owner/component:latest' : 
                      sourceType === 'file' ? '/path/to/component.wasm' : 
                      'https://example.com/component.wasm'}
          required
        />
      </div>
      
      <div class="form-group">
        <label for="component-id">Component ID:</label>
        <input 
          id="component-id"
          type="text" 
          bind:value={componentId}
          placeholder="unique-component-id"
          required
        />
        <div class="help-text">
          A unique identifier for this component instance
        </div>
      </div>
      
      <div class="form-actions">
        <button type="button" class="btn btn-secondary" on:click={closeModal}>
          Cancel
        </button>
        <button type="submit" class="btn btn-primary" disabled={loading}>
          {loading ? 'Loading...' : 'Load Component'}
        </button>
      </div>
    </form>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .modal-content {
    background: white;
    border-radius: 12px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
    width: 90%;
    max-width: 500px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #dee2e6;
  }

  .modal-header h3 {
    margin: 0;
    color: #333;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #666;
    padding: 0;
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.3s ease;
  }

  .close-button:hover {
    background: #f8f9fa;
  }

  form {
    padding: 1.5rem;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #333;
  }

  input, select {
    width: 100%;
    padding: 12px;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
    box-sizing: border-box;
  }

  input:focus, select:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
  }

  .help-text {
    font-size: 0.8rem;
    color: #666;
    margin-top: 0.5rem;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 2rem;
  }

  .btn {
    padding: 12px 24px;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #007bff;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #0056b3;
  }

  .btn-secondary {
    background: #6c757d;
    color: white;
  }

  .btn-secondary:hover {
    background: #545b62;
  }
</style>