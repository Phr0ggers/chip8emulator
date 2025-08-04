{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
  ];
  shellHook = ''
    echo 'nix-shell: Rust'
  '';
}
