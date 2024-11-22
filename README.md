# comment-away

# Requirements
- git
- gcc
- openssl
- openssl-devel

# obtaining parsers
- the submodule "scraper" produces the tool parser_scraper
- this is a simple CLI tool for scraping and building the parser modules from
the tree sitter wiki.
- check the README under scraper/README.md

# Building

- clone this repo, including the submodule:
```
git clone --recurse-submodules https://github.com/f5aaff/comment-away.git
```

- Building should be as simple as running:
```cargo build```

# Usage
generate a config, and the parser shared objects using the scraper tool, ```parser_scraper```:
```./parser_scraper -l java,python,javascript,go,rust```

make sure that the paths in the config make sense, edit the node-types if you wish.

then, use comment-away, pointed at the target directory. It will recurse through the project, removing comments from any file that it has a matching parser for.

for an example, follow the instructions found at test_src/test_src.md.
```
$./target/debug/comment-away --help
Usage: comment-away [OPTIONS]

Options:
  -c, --config <CONFIG>  [default: config.json]
  -t, --target <TARGET>  [default: ./]
  -r, --replace-with-ws
  -h, --help             Print help
  -V, --version          Print version
```

- _-c Config_
    - points at the config to use, defaults to ./config.json
 - _-t target_
     - target directory, defaults to current directory
 - _-r replace-with-ws_
     - this will replace comments with whitespace in their entirety, rather than removing them outright.
 - _-h help_
     - prints the help/usage message
 - _-v version_
     - prints the version number of this tool
