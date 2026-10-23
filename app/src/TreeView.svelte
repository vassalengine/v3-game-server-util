<script module>
	// retain module scoped expansion state for each tree node
	const _expansionState = {}
</script>

<script>
  import TreeView from './TreeView.svelte';

	let {
    tree,
    expandRecursively = false,
    expanded = expandRecursively || _expansionState[tree.label] || false
  } = $props();

  const label = $derived(tree.label);
	const children = $derived(tree.children);

	const toggleExpansion = () => {
		expanded = _expansionState[label] = !expanded;
	}

	let arrowDown = $derived(expanded);

  export function expandAll() {
		expanded = _expansionState[label] = true;
    expandRecursively = true;
  }

  export function collapseAll() {
		expanded = _expansionState[label] = false;
    expandRecursively = false;
  }

</script>

<style>
	ul {
		margin: 0;
		list-style: none;
		padding-left: 1.5rem;
		user-select: none;
	}

	.no-arrow {
    padding-left: 1rem;
  }

  button {
    border: none;
    background: transparent;
  }

  button::before {
    content: '';
    display: inline-block;
    width: 1em;
    height: 0.5em;
    margin-right: 0.5em;
    background-image: url('assets/chevron-down.svg');
    background-size: cover;
  }

	button.arrowDown::before {
    transform: rotate(180deg);
  }
</style>

<ul>
  <li>
  {#if children}
	  <button class:arrowDown onclick={toggleExpansion}>{label}</button>
		{#if expanded}
		  {#each children as child}
			  <TreeView tree={child} expandRecursively={expandRecursively} />
			{/each}
		{/if}
	{:else}
    <span class="no-arrow">
      <svg class="svg-icon"><use xlink:href="#user"></use></svg>
	    <span>{label}</span>
    </span>
  {/if}
  </li>
</ul>
