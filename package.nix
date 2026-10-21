{ lib, rustPlatform }:

let
	manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
	pname = "sprout"; # Cargo.toml name is different because sprout was taken on crates.io
	version = manifest.version;
	src = lib.cleanSource ./.;
	cargoLock.lockFile = ./Cargo.lock;
	env.SPROUT_SOURCE = "nix";
	meta = {
		description = manifest.description;
		license = lib.licenses.gpl3Only;
		mainProgram = "sprout";
		platforms = lib.platforms.unix;
	};
}
