#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE[0]}")"

cargo build --quiet

ls data/*.md | while read file; do
  echo "Updating $file"

  dir="$(dirname "$file")"
  name="$(basename "$file")"

  cargo run --quiet -- "$file" > "$dir/sort/$name"
  cargo run --quiet -- -n -t Today "$file" > "$dir/next/$name"
done
