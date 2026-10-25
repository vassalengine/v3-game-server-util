<script>
  let {
    items = $bindable(),
    activeTabIndex = $bindable(0)
  } = $props();

  let expanded = $state([]);
  let activeTab = $state();

  let activeItem = $derived(items[activeTabIndex]);

  const handleClick = tabIndex => () => (activeTabIndex = tabIndex);

  function expandAll() {
    activeTab.expandAll();
  }

  function collapseAll() {
    activeTab.collapseAll();
  }
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

  li.tab > button {
    border: 1px solid transparent;
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
    display: block;
    padding: 0.5rem 1rem;
    cursor: pointer;
    background: transparent;
  }

  li.tab > button:hover {
    border-color: #e9ecef #e9ecef #dee2e6;
  }

  li.active > button {
    color: #495057;
    border-color: #dee2e6 #dee2e6 #fff;
  }

  li.expander {
    display: flex;
    margin-left: auto;
    align-self: center;
    margin-right: 0.5em;
    font-size: 1.25em;
  }

  li.expander > button {
    display: block;
    background: transparent;
    border: none;
    padding: 0;
  }
</style>

<ul>
{#each items as item, i}
  <li class="tab {activeTabIndex === i ? 'active' : ''}">
    <button title={item.label} onclick={handleClick(i)}>{item.label}</button>
  </li>
{/each}
  <li class="expander">
    <button title="Collapse all" onclick={collapseAll}>
      <svg class="svg-icon"><use xlink:href="#square-minus"></use></svg>
    </button>
    <button title="Expand all" onclick={expandAll}>
      <svg class="svg-icon"><use xlink:href="#square-plus"></use></svg>
    </button>
  </li>
</ul>
<div class="box">
  <activeItem.component bind:items={items} idx={activeTabIndex} bind:expanded bind:this={activeTab} />
</div>
