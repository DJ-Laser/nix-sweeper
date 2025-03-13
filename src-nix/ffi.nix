{
  lib,
  rand-nix,
}: {
  initial = {
    board_width,
    board_height,
    num_mines,
    random_seed ? 0,
  } @ config: let
    rng = rand-nix.rng.withSeed (toString random_seed);

    place_mine = board: rng: let
      mine_x = rng.intBetween 1 board_width - 1;
      mine_y = rng.next.intBetween 1 board_height - 1;

      board' = lib.imap0 (y: row:
        if y == mine_y
        then
          lib.imap0 (x: tile:
            if x == mine_x
            then tile // {mine = true;}
            else tile)
          row
        else row)
      board;
    in {
      board = board';
      rng = rng.skip 2;
    };

    fill_board = n: board: rng: let
      res = place_mine board rng;
    in
      if n == 0
      then board
      else fill_board (n - 1) res.board res.rng;

    number_tile = board: y: x: let
      neighbor = x_offset: y_offset: {
        x = x + x_offset;
        y = y + y_offset;
      };

      neighbors = [
        (neighbor (-1) 1)
        (neighbor 0 1)
        (neighbor 1 1)
        (neighbor (-1) 0)
        (neighbor 1 0)
        (neighbor (-1) (-1))
        (neighbor 0 (-1))
        (neighbor 1 (-1))
      ];

      valid_neighbors = lib.filter (neighbor:
        true
        && neighbor.x >= 0
        && neighbor.x < board_width
        && neighbor.y >= 0
        && neighbor.y < board_height)
      neighbors;

      mine_neighbors = lib.filter (neighbor: (lib.elemAt (lib.elemAt board neighbor.y) neighbor.x).mine) valid_neighbors;
    in
      lib.length mine_neighbors;

    assign_numbers = board:
      lib.imap0 (y: row:
        lib.imap0 (x: tile:
          tile // {number = number_tile board y x;})
        row)
      board;

    empty_board = lib.replicate board_height (lib.replicate board_width {
      revealed = false;
      flagged = false;
      mine = false;
      number = 0;
    });
    filled_board = fill_board num_mines empty_board rng;
    numbered_board = assign_numbers filled_board;
  in {
    inherit (config) board_width board_height;

    cursor_x = 0;
    cursor_y = 0;

    board = numbered_board;
  };

  output = {
    board,
    board_width,
    board_height,
    cursor_x,
    cursor_y,
    ...
  } @ state: let
    tile_to_ascii = y: x: tile: let
      selected = x == cursor_x && y == cursor_y;
      revealed_icon =
        if tile.mine
        then "*"
        else if tile.number == 0
        then " "
        else toString tile.number;

      tile_icon =
        if tile.revealed
        then revealed_icon
        else if tile.flagged
        then "◆"
        else "-";
    in
      if selected
      then "<${tile_icon}>"
      else " ${tile_icon} ";

    row_to_ascii = y: row: "│${lib.concatStrings (lib.imap0 (tile_to_ascii y) row)}│";
    rows_ascii = lib.concatStringsSep "\n" (lib.imap0 row_to_ascii board);
    line_ascii = lib.concatStrings (lib.replicate (3 * board_width) "─");

    board_ascii = ''
      ┌${line_ascii}┐
      ${rows_ascii}
      └${line_ascii}┘
    '';
  in
    board_ascii;
}
