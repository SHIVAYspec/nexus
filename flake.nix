{
  description = "A rust devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        rust-pkgs = pkgs.rust-bin.stable.latest.default.override {
          targets = [ ];
          extensions = [ "rust-src" ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-pkgs
            rust-analyzer
            protobuf
            protoc-gen-rust
            grpcurl
            grpcui
            pkg-config
            sqlite
            libgit2
            openssl
          ];
        };
      }
    );
}
