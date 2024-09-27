{
  inputs = {
    env-nix-pkgs-proxy.url = "github:Aleod-m/env-nix-pkgs-proxy";
    nix-filter.url = "github:numtide/nix-filter";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "env-nix-pkgs-proxy/nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "env-nix-pkgs-proxy/nixpkgs";
    };

  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    flake-utils,
    nix-filter,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      fenixPkgs = fenix.packages.${system};
      craneLib = (crane.mkLib pkgs).overrideToolchain fenixPkgs.stable.toolchain;

      # The port on wich the server operates
      port = 8080;

      app = import ./nix/app.nix {inherit pkgs craneLib port nix-filter;};
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
          elmPackages.elm
          elmPackages.elm-format
          elmPackages.elm-language-server
        ];

        SERVER_PORT = port;
        SERVER_LOCAL = true;
      };
    });
}
