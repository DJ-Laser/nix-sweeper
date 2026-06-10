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
  initial = get_initial_game_state;

  update = update_game_state;

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
