{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ nixpkgs, parts, rust, naersk, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { system, ... }:
        let
          inherit (nixpkgs) lib;
          overlays = [ (import rust) ];
          pkgs = import nixpkgs { inherit system overlays; };
          toolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
            ./rust-toolchain.toml;
          naersk-lib = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          };
          buildDeps = with pkgs; [ pkg-config openssl ];
        in {
          devShells.default = pkgs.mkShellNoCC {
            packages =
              let devTools = with pkgs; [ bacon nil nixfmt-classic taplo ];
              in lib.flatten [ toolchain devTools buildDeps ];
          };
          packages.default = naersk-lib.buildPackage {
            src = ./.;
            nativeBuildInputs = buildDeps;
          };
        };
    };
}
