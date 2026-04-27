{
	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	inputs.flake-parts.url = "github:hercules-ci/flake-parts";
	
	outputs = inputs @ { flake-parts, ... }: flake-parts.lib.mkFlake {
		inherit inputs;
	} {
		systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

		perSystem = { pkgs, lib, ... }: 
		let
			build_inputs = [
				pkgs.openssl
				pkgs.glib
				pkgs.gtk3
				pkgs.libsoup_3
				pkgs.webkitgtk_4_1
				pkgs.xdotool
				pkgs.wayland
				pkgs.fmt
			];
		in {
			packages.summit = pkgs.stdenv.mkDerivation {
				pname = "summit";
				version = "0.1.0";
				src = ./.;
				
				nativeBuildInputs = [
					pkgs.pkg-config
					pkgs.cargo
					pkgs.rustc
					pkgs.dioxus-cli
					pkgs.rustPlatform.cargoSetupHook
					pkgs.wasm-bindgen-cli_0_2_106
					pkgs.lld
					pkgs.binaryen
				];
				
				cargoDeps = pkgs.rustPlatform.importCargoLock {
					lockFile = ./Cargo.lock;
					outputHashes."kore-0.1.0" = "sha256-s+w81rZPnoXxjbaSma6MZ5JKvr1dj3WNr1JIjF9e20g=";
				};
				
				buildInputs = build_inputs;
				
				buildPhase = ''
					export HOME=$TMPDIR
					dx build \
						--release \
						--package summit \
						--platform web \
						--features web
				'';
				
				installPhase = ''
					mkdir -p $out/ 
					cp -r target/dx/summit/release/web/. $out/
				'';
			};
			
			devShells.default = pkgs.mkShell {
				nativeBuildInputs = [
					pkgs.nixd
					pkgs.nixpkgs-fmt
					pkgs.clippy
					pkgs.cargo
					pkgs.rustc
					pkgs.rust-analyzer
					pkgs.pkg-config
					pkgs.dioxus-cli
					pkgs.wasm-bindgen-cli_0_2_106
					pkgs.lld
					pkgs.binaryen
				];
				
				buildInputs = build_inputs;
				
				shellHook = ''
					export PATH="$HOME/.cargo/bin:$PATH"
					export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath build_inputs}:$LD_LIBRARY_PATH
					
				'';
			};
		};
	};
}