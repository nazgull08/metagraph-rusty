with import ./nix/pkgs.nix {};
let project = "metagraph";
in stdenv.mkDerivation rec {
  name = "metagraph-rusty";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    rustup
  ];
  shellHook = ''
  '';
}
