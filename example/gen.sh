#!/usr/bin/env sh
echo "generating example output."
set -e
echo "building.."
cargo build
echo "copying generated source"
cp target/debug/build/example-*/out/* gen/
echo "done."
