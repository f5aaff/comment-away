# comment-away

## building tree-sitter shared objects:

- I used the tree-sitter-cli tool, installed through npm:
```npm install -g tree-sitter-cli```

- clone the repo:
clone the repo for the desired tree-sitter module, as an example, we'll use the javascript module:
```git clone https://github.com/tree-sitter/tree-sitter-javascript.git```

- generate the necessary files for tree-sitter
 ```tree-sitter generate```

 - build the shared object
 ```gcc -fPIC -shared -o libjavascript.so src/parser.c src/scanner.c```
