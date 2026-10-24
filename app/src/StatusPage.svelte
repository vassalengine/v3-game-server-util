<script>
  import StatusTree from './StatusTree.svelte';
  import Tabs from './Tabs.svelte';
  import { getConnections } from './load.js';

  const url = '/util/getCurrentConnections';

  const params = new URLSearchParams(window.location.search);
  let when = $state(params.get('when'));

  const items = [
    {
      label: 'Current',
      component: StatusTree,
      props: {
        getTree: () => getConnections(`${url}?when=current`)
      }
    },
    {
      label: 'Day',
      component: StatusTree,
      props: {
        getTree: () => getConnections(`${url}?when=day`)
      }
    },
    {
      label: 'Week',
      component: StatusTree,
      props: {
        getTree: () => getConnections(`${url}?when=week`)
      }
    },
    {
      label: 'Month',
      component: StatusTree,
      props: {
        getTree: () => getConnections(`${url}?when=month`)
      }
    }
  ];

</script>

<svelte:head>
  <title>Vassal Status</title>
</svelte:head>

<style>
</style>

<div class="container mb-5">
  <div class="row justify-content-center">
    <div class="col-md-8">
      <Tabs {items} />
    </div>
  </div>
</div>

<div class="svg-sprites">
  <div class="fontawesome">
    <svg xmlns="http://www.w3.org/2000/svg">
      <symbol id="square-plus" viewBox="0 0 640 640">
        <!--!Font Awesome Free v7.3.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.-->
        <path d="M160 144C151.2 144 144 151.2 144 160L144 480C144 488.8 151.2 496 160 496L480 496C488.8 496 496 488.8 496 480L496 160C496 151.2 488.8 144 480 144L160 144zM96 160C96 124.7 124.7 96 160 96L480 96C515.3 96 544 124.7 544 160L544 480C544 515.3 515.3 544 480 544L160 544C124.7 544 96 515.3 96 480L96 160zM296 408L296 344L232 344C218.7 344 208 333.3 208 320C208 306.7 218.7 296 232 296L296 296L296 232C296 218.7 306.7 208 320 208C333.3 208 344 218.7 344 232L344 296L408 296C421.3 296 432 306.7 432 320C432 333.3 421.3 344 408 344L344 344L344 408C344 421.3 333.3 432 320 432C306.7 432 296 421.3 296 408z"/>
      </symbol>
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg">
      <symbol id="square-minus" viewBox="0 0 640 640">
        <!--!Font Awesome Free v7.3.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.-->
        <path d="M160 144C151.2 144 144 151.2 144 160L144 480C144 488.8 151.2 496 160 496L480 496C488.8 496 496 488.8 496 480L496 160C496 151.2 488.8 144 480 144L160 144zM96 160C96 124.7 124.7 96 160 96L480 96C515.3 96 544 124.7 544 160L544 480C544 515.3 515.3 544 480 544L160 544C124.7 544 96 515.3 96 480L96 160zM232 296L408 296C421.3 296 432 306.7 432 320C432 333.3 421.3 344 408 344L232 344C218.7 344 208 333.3 208 320C208 306.7 218.7 296 232 296z"/>
      </symbol>
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg">
      <symbol id="user" viewBox="0 0 448 512">
        <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
        <path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"/>
      </symbol>
    </svg>
  </div>
</div>
