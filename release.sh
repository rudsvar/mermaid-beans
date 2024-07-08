#!/bin/sh

set -e

cargo bump "$1"
VERSION=$(cargo metadata --format-version=1 --no-deps | jq '.packages[0].version' -r)
cargo generate-lockfile
git add .
git commit -m "Release version ${VERSION}"
git tag "v${VERSION}"
git push
git push --tags
