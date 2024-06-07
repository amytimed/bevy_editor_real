{
  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.url = "nixpkgs";
    };
  };

  outputs = {
    self,
    systems,
    nixpkgs,
    treefmt-nix,
    ...
  } @ inputs: let
    eachSystem = f:
      nixpkgs.lib.genAttrs (import systems) (
        system:
          f (import nixpkgs {
            inherit system;
            overlays = [inputs.rust-overlay.overlays.default];
          })
      );

    rustToolchain = eachSystem (pkgs: pkgs.rust-bin.stable.latest);

    treefmtEval = eachSystem (pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix);
  in {
    devShells = eachSystem (pkgs: {
      # Based on a discussion at https://github.com/oxalica/rust-overlay/issues/129
      default = pkgs.mkShell.override {
  stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
} {
        /* nativeBuildInputs = [
          pkg-config
          llvmPackages.libclang
          llvmPackages.libcxxClang
          clang
          # Use mold when we are runnning in Linux
          (lib.optionals stdenv.isLinux mold)
        ];
        buildInputs = [
          udev alsa-lib vulkan-loader
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
          libxkbcommon wayland # To use the wayland feature
          rustToolchain.${pkgs.system}.default
          rust-analyzer-unwrapped
          cargo
          # pkg-config
          # openssl
        ]; */

        RUST_SRC_PATH = "${
          rustToolchain.${pkgs.system}.rust-src
        }/lib/rustlib/src/rust/library";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.alsaLib
            pkgs.udev
            pkgs.vulkan-loader
        ];
        #LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        #BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include";

        packages = with pkgs; [
          pkg-config
          llvmPackages.libclang
          llvmPackages.libcxxClang
          udev alsa-lib vulkan-loader
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
          libxkbcommon wayland # To use the wayland feature
          rustToolchain.${pkgs.system}.default
          rust-analyzer-unwrapped
          cargo
        ];
      };
    });

    formatter = eachSystem (pkgs: treefmtEval.${pkgs.system}.config.build.wrapper);

    checks = eachSystem (pkgs: {
      formatting = treefmtEval.${pkgs.system}.config.build.check self;
    });
  };
}
