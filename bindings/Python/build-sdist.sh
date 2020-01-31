#!/bin/bash
set -ex

# Create a symlink for tokenizers-lib
ln -sf ../../ symspell-lib
# Modify cargo.toml to include this symlink
sed -i 's/\.\.\/\.\.\/\.\/symspell-lib/' Cargo.toml
# Build the source distribution
python setup.py sdist
