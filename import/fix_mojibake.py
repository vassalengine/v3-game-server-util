import contextlib
import sqlite3

import ftfy


def run():

    dbpath = 'util.db'

    with contextlib.closing(sqlite3.connect(dbpath)) as conn:
        with conn as cur:

            rows = cur.execute('SELECT game_room, module_name, player_name FROM connections WHERE length(cast(game_room AS TEXT)) != length(cast(game_room AS BLOB))').fetchall()

            for r in rows:
                fixed = ftfy.fix_encoding(r[0])
                if fixed != r[0]:
                    cur.execute('UPDATE connections SET game_room = ? WHERE game_room = ? AND module_name = ? AND player_name = ?', (fixed, r[0], r[1], r[2]));

                    print(r[0], fixed)

            rows = cur.execute('SELECT game_room, module_name, player_name FROM connections WHERE length(cast(module_name AS TEXT)) != length(cast(module_name AS BLOB))').fetchall()

            for r in rows:
                fixed = ftfy.fix_encoding(r[1])
                if fixed != r[1]:
                    cur.execute('UPDATE connections SET module_name = ? WHERE game_room = ? AND module_name = ? AND player_name = ?', (fixed, r[0], r[1], r[2]));

                    print(r[1], fixed)
            
            rows = cur.execute('SELECT game_room, module_name, player_name FROM connections WHERE length(cast(player_name AS TEXT)) != length(cast(player_name AS BLOB))').fetchall()

            for r in rows:
                fixed = ftfy.fix_encoding(r[2])
                if fixed != r[2]:
                    cur.execute('UPDATE connections SET player_name = ? WHERE game_room = ? AND module_name = ? AND player_name = ?', (fixed, r[0], r[1], r[2]));

                    print(r[2], fixed)


if __name__ == '__main__':
    run()
