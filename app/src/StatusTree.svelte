<script>
  import TreeView from './TreeView.svelte';
  import { getConnections } from './load.js';

  let {
    idx,
    items = $bindable(),
    expanded = $bindable()
  } = $props();

  export function expandAll() {
    setExpanded(items[idx].root, true);
  }

  export function collapseAll() {
    setExpanded(items[idx].root, false);
  }

  function setExpanded(node, exp) {
    expanded[node.id] = exp;
    node.children?.forEach((child) => setExpanded(child, exp));
  }

  async function loadTree() {
    if (!items[idx].root) {
      items[idx].root = await getConnections(items[idx].url, expanded);
    }
  }
</script>

{#await loadTree()}
<div>Loading...</div>
{:then}
<TreeView bind:expanded node={items[idx].root} />
{/await}
