{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    rand-nix = {
      url = "github:figsoda/rand-nix";
      follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    rand-nix,
  }: {
    ffi = {
      initial = {
        cursor_x = 0;
        cursor_y = 0;

        board_width = 15;
        board_height = 15;
      };

      update = x: {e = x + 2;};
    };
  };
}
