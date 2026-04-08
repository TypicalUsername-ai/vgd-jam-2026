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
          # for Linux
          # Audio (Linux only)
          alsa-lib
          # Cross Platform 3D Graphics API
          vulkan-loader
          # vulkan-intel
          # For debugging around vulkan
          vulkan-tools
          # Other dependencies
          libudev-zero
          libx11
          libxcursor
          libxi
          libxrandr
          libxkbcommon
          wayland
        ];

        naersk' = pkgs.callPackage naersk {};
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            pkg-config
            just
            rust-bin.stable.latest.default
            mold
            clang
          ] ++ lib.optionals (lib.strings.hasInfix "linux" system) linux-deps;

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "clang";
          CARGO_BUILD_RUSTFLAGS = [
            "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold"
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath linux-deps;
          shellHook = ''
          '';
        };

        packages.default  = naersk'.buildPackage {
                  nativeBuildInputs = [
                    pkg-config
                  ] ++ lib.optionals (lib.strings.hasInfix "linux" system) linux-deps;
                  buildInputs = [
                    mold # faster linker instead of gcc - requires linking via clang
                    clang # for linker override in binary Cargo.toml
                  ];
                  src = ./.;
                  CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
                  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "clang";
                  CARGO_BUILD_RUSTFLAGS = [
                    "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold"
                  ];
                  LD_LIBRARY_PATH = lib.makeLibraryPath linux-deps;
                  pname = "insert-pet-hero-tower-offense-game-here";
                  release = false;
                };

      }
    );
}
