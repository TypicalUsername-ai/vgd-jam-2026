{
  description = "Flake for Video Game Design project @ ISTAN WUST 26";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url  = "github:numtide/flake-utils";
    };
    naersk = {
          url = "github:nix-community/naersk";
          inputs.nixpkgs.follows = "nixpkgs";
        };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        linux-deps = with pkgs; [
          wayland # bevy required for display
          alsa-lib # bevy required for sound
          udev # bevy: devices support ??
          pkg-config
        ];
        naersk' = pkgs.callPackage naersk {};
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            just
            rust-bin.stable.latest.default
          ];

          shellHook = ''
          '';
        };

        packages.default  = naersk'.buildPackage {
                  nativeBuildInputs = linux-deps;
                  src = ./.;
                  pname = "insert-pet-hero-tower-offense-game-here";
                };

      }
    );
}
