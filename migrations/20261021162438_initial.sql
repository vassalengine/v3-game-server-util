CREATE TABLE IF NOT EXISTS connections (
  sha1 BLOB PRIMARY KEY NOT NULL,
  time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  game_room TEXT NOT NULL,
  module_name TEXT NOT NULL,
  player_name TEXT NOT NULL
);

CREATE INDEX idx_connections_time ON connections(time);

CREATE TABLE IF NOT EXISTS messages (
  module_name TEXT NOT NULL,
  sender TEXT NOT NULL,
  content TEXT NOT NULL,
  time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_messages_time ON messages(time);
CREATE INDEX idx_messages_module_name ON messages(module_name);
