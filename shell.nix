{ 
	pkgs ? import (
		builtins.fetchTarball {
			url = "https://github.com/NixOS/nixpkgs/archive/caece28c8ee20e6b77323a784451262b470b47c8.tar.gz";
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