{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    crate2nix,
  }: let
    system = "x86_64-linux";
    overlays = [
      fenix.overlays.default
    ];

    pkgs = import nixpkgs {inherit system overlays;};

    rustToolchain = with pkgs.fenix;
      combine [
        stable.cargo
        stable.rustc
        stable.rustfmt
        stable.rust-src
      ];

    crate2nixPkgs =
      pkgs.extend
      (final: prev: let
      in {
        rustc = rustToolchain;
        cargo = rustToolchain;
      });

    crate2nix' = crate2nixPkgs.callPackage (import "${crate2nix}/tools.nix") {};
    cargoNix = crate2nix'.appliedCargoNix {
      name = "my-crate";
      src = ./.;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [alejandra rustToolchain];
      RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
    };

    packages.${system}.default = cargoNix.rootCrate.build;
  };
}
