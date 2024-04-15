{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    geng.url = "github:geng-engine/cargo-geng";
    geng.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { geng, nixpkgs, ... }: geng.makeFlakeOutputs (system:
    let pkgs = import nixpkgs { inherit system; }; in
    {
      src = ./.;
      extraBuildInputs = [ pkgs.nodejs ];
    });
}
