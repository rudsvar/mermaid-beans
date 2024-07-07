#!/bin/sh

set -e

cargo bump "$1" -g
git push
git push --tags
