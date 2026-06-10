{
  lib,
  rand-nix,
  ...
} @ imports: let
  inherit
    (import ./gameplay/gameplay.nix imports)
    get_initial_game_state
    update_game_state
    get_win_state
    ;

  inherit (import ./output/output.nix imports) board_to_ascii;
in {
  initial = random_seed:
    get_initial_game_state {
      inherit random_seed;
      board_width = 10;
      board_height = 10;
      num_mines = 12;
    };

  update = action: state: (
    if action == "restart"
    then
      get_initial_game_state {
        inherit
          (state)
          board_width
          board_height
          num_mines
          random_seed
          ;
      }
    else update_game_state action state
  );

  output = state: let
    inherit (get_win_state state) game_over game_won;
    board_ascii = board_to_ascii state;

    end_text =
      if game_won
      then ["You Win!" "Press `r` to restart"]
      else ["Boom!" "Game over. Press `r` to restart"];
  in
    if game_over
    then board_ascii + "\n" + lib.concatStringsSep "\n" end_text
    else board_ascii;
}
