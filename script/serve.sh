#!/usr/bin/env bash

#
# Serve the website locally with live reload.
#

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))

cd "$BASEDIR/public"

# Using https://github.com/tapio/live-server because it loads fast and supports live reload.
# Install with npm install -g live-server.
# Or via Nix, install nodePackages_latest.live-server.
live-server --no-browser
