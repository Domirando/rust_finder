pkgs: pkgs.stdenv.mkDerivation{
	name = "devshell";
	nativeBuildInputs = with pkgs; [
		cargo
		clippy
		rustc
	];
	installHook = ''echo hello'';
}
