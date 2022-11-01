# To update nix-prefetch-git https://github.com/NixOS/nixpkgs
import ((import <nixpkgs> {}).fetchFromGitHub {
  owner = "NixOS";
  repo = "nixpkgs";
  rev = "412b9917cea092f3d39f9cd5dead4effd5bc4053";
  sha256  = "0sbr9knmh3fc0gwwpimgq1p2529lmlakpas72siz5aam1yr73ciq";
})
