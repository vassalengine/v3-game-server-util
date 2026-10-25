<script>
  import TreeView from './TreeView.svelte';
  import { getConnections } from './load.js';

  let {
    item = $bindable(),
    expanded = $bindable()
  } = $props();

  export function expandAll() {
    setExpanded(item.root, true);
  }

  export function collapseAll() {
    setExpanded(item.root, false);
  }

  function setExpanded(node, exp) {
    expanded[node.id] = exp;
    node.children?.forEach((child) => setExpanded(child, exp));
  }

  async function loadTree() {
    if (!item.root) {
      item.root = await getConnections(item.url, expanded);
    }
  }
</script>

{#await loadTree()}
<div>Loading...</div>
{:then}
  {#if item.root}
<TreeView bind:expanded node={item.root} />
  {/if}
{:catch error}
<div>Failed to load server status: {error.message}</div>
{/await}
