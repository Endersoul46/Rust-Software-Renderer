{ pkgs, lib, config, inputs, ... }:
{


  packages = with pkgs; [ 
    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi 
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
    ];
}
