# shell.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.openssl
    pkgs.pkg-config
  ];

shellHook = ''
    export SHELL=$(which zsh)
    exec zsh
  '';
}

