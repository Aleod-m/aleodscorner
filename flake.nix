{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      fenixPkgs = fenix.packages.${system};
      craneLib = crane.lib.${system}.overrideToolchain fenixPkgs.stable.toolchain;

      # The port on wich the server operates
      port = 8080;

      app = import ./nix/app.nix {inherit pkgs craneLib port;};
    in {
      inherit (app) checks packages;

      formatter = pkgs.alejandra;

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          fenixPkgs.stable.toolchain
          cargo-watch
          docker
          flyctl
          just
          dart-sass
          rust-analyzer
          dive
        ];

        SERVER_PORT = port;
        SERVER_LOCAL = true;
      };
    });
}
