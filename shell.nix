{ pkgs ? import <nixpkgs> {} }:
let
	pin = import (builtins.fetchTarball {
		url = "https://github.com/NixOS/nixpkgs/archive/3609928eec5894d5a483a14f81346f53480619f9.tar.gz";
	}) {};
in
	pkgs.mkShell {
		buildInputs = [
			pin.rustc
			pin.cargo
			pin.openssl
			pin.pkg-config
			pin.clang
			pin.llvm
			pin.cmake
			pin.perl
			pin.python3
		];

		nativeBuildInputs = [
			pin.wasm-bindgen-cli
			pin.dioxus-cli
		];

		shellHook = ''
			export PKG_CONFIG_PATH="${pin.openssl.dev}/lib/pkgconfig"
		'';
	}