shebang := "/usr/bin/env bash"

alias b := build
alias s := serve

default:
    just --list

# Copy files to the public directory.
public:
    #!{{shebang}}

    set -e

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

# Compile Rust to WebAssembly and copy the result to the public directory.
build: && public
    #!{{shebang}}

    set -e

    # The || true prevents the script from failing if the grep returns no results.
    EMSCRIPTEN_GREP="$(rustup target list --installed | { grep wasm32-unknown-emscripten || true; })"
    if [[ "$EMSCRIPTEN_GREP" == "" ]]; then
        echo "Expected the wasm32-unknown-emscripten target to be installed."
        echo "Run 'rustup target add wasm32-unknown-emscripten' to fix this error."
        echo ""
        echo "You might also need to install the emscripten compiler (emcc) via your package manager."
        echo "For Nix, set the EM_CACHE environment variable to something like ~/.cache/emscripten."
    fi

    echo "Compiling Rust to WebAssembly..."
    cargo build --target wasm32-unknown-emscripten --release

# Serve the website locally with live reload.
serve:
    #!{{shebang}}

    set -e

    echo "Starting server..."
    # Using https://github.com/tapio/live-server because it loads fast and supports live reload.
    # Install with npm install -g live-server.
    # Or via Nix, install nodePackages_latest.live-server.
    cd public && live-server --no-browser & SERVER_PID=$!
    
    find dist power index.html style.css frontend.js Cargo.toml | entr -s "just --justfile {{justfile()}} build"

    # Kill the server when the script exits
    trap "kill $SERVER_PID" EXIT
