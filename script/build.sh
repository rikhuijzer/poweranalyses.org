#!/usr/bin/env bash

#
# Compile the Rust code to WebAssembly and copy the result to the public directory.
#

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

# The || true prevents the script from failing if the grep returns no results.
EMSCRIPTEN_GREP="$(rustup target list --installed | { grep wasm32-unknown-emscripten || true; })"
if [[ "$EMSCRIPTEN_GREP" == "" ]]; then
    echo "Expected the wasm32-unknown-emscripten target to be installed."
    echo "Run 'rustup target add wasm32-unknown-emscripten' to fix this error."
    echo ""
    echo "You might also need to install the emscripten compiler (emcc) via your package manager."
    echo "For Nix, set the EM_CACHE environment variable to something like ~/.cache/emscripten."
fi

cargo build --target wasm32-unknown-emscripten --release

./script/public.sh
