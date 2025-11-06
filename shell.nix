{ pkgs ? import (builtins.fetchTarball {
	url = "https://github.com/NixOS/nixpkgs/archive/refs/heads/nixpkgs-unstable.tar.gz";
}) {} }:

pkgs.mkShell {
	buildInputs = [
		pkgs.rustc
		pkgs.cargo
		pkgs.openssl
		pkgs.pkg-config
		pkgs.clang
		pkgs.llvm
		pkgs.cmake
		pkgs.perl
		pkgs.python3
	];
	
	nativeBuildInputs = [
		pkgs.wasm-bindgen-cli
		pkgs.dioxus-cli
	];

	RUST_BACKTRACE = 1;
	PKG_CONFIG_PATH = "PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig";
}