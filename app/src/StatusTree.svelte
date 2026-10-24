<script>
  import TreeView from './TreeView.svelte';
  import { getConnections } from './load.js';

  let { url } = $props();

  let tree = $state({});

  getConnections(url)
    .then((root) => tree = root);

  export function expandAll() {
    setExpanded(tree, true);
  }

  export function collapseAll() {
    setExpanded(tree, false);
  }

  function setExpanded(node, expanded) {
    node.expanded = expanded;
    for (const child of node.children ?? []) {
      setExpanded(child, expanded);
    }
  }
</script>

<TreeView bind:tree={tree} />
