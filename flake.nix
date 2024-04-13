{
  description = "Von Koch snowflake generator for amatgil.cat";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let system = "x86_64-linux";
    in {
      packages.default = nixpkgs.callPackage ./default.nix { pkgs = nixpkgs; };
      devShell.${system}.default = nixpkgs.callPackage ./shell.nix { pkgs = nixpkgs; };
    };
}
