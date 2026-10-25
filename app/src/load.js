export async function getConnections(url, expanded) {
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

  let next_id = expanded.length;

  // build the tree
  const root = {
    id: next_id, 
    children: [],
  };

  for (const [module_name, rooms] of Object.entries(tr)) {
    const room_trees = [];

    for (const [room_name, players] of Object.entries(rooms)) {
      const player_trees = [];

      for (const player_name of players) {
        player_trees.push({ label: player_name })
      }

      const room_tree = {
        id: ++next_id,
        label: `${room_name} (${players.length})`,
        children: player_trees,
      };

      room_trees.push(room_tree);
    }

    const mod_tree = {
      id: ++next_id,
      label: `${module_name} (${room_trees.length})`,
      children: room_trees,
    };

    root.children.push(mod_tree);
  }

  root.label = `Vassal (${root.children.length})`;

  expanded.push(...Array(next_id - expanded.length).fill(false));
  expanded[root.id] = true;

  return root;
}
