{
  description = "Basic Rust development flake (per a mi)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };
  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = system: (import nixpkgs { inherit system; });
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlay];
          };
        in
        {
          default = pkgs.callPackage ./shell.nix { inherit pkgs; };
        }
      );
    };
}
