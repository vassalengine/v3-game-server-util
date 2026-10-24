<script>
  import TreeView from './TreeView.svelte';

	let {
    tree = $bindable()
  } = $props();

	const toggleExpansion = () => { tree.expanded = !tree.expanded; }

	let arrowDown = $derived(tree.expanded);
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
    text-align: start;
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
  {#if tree.children}
	  <button class:arrowDown onclick={toggleExpansion}>{tree.label}</button>
		{#if tree.expanded}
		  {#each tree.children as child, i (child.label)}
			  <TreeView bind:tree={tree.children[i]} />
			{/each}
		{/if}
	{:else}
    <span class="no-arrow">
      <svg class="svg-icon"><use xlink:href="#user"></use></svg>
	    <span>{tree.label}</span>
    </span>
  {/if}
  </li>
</ul>
