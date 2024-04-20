#!/bin/bash
# Find all Cargo.toml files in any subdirectory and check formatting

find . -name Cargo.toml -print0 | while IFS= read -r -d '' file; do
    dir=$(dirname "$file")
    echo "Running cargo fmt --check in $dir"
    cargo fmt --check --manifest-path "$file"
    if [ $? -ne 0 ]; then
        echo "Formatting check failed in $dir"
        exit 1
    fi
done

