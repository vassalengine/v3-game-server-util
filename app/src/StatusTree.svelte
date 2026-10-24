<script>
  import { onMount } from 'svelte';
  import TreeView from './TreeView.svelte';

  let { getTree } = $props();

  let tree = $state({});

  onMount(() => {
    getTree()
      .then((root) => {
          root.expanded = true;
          tree = root;
      });
  });

  export function expandAll() {
    setExpanded(tree, true);
  }

  export function collapseAll() {
    setExpanded(tree, false);
  }

  function setExpanded(node, expanded) {
    node.expanded = expanded;
    node.children?.forEach((child) => setExpanded(child, expanded));
  }
</script>

<TreeView bind:tree={tree} />
