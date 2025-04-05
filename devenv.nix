{ pkgs, ... }:

{
  packages = [ pkgs.git pkgs.wrangler ];

  languages = {
    nix.enable = true;
    typescript.enable = true;
    javascript.enable = true;
    shell.enable = true;
  };

  devcontainer.enable = true;
  difftastic.enable = true;

  pre-commit.hooks = {
    shellcheck.enable = true;
    shfmt.enable = true;
    actionlint.enable = true;
    nixpkgs-fmt.enable = true;
    markdownlint.enable = true;
    statix.enable = true;
  };
}
