#!/bin/bash

# Function to print success messages in green
print_success() {
  echo -e "\033[0;32m$1\033[0m"
}

# Function to print error messages in red
print_error() {
  echo -e "\033[0;31m$1\033[0m"
}

# Default values for the languages file and output directory
LANGUAGES_FILE="./languages"
GRAMMARS_DIR="./shared_libs/"

# Parse command line arguments
while getopts "l:o:" opt; do
  case $opt in
    l)
      LANGUAGES_FILE="$OPTARG"
      ;;
    o)
      GRAMMARS_DIR="$OPTARG"
      ;;
    *)
      print_error "Usage: $0 [-l languages_file] [-o output_directory]"
      exit 1
      ;;
  esac
done

# Check if the languages file exists
if [[ ! -f "$LANGUAGES_FILE" ]]; then
  print_error "Error: Languages file '$LANGUAGES_FILE' not found."
  exit 1
fi

# Read the languages from the file
LANGUAGES=$(grep -vE '^\s*#|^\s*$' "$LANGUAGES_FILE")

# Check if any languages were found
if [[ -z "$LANGUAGES" ]]; then
  print_error "Error: No languages found in the file '$LANGUAGES_FILE'."
  exit 1
fi

# Create the grammars directory if it doesn't exist
mkdir -p "$GRAMMARS_DIR"

# Loop over the languages and process them
for lang in $LANGUAGES; do
  # Skip empty lines (though we should not have any at this point)
  if [[ -z "$lang" || "$lang" == \#* ]]; then
    continue
  fi

  REPO="https://github.com/tree-sitter/tree-sitter-$lang.git"
  echo "Cloning $REPO..."

  # Clone the grammar repository
  git clone "$REPO" && cd "tree-sitter-$lang" || {
    print_error "Failed to clone $REPO."
    continue
  }

  echo "Building $lang grammar..."

  # Build the grammar using GCC
  gcc -shared -fPIC -o "lib$lang.so" src/parser.c src/scanner.c && {
    # Move the shared library to the target directory
    mv "lib$lang.so" "../$GRAMMARS_DIR"
    print_success "Successfully built and moved $lang grammar."
  } || {
    print_error "Failed to build $lang grammar."
  }

  # Go back to the previous directory and clean up
  cd .. || exit
  rm -rf "tree-sitter-$lang"
done

print_success "Done. Shared libraries are in $GRAMMARS_DIR"

