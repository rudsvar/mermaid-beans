#!/bin/sh

set -e

cargo bump "$1"
cargo generate-lockfile
git add .
git commit -m "Release $(cargo metadata --format-version=1 --no-deps | jq '.packages[0].version' -r)"
# git push
# git push --tags
