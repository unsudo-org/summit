{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
    buildInputs = [
        rustc
        cargo
        openssl
        pkg-config
        clang
        llvm
        cmake
        perl
        python3
    ];

	nativeBuildInputs = [
		wasm-bindgen-cli
		dioxus-cli
	];

	shellHook = ''
		export PKG_CONFIG_PATH="${openssl.dev}/lib/pkgconfig"
	'';
}