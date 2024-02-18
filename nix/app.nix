{
  pkgs,
  nix-filter,
  craneLib,
  port,
}: let
  l = pkgs.lib;
  f = nix-filter.lib;

  src = let 
    templateFilter = path: _type: (__match ".*rs.html$" path) != null;
  in l.cleanSourceWith {
      src = craneLib.path ../.;
      filter = path: type: 
        (templateFilter path type) 
        || (craneLib.filterCargoSources path type);
  };

  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;
    # Add additional build inputs here 
    buildInputs = [];
  };

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  serverBin = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;
    });

  # Todo add a css purification step.
  staticFiles = pkgs.runCommandLocal "static_files" { 
    src = nix-filter {
      root = ../.;
      include = [ 
        "static"
        "styles"
      ];
      exclude = [
        # Exclude styles built for local dev.
        (f.and 
          (f.inDirectory "static")
          (f.matchExt "css")
        )
      ];
    };
    nativeBuildInputs = [pkgs.dart-sass]; 
  } ''
    mkdir -p $out/static
    cp -r $src/static $out/static
    sass --no-source-map \
      $src/styles/globals.scss:$out/static/globals.css \
      $src/styles/pages:$out/static/pages
  '';

  server = pkgs.symlinkJoin {
    name = "server";
    paths = [serverBin staticFiles];
  };

  # The docker image to deploy to fly.
  dockerImage = pkgs.dockerTools.buildImage {
    name = "aleods-corner";
    tag = "latest";
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [server pkgs.coreutils];
    };
    config = {
      Cmd = ["${server}/bin/server"];
      Env = ["SERVER_PORT=${toString port}"];
      ExposedPorts = {
        "${toString port}/tcp" = {};
      };
    };
  };

in {
  packages = {
    inherit dockerImage server staticFiles;
    default = server;
  };

  checks = {
    # Build the crate as part of `nix flake check` for convenience
    inherit serverBin;

    # Run clippy (and deny all warnings) on the crate source,
    # again, resuing the dependency artifacts from above.
    server-clippy = craneLib.cargoClippy (commonArgs
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });

    # Check formatting
    server-fmt = craneLib.cargoFmt {
      inherit src;
    };
  };
}
