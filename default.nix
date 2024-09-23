# This is here to make a working enviroment faster and easier to set up in NixOS.  The use of bindgen in rust can be
# tricky.  If you don't use nixos, feel free to skip this.  This project has very few deps and they are easy to set up
# on other OSes
let
  rust_overlay = import (builtins.fetchTarball
    # Head of master
    "https://github.com/oxalica/rust-overlay/archive/36d73192555e569d27579f6c486fea3ab768823c.tar.gz"
  );
  nixpkgs = import
    (builtins.fetchTarball
      # Head of nixos-24.05
      "https://github.com/nixos/nixpkgs/archive/944b2aea7f0a2d7c79f72468106bc5510cbf5101.tar.gz"
    )
    { overlays = [ rust_overlay ]; };
  rust_toolchain = nixpkgs.rust-bin.nightly.latest.default;
in
nixpkgs.mkShell {
  buildInputs = [
    rust_toolchain
    nixpkgs.libclang
  ];
  # Partially taken from here: https://hoverbear.org/blog/rust-bindgen-in-nix/
  shellHook = ''
    export LIBCLANG_PATH=${nixpkgs.libclang.lib}/lib/
    export BINDGEN_EXTRA_CLANG_ARGS="$(< ${nixpkgs.stdenv.cc}/nix-support/libc-crt1-cflags) \
      $(< ${nixpkgs.stdenv.cc}/nix-support/libc-cflags) \
      $(< ${nixpkgs.stdenv.cc}/nix-support/cc-cflags) \
      $(< ${nixpkgs.stdenv.cc}/nix-support/libcxx-cxxflags) \
      ${nixpkgs.lib.optionalString nixpkgs.stdenv.cc.isClang "-idirafter ${nixpkgs.stdenv.cc.cc}/lib/clang/${nixpkgs.lib.getVersion nixpkgs.stdenv.cc.cc}/include"}  \
      ${nixpkgs.lib.optionalString nixpkgs.stdenv.cc.isGNU "-isystem ${nixpkgs.stdenv.cc.cc}/include/c++/${nixpkgs.lib.getVersion nixpkgs.stdenv.cc.cc} -isystem ${nixpkgs.stdenv.cc.cc}/include/c++/${nixpkgs.lib.getVersion nixpkgs.stdenv.cc.cc}/${nixpkgs.stdenv.hostPlatform.config} -idirafter ${nixpkgs.stdenv.cc.cc}/lib/gcc/${nixpkgs.stdenv.hostPlatform.config}/${nixpkgs.lib.getVersion nixpkgs.stdenv.cc.cc}/include"}
    "
  '';
}
