# comment grabber

## building js shared object

- I used the tree-sitter-cli tool, installed through npm:
```npm install -g tree-sitter-cli```

- clone the repo:
```git clone https://github.com/tree-sitter/tree-sitter-javascript.git```

- generate the necessary parts for tree-sitter
 ```tree-sitter generate```

 - build the shared object
 ```gcc -fPIC -shared -o libjavascript.so src/parser.c src/scanner.c```
