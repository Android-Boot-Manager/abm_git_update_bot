{ pkgs, ... }:

{
  packages = [ pkgs.git pkgs.wrangler ];

  languages = {
    nix.enable = true;
    rust.enable = true;
    javascript.enable = true;
    shell.enable = true;
  };

  devcontainer.enable = true;
  difftastic.enable = true;

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
    shellcheck.enable = true;
    shfmt.enable = true;
    actionlint.enable = true;
    nixpkgs-fmt.enable = true;
    markdownlint.enable = true;
    statix.enable = true;
  };
  pre-commit.hooks.clippy.settings.offline = false;
}
