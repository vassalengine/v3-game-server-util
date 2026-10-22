<script>
  import TreeView from './TreeView.svelte';

  let { url } = $props();

  let tree = $state({});

  async function getConnections() {
    try {
      const result = await fetch(url);
      let payload = await result.text();
    
      const tr = {};

      for (const line of payload.split('\n')) {
        const [module_name, game_room, player_name] = line.split('\t', 3);

        let mod = tr[module_name];
        if (mod === undefined) {
          tr[module_name] = mod = {};
        }

        let room = mod[game_room];
        if (room === undefined) {
          mod[game_room] = room = [];
        }

        room.push(player_name);
      }

      const root = {
        children: []
      };

      for (const [module_name, rooms] of Object.entries(tr)) {
        const room_trees = [];

        for (const [room_name, players] of Object.entries(rooms)) {
          const player_trees = [];

          for (const player_name of players) {
            player_trees.push({ label: player_name })
          }

          const room_tree = {
            label: `${room_name} (${players.length})`,
            children: player_trees
          };

          room_trees.push(room_tree);
        }

        const mod_tree = {
          label: `${module_name} (${room_trees.length})`,
          children: room_trees
        };

        root.children.push(mod_tree);
      }
      
      root.label = `Vassal (${root.children.length})`;

      tree = root;
    }
    catch (err) {
      console.log(err);
    }
  }

  getConnections();

</script>

<TreeView {tree} startsExpanded=true />
