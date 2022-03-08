# It would be cool to produce OCI images instead of docker images to
# avoid dependency on docker tool chain. Though the maturity of OCI
# builder in nixpkgs is questionable which is why we postpone this step.
#
# We limit max number of image layers to 42 because there is a bug in
# containerd triggered when there are too many layers:
# https://github.com/containerd/containerd/issues/4684

{ busybox
, dockerTools
, e2fsprogs
, git
, lib
, mayastor
, mayastor-dev
, stdenv
, utillinux
, writeScriptBin
, xfsprogs
}:
let
  versionDrv = import ../../lib/version.nix { inherit lib stdenv git; };
  version = builtins.readFile "${versionDrv}";
  path = lib.makeBinPath [ "/" busybox xfsprogs e2fsprogs utillinux ];

  # common props for all mayastor images
  mayastorImageProps = {
    tag = version;
    created = "now";
    config = {
      Env = [
        "PATH=${path}"
        "RUST_BACKTRACE=1"
      ];
      ExposedPorts = { "10124/tcp" = { }; };
      Entrypoint = [ "/bin/mayastor" ];
    };
    extraCommands = ''
      mkdir tmp
      mkdir -p var/tmp
    '';
  };
  clientImageProps = {
    tag = version;
    created = "now";
    config = {
      Env = [ "PATH=${path}" ];
    };
    extraCommands = ''
      mkdir tmp
      mkdir -p var/tmp
    '';
  };

  mctl = writeScriptBin "mctl" ''
    /bin/mayastor-client "$@"
  '';
in
{
  bolt = dockerTools.buildImage (mayastorImageProps // {
    name = "datacore/bolt";
    contents = [ busybox mayastor mctl ];
  });

  bolt-dev = dockerTools.buildImage (mayastorImageProps // {
    name = "datacore/bolt-dev";
    contents = [ busybox mayastor-dev ];
  });

  bolt-client = dockerTools.buildImage (clientImageProps // {
    name = "datacore/bolt-client";
    contents = [ busybox mayastor ];
    config = { Entrypoint = [ "/bin/mayastor-client" ]; };
  });
}
