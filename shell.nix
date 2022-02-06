with import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; };
mkShell {
  RUSTFLAGS = "";
  buildInputs = [
    (rust-bin.selectLatestNightlyWith (toolchain:
      toolchain.default.override {
        extensions = [ "rust-src" "rustfmt-preview" ];
      }))
    pkg-config
    openssl
    clang_12
  ] ++ lib.optionals stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
  ];
}
