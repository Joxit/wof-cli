# wof

[![Rust](https://github.com/Joxit/wof-cli/workflows/Rust/badge.svg)](https://github.com/Joxit/wof-cli/actions?query=workflow%3ARust)
[![Crates.io version shield](https://img.shields.io/crates/v/wof.svg)](https://crates.io/crates/wof)
[![Crates.io license shield](https://img.shields.io/crates/l/wof.svg)](https://crates.io/crates/wof)

This project is both a CLI and a library to work with [Who's On First](https://www.whosonfirst.org/) documents in Rust.

If you want the CLI, install it with [cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install wof --force --features cli
```

## CLI

Where is the help page when you use the command line with the list of current features.

```
wof 0.1.0
Jones Magloire @Joxit
The Who's On First rust library and command line.

USAGE:
    wof <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build         Build a WOF database (sqlite or shapefile)
    completion    Generate autocompletion file for your shell
    export        Export tools for the Who's On First documents
    fetch         Fetch WOF data from github
    help          Prints this message or the help of the given subcommand(s)
    install       Install what you need to use this CLI (needs python2 and go)
    list          List all WOF document in the directory
    print         Print to stdout WOF document by id. Can be via stdin or cmd argument
```

### Build

You can build SQLite database or ESRI Shapefile.

```
wof-build 0.1.0
Build a WOF database (sqlite or shapefile)

USAGE:
    wof build <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    shapefile    Who's On First documents to ESRI shapefiles
    sqlite       Who's On First documents to SQLite database
```

### Completion

You can create your bash/elvish/fish/zsh completion.

```
wof-completion 0.1.0
Generate autocompletion file for your shell

USAGE:
    wof completion <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bash      Generates a .bash completion file for the Bourne Again SHell (BASH)
    elvish    Generates a completion file for Elvish
    fish      Generates a .fish completion file for the Friendly Interactive SHell (fish)
    help      Prints this message or the help of the given subcommand(s)
    zsh       Generates a completion file for the Z SHell (ZSH)
```

```
# Add bash completion for Debian
mkdir -p ~/.local/share/bash-completion/completions/
wof completion bash > ~/.local/share/bash-completion/completions/wof
```
