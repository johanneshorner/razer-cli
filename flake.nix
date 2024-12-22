{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
      in
      with pkgs;
      {
        devShell = pkgs.mkShell {
          packages = [
            pkg-config
            systemd
          ];
          LD_LIBRARY_PATH = "${systemd}/lib";
        };
        packages.default = callPackage ./nix/package.nix { };
      }
    );
}
