{inputs, ...}: {
  perSystem = {
    system,
    pkgs,
    ...
  }: {
    _module.args.pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [inputs.rust-overlay.overlays.default];
    };
    devShells.default = with pkgs;
      mkShell {
        buildInputs = [
          gdb
          rust-analyzer
          rust-bin.stable.latest.default
        ];
      };
  };
}
