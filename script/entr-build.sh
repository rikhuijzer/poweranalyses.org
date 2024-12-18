#!/usr/bin/env bash

#
# Build the project when a file changes.
#

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

find dist power | entr -s ./script/build.sh
