{
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
	outputs = { self, nixpkgs }:
		let
			systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];
			forAll = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
		in {
			packages = forAll (pkgs: {
				default = let
					manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
				in pkgs.rustPlatform.buildRustPackage {
					pname = manifest.name;
					version = manifest.version;
					src = ./.;
					cargoLock.lockFile = ./Cargo.lock;
					env.SPROUT_SOURCE = "nix";
				};
			});
		};
}
