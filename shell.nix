#nix-shell -p clang rocmPackages_5.llvm.lld nodejs_22 python3
{ pkgs ? import <nixpkgs> {
	#config.allowUnfree = true;
} }:

let
in 
pkgs.stdenv.mkDerivation rec {
    name = "factor";
    buildInputs = with pkgs; [
      factor-lang
	];
	shellHook = ''
		alias factor="${pkgs.factor-lang}/bin/factor"
	'';
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
