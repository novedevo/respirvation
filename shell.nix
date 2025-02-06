{pkgs ? import <nixpkgs> {} }:

  pkgs.mkShell {
    LD_LIBRARY_PATH = "ub:/usr/lib/wsl/lib:$LD_LIBRARY_PATH:${
      with pkgs;
      pkgs.lib.makeLibraryPath [
        libxkbcommon 
        libGL
        wayland 
        wayland.dev
      ]
    }";
  }
