{
	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	inputs.flake-parts.url = "github:hercules-ci/flake-parts";
	
	outputs = inputs @ { flake-parts, ... }: flake-parts.lib.mkFlake {
		inherit inputs;
	} {
		systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

		perSystem = { pkgs, ... }: {
			packages.default = pkgs.stdenv.mkDerivation {
				pname = "default";
				version = "0.1.0";
				src = ./.;
				
				nativeBuildInputs = [
					pkgs.pkg-config
					pkgs.cargo
					pkgs.rustc
					pkgs.dioxus-cli
					pkgs.rustPlatform.cargoSetupHook
				];
				
				cargoDeps = pkgs.rustPlatform.importCargoLock {
					lockFile = ./Cargo.lock;
				};
				
				buildInputs = [
					pkgs.openssl
					pkgs.glib
					pkgs.gtk3
					pkgs.libsoup_3
					pkgs.webkitgtk_4_1
					pkgs.xdotool
					pkgs.wayland
					pkgs.fmt
				];
				
				buildPhase = ''
					dx build \
						--release \
						--package summit \
						--platform web \
						--features web
				'';
				
				installPhase = ''
					mkdir -p $out/bin
					cp -r target/dx/summit/release/linux/app/summit $out/bin/summit
				'';
			};
			
			devShells.default = pkgs.mkShell {
				nativeBuildInputs = [
					pkgs.nixd
					pkgs.nixpkgs-fmt
					pkgs.clippy
					pkgs.cargo
					pkgs.rustc
					pkgs.rust-rust-analyzer
					pkgs.pkg-config
					pkgs.dioxus-cli
				];
				
				buildInputs = [
					pkgs.openssl
					pkgs.glib
					pkgs.gtk3
					pkgs.libsoup_3
					pkgs.webkitgtk_4_1
					pkgs.xdotool
					pkgs.wayland
					pkgs.fmt
				];
				
				shellHook = ''
					export PATH="$HOME/.cargo/bin:$PATH"
					export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath build_inputs}:$LD_LIBRARY_PATH
					
				'';
			};
		};
	};
}