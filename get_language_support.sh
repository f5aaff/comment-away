#!/bin/bash

LANGUAGES=(
  "javascript"
  "python"
  "rust"
  "go"
)

GRAMMARS_DIR="./src/shared_libs/"
mkdir -p $GRAMMARS_DIR

for lang in "${LANGUAGES[@]}"; do
  REPO="https://github.com/tree-sitter/tree-sitter-$lang.git"
  echo "Cloning $REPO..."
  git clone $REPO
  cd tree-sitter-$lang

  echo "Building $lang grammar..."
  gcc -shared -fPIC -o lib$lang.so src/parser.c src/scanner.c
  mv lib$lang.so ../$GRAMMARS_DIR

  cd ..
  rm -rf tree-sitter-$lang
done

echo "Done. Shared libraries are in $GRAMMARS_DIR"
