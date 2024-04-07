{
  inputs,
  lib,
  ...
}: {
  imports = [
    inputs.devshell.flakeModule
  ];

  config.perSystem = {pkgs, ...}: {
    config.devshells.default = {
      imports = [
        "${inputs.devshell}/extra/language/c.nix"
      ];

      commands = with pkgs; [
        {
          package = rust-toolchain;
          category = "rust";
        }
      ];

      language.c = {
        libraries = lib.optional pkgs.stdenv.isDarwin pkgs.libiconv;
      };
    };
  };
}
