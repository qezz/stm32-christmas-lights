let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust-nightly = (nixpkgs.latest.rustChannels.nightly.rust.override {
    extensions = [
      "rust-src"
      "rls-preview"
      "rust-analysis"
      "rustfmt-preview"
    ];
    targets = [ "thumbv7m-none-eabi" ];
  });
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
      rust-nightly
      rust-analyzer
      pkgconfig
      stlink
    ];
  }
