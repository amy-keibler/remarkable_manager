{
  description = "Back up the Remarkable tablet and manage custom templates";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.remarkable_manager = naersk-lib.buildPackage {
        pname = "remarkable_manager";
        root = ./.;
        doCheck = true;
      };
      defaultPackage = packages.remarkable_manager;

      # `nix run`
      apps.remarkable_manager = utils.lib.mkApp {
        drv = packages.remarkable_manager;
        exePath = "/bin/remarkable_manager";
      };
      defaultApp = apps.remarkable_manager;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          cargo-edit
          cargo-insta
          cargo-outdated
          clippy
          rustc
          rustfmt
          rust-analyzer

          # hex editor to verify UTF-8 output
          hexyl
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
