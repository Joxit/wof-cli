# wof

[![Rust](https://github.com/Joxit/wof-cli/workflows/Rust/badge.svg)](https://github.com/Joxit/wof-cli/actions?query=workflow%3ARust)
[![Crates.io version shield](https://img.shields.io/crates/v/wof.svg)](https://crates.io/crates/wof)
[![Crates.io license shield](https://img.shields.io/crates/l/wof.svg)](https://crates.io/crates/wof)

This project is both a CLI and a library to work with [Who's On First](https://www.whosonfirst.org/) documents in Rust.

If you want the CLI, install it with [cargo](https://doc.rust-lang.org/cargo/):

Gdal feature is usefull only for export feature because there is precision differences between GDAL C library and rust when I'm recalculating area in meter. 

```bash
cargo install wof --force --features cli --features with-gdal
```

## CLI

Where is the help page when you use the command line with the list of current features.

```
wof 0.2.0
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
wof-build 0.2.0
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

#### Build Shapefile

```
wof-build-shapefile 0.2.0
Who's On First documents to ESRI shapefiles

USAGE:
    wof build shapefile [FLAGS] [OPTIONS] [--] [directory]

FLAGS:
    -h, --help       Prints help information
        --timings    Display timings during and after indexing
    -V, --version    Prints version information
    -v, --verbose    Activate verbose mode

OPTIONS:
        --belongs-to <belongs-to>...        Include only records that belong to this ID. You may pass multiple -belongs-
                                            to flags
        --exclude-placetype <exclude>...    Exclude records of this placetype. You may pass multiple -exclude-placetype
                                            flags
        --include-placetype <include>...    Include only records of this placetype. You may pass multiple -include-
                                            placetype flags
        --mode <mode>                       The mode to use importing data [default: repo]  [possible values: directory,
                                            feature, feature-collection, files, geojson-ls, meta, path, repo, sqlite]
        --out <out>                         Where to write the new shapefile [default: whosonfirst-data-latest.shp]
        --shapetype <shapetype>             The shapefile type to use indexing data [default: POLYGON]  [possible
                                            values: POINT, POLYLINE, POLYGON]

ARGS:
    <directory>     [default: .]
```

#### Build SQLite

```
wof-build-sqlite 0.2.0
Who's On First documents to SQLite database

USAGE:
    wof build sqlite [FLAGS] [OPTIONS] [directories]...

FLAGS:
    -h, --help             Prints help information
        --no-deprecated    Don't insert deprecated features
        --no-pretty        Don't prettify the geojson
        --timings          Display timings during the build process, implies verbose
    -V, --version          Prints version information
    -v, --verbose          Activate verbose mode

OPTIONS:
        --out <out>          Where to store the final build file. If empty the code will attempt to create whosonfirst-
                             data-latest.db the current working directory [default: whosonfirst-data-
                             latest.db]
        --preset <preset>    Preset for pelias use. Will insert only in geojson and spr tables [possible values: pelias]

ARGS:
    <directories>...    WOF data directories [default: .]
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

### Export

```
wof-export 0.2.0
Export tools for the Who's On First documents

USAGE:
    wof export [FLAGS] [OPTIONS]

FLAGS:
    -c, --collection    
        --debug         Activate debug mode
    -h, --help          Prints help information
        --stagged       Run export on all stagged files (needs git repository)
        --stdin         Read stdin for the object to export
    -V, --version       Prints version information
    -v, --verbose       Activate verbose mode

OPTIONS:
    -a, --alt <alt>              
        --commit <commit>        Run export on all files of a specific commit/ref (needs git repository)
    -d, --display <display>      
    -e, --exporter <exporter>    Where to write the export, on stdout or flatfile (needs source) [possible values:
                                 flatfile, stdout]
    -i, --id <id>                The WOF id of the object to export
    -p, --path <path>            Path of the object to export
    -s, --source <source>        WOF data folder where are stored GeoJSONs to exportify
```

### Fetch

```
wof-fetch 0.2.0
Fetch WOF data from github

USAGE:
    wof fetch [FLAGS] [OPTIONS] [countries]...

FLAGS:
    -h, --help       Prints help information
        --timings    Display timings during the download process, implies verbose
    -V, --version    Prints version information
    -v, --verbose    Activate verbose mode

OPTIONS:
        --admin <admin>              Should download admin repositories, default true [possible values: true, false]
    -o, --out <out>                  Ouput directory to download WOF documents [default: .]
        --postalcode <postalcode>    Should download postalcodes repositories [possible values: true, false]

ARGS:
    <countries>...    Two letters country code to download. No values will download all repositories
```

### Install

```
wof-install 0.2.0
Install what you need to use this CLI (needs python2 and go)

USAGE:
    wof install <package>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <package>    Name of the package to install (saved in ~/.wof directory) [possible values: export]
```

### List

```
wof-list 0.2.0
List all WOF document in the directory

USAGE:
    wof list [FLAGS] [directories]...

FLAGS:
        --alt              List also alternate geometries
    -h, --help             Prints help information
        --no-deprecated    Don't print deprecated features
    -V, --version          Prints version information

ARGS:
    <directories>...    Paths to WOF documents [default: .]
```

### Print

```
wof-print 0.2.0
Print to stdout WOF document by id. Can be via stdin or cmd argument

USAGE:
    wof print [FLAGS] [OPTIONS] [--] [ids]...

FLAGS:
    -h, --help         Prints help information
        --no-geom      Remove the geometry before pretty print
        --no-pretty    Remove the geometry before pretty print
    -r, --raw          Send the raw data, do not pretty print it. You can't use filters with this
    -V, --version      Prints version information

OPTIONS:
    -e, --exclude <excludes>...    Exclude some properties from the input. `wof:` will exclude all properties starting
                                   with `wof:`
    -i, --include <includes>...    Include some properties from the input. `wof:` will include only properties starting
                                   with `wof:`

ARGS:
    <ids>...    Ids or paths to WOF documents to print
```