<script>
  import TreeView from './TreeView.svelte';

  let {
    node,
    expanded = $bindable()
  } = $props();

  const toggleExpansion = () => { expanded[node.id] = !expanded[node.id]; }

  let arrowDown = $derived(expanded[node.id]);
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
  {#if node.children}
    <button class:arrowDown onclick={toggleExpansion}>{node.label}</button>
    {#if expanded[node.id]}
      {#each node.children as child, i (child.label)}
        <TreeView bind:expanded node={child} />
      {/each}
    {/if}
  {:else}
    <span class="no-arrow">
      <svg class="svg-icon"><use xlink:href="#user"></use></svg>
      <span>{node.label}</span>
    </span>
  {/if}
  </li>
</ul>
