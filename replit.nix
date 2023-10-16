{ pkgs }: {
	deps = [
    pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
		pkgs.openssl # for hf-hub model loading
		pkgs.pkg-config
		pkgs.sqlite # for sqlx-sqlite example
	];
}