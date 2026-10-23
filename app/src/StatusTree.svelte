<script>
  import TreeView from './TreeView.svelte';
  import { getConnections } from './load.js';

  let { url } = $props();

  let tree = $state({});

  getConnections(url)
    .then((root) => tree = root);


  }

</script>

<TreeView {tree} startsExpanded=true />
