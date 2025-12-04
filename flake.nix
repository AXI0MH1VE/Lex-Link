# AXIOM HIVE - Nix Flake
# Reproducible builds for deterministic verification
#
# [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

{
  description = "AXIOM HIVE / LEX-Ω Production System";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "x86_64-apple-darwin" "aarch64-apple-darwin" ];
        };
        
        pythonEnv = pkgs.python311.withPackages (ps: with ps; [
          cryptography
          pytest
          pytest-cov
          hypothesis
          mypy
        ]);
      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-audit
            
            # Python
            pythonEnv
            
            # Build tools
            gnumake
            cmake
            pkg-config
            
            # Crypto
            openssl
            
            # Code quality
            pre-commit
            shellcheck
            
            # Documentation
            mdbook
            
            # SBOM
            syft
            cosign
          ];
          
          shellHook = ''
            echo "[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]"
            echo "Development environment ready"
            echo "Rust: $(rustc --version)"
            echo "Python: $(python --version)"
          '';
          
          RUST_BACKTRACE = "1";
          AXIOM_DEV_MODE = "1";
        };

        # Packages
        packages = {
          # SAP-4D Proof Engine
          sap4d = pkgs.rustPlatform.buildRustPackage {
            pname = "sap4d";
            version = "1.0.0";
            src = ./sap4d;
            cargoLock.lockFile = ./sap4d/Cargo.lock;
            
            meta = {
              description = "SAP-4D Proof Engine";
              homepage = "https://axiomhive.local";
              license = pkgs.lib.licenses.unfree;
            };
          };
          
          # Audit Service
          audit = pkgs.rustPlatform.buildRustPackage {
            pname = "axiom-audit";
            version = "1.0.0";
            src = ./audit;
            cargoLock.lockFile = ./audit/Cargo.lock;
            
            meta = {
              description = "Deterministic Fractal Audit Service";
              homepage = "https://axiomhive.local";
            };
          };
          
          # Verification Portal
          portal = pkgs.rustPlatform.buildRustPackage {
            pname = "axiom-portal";
            version = "1.0.0";
            src = ./portal;
            cargoLock.lockFile = ./portal/Cargo.lock;
            
            meta = {
              description = "AXIOM HIVE Verification Portal";
              homepage = "https://axiomhive.local";
            };
          };
          
          # Hunter-Killer
          hunter-killer = pkgs.rustPlatform.buildRustPackage {
            pname = "hunter-killer";
            version = "1.0.0";
            src = ./tools/hunter_killer;
            cargoLock.lockFile = ./tools/hunter_killer/Cargo.lock;
            
            meta = {
              description = "Prompt injection detection tool";
              homepage = "https://axiomhive.local";
            };
          };
          
          # Invariance Python library
          invariance = pkgs.python311Packages.buildPythonPackage {
            pname = "axiomhive-invariance";
            version = "1.0.0";
            src = ./invariance;
            format = "pyproject";
            
            propagatedBuildInputs = with pkgs.python311Packages; [
              cryptography
            ];
            
            meta = {
              description = "AXIOM HIVE Invariance Layer";
              homepage = "https://axiomhive.local";
            };
          };
          
          # All packages
          all = pkgs.symlinkJoin {
            name = "axiomhive-all";
            paths = with self.packages.${system}; [
              sap4d
              audit
              portal
              hunter-killer
              invariance
            ];
          };
        };

        # Default package
        defaultPackage = self.packages.${system}.all;

        # Apps
        apps = {
          portal = flake-utils.lib.mkApp {
            drv = self.packages.${system}.portal;
          };
          audit = flake-utils.lib.mkApp {
            drv = self.packages.${system}.audit;
          };
          sap4d = flake-utils.lib.mkApp {
            drv = self.packages.${system}.sap4d;
          };
        };

        # Checks (run with `nix flake check`)
        checks = {
          format = pkgs.runCommand "format-check" {
            buildInputs = [ rustToolchain pkgs.rustfmt ];
          } ''
            cd ${./.}
            cargo fmt --all -- --check
            touch $out
          '';
        };
      }
    );
}

