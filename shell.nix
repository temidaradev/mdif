{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    clang
    lld
    pkg-config
  ];

  RUSTFLAGS = "-C link-arg=-fuse-ld=lld";
}

