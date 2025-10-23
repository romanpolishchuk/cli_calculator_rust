{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    rustfmt

    #VS codium with extensions
    (vscode-with-extensions.override {
      vscode = vscodium;
      vscodeExtensions = with vscode-extensions; [
        rust-lang.rust-analyzer
        vadimcn.vscode-lldb
      ];
    })
  ];

  shellHook = ''
    echo "Dev environment"

    alias run='cargo run'
  '';
}
