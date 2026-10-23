<script>
  import TreeView from './TreeView.svelte';
  import { getConnections } from './load.js';

  let { url } = $props();

  let tree = $state({});

  getConnections(url)
    .then((root) => tree = root);

  let view;

  export function expandAll() {
    view?.expandAll();
  }

  export function collapseAll() {
    view?.collapseAll();
  }
</script>

<TreeView {tree} expanded=true bind:this={view} />
