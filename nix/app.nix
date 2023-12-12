{
  pgks,
  craneLib,
  port,
}: let
  src = craneLib.cleanCargoSource (craneLib.path ../.);
  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;

    buildInputs =
      [
        # Add additional build inputs here
      ]
      ++ lib.optionals pkgs.stdenv.isDarwin [
        # Additional darwin specific inputs can be set here
        pkgs.libiconv
      ];
  };

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Build the actual crate itself, reusing the dependency
  # artifacts from above.
  server = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;
    });

  staticFiles = pkgs.runCommandLocal "copy-static" {src = ./static;} ''
    mkdir -p $out
    cp -r $src $out/static
  '';

  styles =
    pkgs.runCommandLocal "compile-scss" {
      src = ./styles;
      nativeBuildInputs = [pkgs.dart-sass];
    } ''
      mkdir -p $out
      sass $src:$out/static
    '';

  dockerImage = pkgs.dockerTools.buildImage {
    name = "aws";
    tag = "latest";
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [staticFiles server pkgs.coreutils];
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
    inherit server;

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
