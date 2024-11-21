# comment-away

# Requirements
- git
- gcc
- openssl
- openssl-devel

# obtaining parsers
- the submodule "scraper"
- this is a simple CLI tool for scraping and building the parser modules from
the tree sitter wiki.
- check the README under scraper/README.md

# Building
- Building should be as simple as running:
```cargo build```

# Usage
```
$./target/debug/comment-away --help
Usage: comment-away [OPTIONS]

Options:
  -c, --config <CONFIG>  [default: config.json]
  -t, --target <TARGET>  [default: ./]
  -h, --help             Print help
  -V, --version          Print version
```

- _-c Config_
    - points at the config to use, defaults to ./config.json
 - _-t target_
     - target directory, defaults to current directory
 - _-h help_
     - prints the help/usage message
 - _-v version_
     - prints the version number of this tool
