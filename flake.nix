{
  description = "Dropspot development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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

        # Google Cloud to test bucket uploads
        gdk = pkgs.google-cloud-sdk.withExtraComponents( with pkgs.google-cloud-sdk.components; [
          gke-gcloud-auth-plugin
        ]);

        gcloud-login = pkgs.writeShellScriptBin "glogin" ''
          unset GOOGLE_APPLICATION_CREDENTIALS # Remove in case it tries to set service account details
          gcloud auth application-default login
        '';

        rust = with pkgs; (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        rustDeps = with pkgs; [
          rust
          sqlx-cli
          bacon # File watching
          wasm-pack # WebAssembly compilation
        ];

        # Wrapper to use the project version of TypeScript
        tsDeps = with pkgs; [
          typescript
          vtsls
          pnpm
        ];

        editorDeps = with pkgs; [
          neovim
          rustfmt
        ];

        psql = (pkgs.postgresql_17.withPackages (ps: with ps; [ ]));
        deps = with pkgs; [
          psql
          direnv
          gdk
          gcloud-login
          opencode
        ] ++ rustDeps ++ tsDeps ++ editorDeps;
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = deps;

          env = {
            # For later when I add Postgres support
            PGHOST = "/tmp";
          };

          # TODO(alec): How to get RUST_SRC_PATH from the `rust` package we use from rust-overlay
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
