{
  perSystem = {pkgs, ...}: let
    inherit (builtins) attrNames filter match readDir;
    inherit (pkgs) lib rustPlatform;
    inherit (lib) mkMerge;
    fs = lib.fileset;

    files = attrNames (readDir ../.);
    isMatch = pattern: name: (match pattern name) != null;
    days = filter (isMatch "day-[0-9]{2}") files;

    mkPackage = name: folder_names: let
      folders = map (name: ../${name}) folder_names;
    in {
      ${name} = rustPlatform.buildRustPackage {
        pname = name;
        version = "0.1.0";

        src = fs.toSource {
          root = ../.;
          fileset = fs.unions (folders
            ++ [
              ../Cargo.toml
              ../Cargo.lock
            ]);
        };

        cargoLock = {
          lockFile = ../Cargo.lock;
          # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
          # allowBuiltinFetchGit = true;
        };

        meta = with lib; {
          description = "${name} of Advent of Code 2024";
          homepage = "https://github.com/lemarsu/advent-of-code-2024";
          license = licenses.mit;
        };
      };
    };
  in {
    packages =
      mkMerge ((map (day: mkPackage day [day]) days)
        ++ [(mkPackage "default" days)]);
  };
}
