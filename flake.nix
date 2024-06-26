{
  description = "Rust development environment for OpenWeatherCLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    overlays = [
      rust-overlay.overlays.default
      (final: prev: {
        rustToolchain = let
          rust = prev.rust-bin;
        in
          if builtins.pathExists ./rust-toolchain.toml
          then rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain
          then rust.fromRustupToolchainFile ./rust-toolchain
          else rust.stable.latest.default;
      })
    ];
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit overlays system;};
        });
  in {
    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer
        ];

        shellHook = ''
          exec zsh
        '';
      };
    });

    packages = forEachSupportedSystem ({pkgs}: {
      default = let
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rustToolchain;
          rustc = pkgs.rustToolchain;
        };
      in
        rustPlatform.buildRustPackage rec {
          name = "openweathercli";
          pname = "openweathercli";
          version = "0.2.0-alpha";

          src = pkgs.fetchFromGitHub {
            owner = "Kodlak15";
            repo = "openweathercli";
            rev = version;
            hash = "sha256-Jz7XOe/YCt9KYVOe/wx46Kyw/fPHa/BLUaa2TCBh6NI=";
          };
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [pkg-config];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
    });
  };
}
