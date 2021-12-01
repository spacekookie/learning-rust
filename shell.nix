with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "teaching-rust";
  buildInputs = [
    graphviz
  ];
}
