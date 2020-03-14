#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE[0]}")"

cargo build --quiet

ls data/*.md | while read file; do
  dir="$(dirname "$file")"
  name="$(basename "$file")"

  cargo run --quiet -- "$file" > "$dir/sort/$name"
  cargo run --quiet -- -n -t Today "$file" > "$dir/next/$name"
done
