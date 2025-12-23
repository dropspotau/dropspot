{
  description = "Dropspot development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
            permittedInsecurePackages = [];
          };
        };

        rustDeps = with pkgs; [
          rustup
          rust-analyzer
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          sqlx-cli
        ];

        editorDeps = with pkgs; [
          neovim
          rustfmt
        ];

        psql = (pkgs.postgresql_17.withPackages (ps: with ps; [ ]));
        deps = with pkgs; [
          psql
          direnv
        ] ++ rustDeps ++ editorDeps;
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = deps;

          env = {
            # For later when I add Postgres support
            PGHOST = "/tmp";
          };
        };
      }
    );
}
