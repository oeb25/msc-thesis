{ pkgs, ... }:

{
  starship.enable = true;

  packages = [ pkgs.git pkgs.just pkgs.pandoc pkgs.python310Packages.pandoc-xnos ];

  enterShell = '''';

  # https://devenv.sh/languages/
  languages.python.enable = true;
  languages.python.venv.enable = true;
}
