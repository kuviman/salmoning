{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nixpkgs-stable.url = "nixpkgs/release-23.11";
    rust-overlay.url = "github:Oxalica/rust-overlay";
    geng.url = "github:cgsdev0/cargo-geng";
    geng.inputs.rust-overlay.follows = "rust-overlay";
    geng.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { geng, nixpkgs, ... }@inputs: geng.makeFlakeOutputs (system:
    let
      pkgs = import nixpkgs { inherit system; };
      pkgs-stable = import inputs.nixpkgs-stable { inherit system; };
    in
    {
      src = ./.;
      extraBuildInputs = [ pkgs.nodejs pkgs-stable.butler ];
    });
}
