{
  description = "Utilities for the Niri window manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      # Version is read from Cargo.toml. Override here to change it.
      version = cargoToml.package.version;

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      mkPackage =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          pname = "niri-utilities";
          inherit version;

          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset =
              pkgs.lib.fileset.intersection (pkgs.lib.fileset.fromSource (pkgs.lib.sources.cleanSource ./.))
                (
                  pkgs.lib.fileset.unions [
                    ./Cargo.toml
                    ./Cargo.lock
                    ./src
                  ]
                );
          };

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            installShellFiles
            pkg-config
          ];
          buildInputs = with pkgs; [
            wayland
            libxkbcommon
          ];

          postInstall = ''
            installShellCompletion --cmd niri-utilities \
              --bash <($out/bin/niri-utilities completions bash) \
              --zsh <($out/bin/niri-utilities completions zsh) \
              --fish <($out/bin/niri-utilities completions fish)
          '';

          meta = {
            description = "Utilities for the Niri window manager";
            license = pkgs.lib.licenses.mit;
            mainProgram = "niri-utilities";
            platforms = pkgs.lib.platforms.linux;
          };
        };
    in
    flake-utils.lib.eachSystem supportedSystems (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          niri-utilities = mkPackage pkgs;
          default = self.packages.${system}.niri-utilities;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.niri-utilities ];
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
          ];
        };
      }
    )
    // {
      overlays.default = _final: prev: {
        niri-utilities = mkPackage prev;
      };
    };
}
