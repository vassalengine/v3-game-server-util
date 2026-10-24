export async function getConnections(url) {
  // get the data
  const result = await fetch(url);
  let payload = await result.text();

  // parse the data
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

  // build the tree
  const root = {
    children: [],
    expanded: false
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
        children: player_trees,
        expanded: false
      };

      room_trees.push(room_tree);
    }

    const mod_tree = {
      label: `${module_name} (${room_trees.length})`,
      children: room_trees,
      expanded: false
    };

    root.children.push(mod_tree);
  }

  root.label = `Vassal (${root.children.length})`;

  return root;
}
