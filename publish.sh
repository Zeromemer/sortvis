#!/bin/bash
set -e 
set -o pipefail

# builds
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu

# pushing to github
git tag -f alpha
git push origin alpha --force
gh release upload --clobber alpha target/release/sortvis target/x86_64-pc-windows-gnu/release/sortvis.exe