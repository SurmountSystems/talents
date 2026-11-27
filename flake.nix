{
  description = "Talents: BIP32, BIP39, and native Ledger device communication";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/DeterminateSystems/nixpkgs-weekly/0.1";

    crane.url = "https://flakehub.com/f/ipetkov/crane/*";

    rust-overlay = {
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    advisory-db = {
      url = "github:RustSec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      nixpkgs,
      crane,
      rust-overlay,
      advisory-db,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      eachSystem = nixpkgs.lib.genAttrs systems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          inherit (pkgs) lib;

          # Workspace rust-version is 1.100.0. Edition 2024 is stable since 1.85.
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rustfmt"
              "clippy"
            ];
            targets = [ "wasm32-unknown-unknown" ];
          };

          craneLib = (crane.mkLib pkgs).overrideToolchain (_: rustToolchain);

          # bip39 word lists are .txt files pulled in with include_str!.
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./.)
              (lib.fileset.fileFilter (file: file.hasExt "txt") ./crates)
            ];
          };

          linuxHidraw = lib.optionals pkgs.stdenv.hostPlatform.isLinux [
            pkgs.udev
            pkgs.linuxHeaders
          ];

          commonArgs = {
            inherit src;
            strictDeps = true;
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.clang
            ];
            buildInputs =
              linuxHidraw
              ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
                pkgs.libiconv
              ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          wasmArgs = commonArgs // {
            pname = "talents-wasm32-unknown-unknown";
            doCheck = false;
            cargoExtraArgs = "--locked -p talents-core -p talents-bip32 -p talents-bip39 --target wasm32-unknown-unknown";
            # The host cc-wrapper adds glibc headers. wasm32 clang then fails on gnu/stubs-32.h.
            # secp256k1-sys must call the unwrapped compiler. Host build scripts still use the wrapper,
            # so only zerocallusedregs is disabled there. wasm32 clang rejects that flag.
            hardeningDisable = [ "zerocallusedregs" ];
            env.CC_wasm32_unknown_unknown = "${pkgs.clang.cc}/bin/clang";
          };

          wasmArtifacts = craneLib.buildDepsOnly wasmArgs;

          wasmBuild =
            pname: package:
            craneLib.cargoBuild (
              wasmArgs
              // {
                inherit pname;
                cargoArtifacts = wasmArtifacts;
                cargoExtraArgs = "--locked -p ${package} --target wasm32-unknown-unknown";
              }
            );
        in
        {
          formatter = pkgs.nixfmt-rfc-style;

          checks = {
            talents-rustfmt = craneLib.cargoFmt {
              inherit src;
              pname = "talents-rustfmt";
              strictDeps = true;
              cargoExtraArgs = "--all";
            };

            talents-nixfmt =
              pkgs.runCommand "talents-nixfmt" { nativeBuildInputs = [ pkgs.nixfmt-rfc-style ]; }
                ''
                  nixfmt --check ${./flake.nix}
                  mkdir -p "$out"
                '';

            talents-clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "talents-clippy";
                cargoClippyExtraArgs = "--workspace --all-targets --all-features -- -D warnings";
              }
            );

            talents-cargo-test = craneLib.cargoTest (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "talents-cargo-test";
                cargoTestExtraArgs = "--workspace --all-targets";
              }
            );

            talents-cargo-doc-test = craneLib.cargoTest (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "talents-cargo-doc-test";
                cargoTestExtraArgs = "--workspace --doc";
              }
            );

            talents-bip39-no-default-features-test = craneLib.cargoTest (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "talents-bip39-no-default-features-test";
                cargoTestExtraArgs = "-p talents-bip39 --no-default-features";
              }
            );

            talents-core-wasm32-unknown-unknown = wasmBuild "talents-core-wasm32-unknown-unknown" "talents-core";
            talents-bip32-wasm32-unknown-unknown = wasmBuild "talents-bip32-wasm32-unknown-unknown" "talents-bip32";
            talents-bip39-wasm32-unknown-unknown = wasmBuild "talents-bip39-wasm32-unknown-unknown" "talents-bip39";

            talents-bip39-no-default-features-build = craneLib.cargoBuild (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "talents-bip39-no-default-features";
                cargoExtraArgs = "--locked -p talents-bip39 --no-default-features";
              }
            );

            talents-cargo-doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
                pname = "talents-cargo-doc";
                cargoExtraArgs = "--locked --workspace";
                env.RUSTDOCFLAGS = "-D warnings";
              }
            );

            talents-cargo-audit = craneLib.cargoAudit {
              inherit src advisory-db;
              pname = "talents-cargo-audit";
              strictDeps = true;
              # crane's default passes `--ignore yanked`. Leave no ignore list.
              cargoAuditExtraArgs = "";
            };
          };

          devShell = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.cargo-audit
              pkgs.just
              pkgs.pkg-config
              pkgs.clang
              pkgs.gcc
            ]
            ++ linuxHidraw;
          };
        }
      );
    in
    {
      formatter = builtins.mapAttrs (_system: attrs: attrs.formatter) eachSystem;
      checks = builtins.mapAttrs (_system: attrs: attrs.checks) eachSystem;
      devShells = builtins.mapAttrs (_system: attrs: {
        default = attrs.devShell;
      }) eachSystem;
    };
}
