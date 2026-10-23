<script>
  let {
    items = [],
    activeTabIndex = $bindable(1),
    toggleExpanded
  } = $props();

  const handleClick = tabIndex => () => (activeTabIndex = tabIndex);
</script>

<style>
	.box {
		margin-bottom: 10px;
		padding: 40px;
		border: 1px solid #dee2e6;
    border-radius: 0 0 .5rem .5rem;
    border-top: 0;
	}

  ul {
    display: flex;
    flex-wrap: wrap;
    padding-left: 0;
    margin-bottom: 0;
    list-style: none;
    border-bottom: 1px solid #dee2e6;
  }

	li {
		margin-bottom: -1px;
	}

  span {
    border: 1px solid transparent;
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
    display: block;
    padding: 0.5rem 1rem;
    cursor: pointer;
  }

  span:hover {
    border-color: #e9ecef #e9ecef #dee2e6;
  }

  li.active > span {
    color: #495057;
    background-color: #fff;
    border-color: #dee2e6 #dee2e6 #fff;
  }

  li.expander {
    margin-left: auto;
    align-self: center;
    margin-right: 0.5em;
    font-size: 1.25em;
  }

  li.expander > button {
    background: transparent;
    border: none;
  }
</style>

<ul>
{#each items as item, i}
	<li class={activeTabIndex === i ? 'active' : ''}>
		<span onclick={handleClick(i)}>{item.label}</span>
	</li>
{/each}
  <li class="expander">
    <button onclick={toggleExpanded}>
      <svg class="svg-icon"><use xlink:href="#square-plus"></use></svg>
    </button>
  </li>
</ul>
{#each items as item, i}
	{#if activeTabIndex === i}
	<div class="box">
		<item.component {...item.props} />
	</div>
	{/if}
{/each}
