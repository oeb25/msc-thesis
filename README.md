# DTU Thesis Pandoc Template

This is a template based on the LaTeX template provided by DTU Compute.

## Requirements

- [🤖 Just](https://github.com/casey/just)
- [🐼 Pandoc](https://pandoc.org/)
- [🪶 `pandoc-secnos`](https://github.com/tomduck/pandoc-secnos)
- [🔍 Skim](https://skim-app.sourceforge.io/)
    - This one is optional, but on macOS it allows for auto reloading of PDF's
      on change. To do so, press `CMD+,` in Skim and goto **Sync** and enable
      **Check for file changes** and **Reload automatically**.

## Usage

The `Justfile` should contain all the common tasks you want to perform. To get
started writing, open an editor, and run:

```bash
just
```

This compiles your thesis once, opens the the generated PDF. Now anytime you
change a `.md` file, the thesis gets recompiled, and the PDF updates
automatically if setup to reload.

## devenv.sh configuration

[devenv.sh] is a neat tool for: "Fast, Declarative, Reproducible, and Composable
Developer Environments". If you want to use it, [install it](https://devenv.sh/getting-started/), run `devenv init`, and edit the generated `devenv.nix` to contain something like the following:

```nix
{ pkgs, ... }:

{
  starship.enable = true;

  packages = [ pkgs.git pkgs.just pkgs.pandoc pkgs.python310Packages.pandoc-xnos ];

  enterShell = '''';

  # https://devenv.sh/languages/
  languages.python.enable = true;
  languages.python.venv.enable = true;
}
```
