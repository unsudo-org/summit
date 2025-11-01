{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];

  shellHook = ''
    export PATH=$HOME/.cargo/bin:$PATH
    REQUIRED_VERSION="0.2.104"
    if ! command -v wasm-bindgen >/dev/null 2>&1 || [[ "$(wasm-bindgen --version)" != *"$REQUIRED_VERSION"* ]]; then
      echo "Installing wasm-bindgen-cli $REQUIRED_VERSION via cargo..."
      cargo install -f wasm-bindgen-cli --version $REQUIRED_VERSION
    fi
  '';
}