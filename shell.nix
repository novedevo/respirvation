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
    # buildInputs = with pkgs.buildPackages; [ 
    #     wayland wayland-protocols 
    #     wayland.dev

    #     xorg.libX11 
    #     xorg.libXcursor 
    #     xorg.libXi 
    #     libxkbcommon 
    #     xorg.libxcb  
    #     alsa-lib 
    #     libudev-zero 
    #     openssl 
    #     llvm 
    #     pkg-config 
    #     gcc
    #     glxinfo
    # ];

    # LD_LIBRARY_PATH = "ub:/usr/lib/wsl/lib:$LD_LIBRARY_PATH:${
    #           with pkgs;
    #           pkgs.lib.makeLibraryPath [
    #             # xorg.libX11 
    #             # xorg.libXcursor 
    #             # xorg.libXi 
    #             libxkbcommon 
    #             # xorg.libxcb  
    #             # pkgs.vulkan-loader
    #             # pkgs.glfw

    #             libGL

    #     wayland 
    #     # wayland-protocols 
    #     wayland.dev
    #           ]
    #         }";

