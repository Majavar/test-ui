let
  pkgs = import <nixpkgs> {};
in with pkgs; stdenv.mkDerivation rec {
  name = "rust env";

  # Allow cargo to download crates.
  SSL_CERT_FILE = "/etc/ssl/certs/ca-bundle.crt";
  GIT_SSL_CAINFO = "/etc/ssl/certs/ca-bundle.crt";

  buildInputs = [
    cmake
    fontconfig
    freetype
    git
    jetbrains.idea-community
    pkg-config
    rustup
    libxkbcommon
    libGL
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}

