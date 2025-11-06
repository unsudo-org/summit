{ 
	pkgs ? import (
		builtins.fetchTarball {
			url = "https://github.com/NixOS/nixpkgs/archive/88683646458dfafc95051c09f62c5b97d4249fcc.tar.gz";
		}
	) {}
}:

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

	# after launching `nix-shell`
	# cargo install wasm-bindgen-cli

	shellHook = ''
		export PATH="$HOME/.cargo/bin:$PATH"
	'';

	RUST_BACKTRACE = 0;
	PKG_CONFIG_PATH = "PKG_CONFIG_PATH=${openssl.dev}/lib/pkgconfig";
}