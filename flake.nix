{
  description = "Flake for Rust development";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay ) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rust-analyzer"
        ];
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rust

            z3.dev
        ];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

        Z3_SYS_Z3_HEADER = "${pkgs.z3.dev}/include/z3.h";
        Z3_SYS_Z3_LIB_DIR = "${pkgs.z3.dev}/lib";
      };
    };
}
