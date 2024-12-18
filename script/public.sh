#!/usr/bin/env bash

#
# Copy the files to the public directory.
#

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

mkdir -p public
if [ -f public/index.html ]; then
    chmod 666 public/index.html
fi
if [ -f public/style.css ]; then
    chmod 666 public/style.css
fi
if [ -f public/frontend.js ]; then
    chmod 666 public/frontend.js
fi
if [ -f public/pa.js ]; then
    chmod 666 public/pa.js
fi
if [ -f public/pa.wasm ]; then
    chmod 666 public/pa.wasm
fi

cp --verbose index.html public
cp --verbose style.css public
cp --verbose favicon.png public
cp --verbose frontend.js public
cp --verbose target/wasm32-unknown-emscripten/release/pa.js public
cp --verbose target/wasm32-unknown-emscripten/release/pa.wasm public

# To avoid accidentally editing the files in public manually.
chmod 444 public/index.html
chmod 444 public/style.css
chmod 444 public/frontend.js
chmod 444 public/pa.js
chmod 444 public/pa.wasm
