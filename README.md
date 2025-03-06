# firefox-dmenu-integration

An integration util between dmenu and firefox. It gets recently accessed 
host names from history database, and calls dmenu for selection.   
**100% Written in Rust.**

## Usage

```
Usage: firefox-dmenu-integration [OPTIONS] --browser <BROWSER> --dmenu <DMENU>

Options:
  -b, --browser <BROWSER>  Path to browser executable
  -m, --dmenu <DMENU>      Path to dmenu executable
  -p, --profile <PROFILE>  Location history sqlite database [default: ~/.mozilla/firefox/000000.default]
  -l, --limit <LIMIT>      Limit of location history entries to collect [default: 100]
  -h, --help               Print help
  -V, --version            Print version
```

### Finding your Firefox profile location
> In the future, fuzzily attempting to discover firefox profile directories can
be implemented. See To-dos.  

Your Firefox profile is most like located in
`~/.mozilla/firefox/<string of numbers>.default`.
To verify that this is the correct folder, confirm that `places.sqlite` exists
in this folder.

## How it works

It uses `rusqlite` to connect to the locations database, then collects
non-duplicate items into a set, then formatted as standard input for dmenu (or
dmenu-like alternatives). The choice output by dmenu is then put as URL
argument for launching the browser specified in `--browser` argument.
