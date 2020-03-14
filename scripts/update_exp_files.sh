#!/usr/bin/env bash

cargo build --quiet

ls tests/*.md | while read file; do
  echo "$file"

  if [ -f "$file.sort.exp" ]; then
    echo "  $file.sort.exp"
    cargo run --quiet -- "$file" > "$file.sort.exp"
  fi

  if [ -f "$file.next.exp" ]; then
    echo "  $file.next.exp"
    cargo run --quiet -- -n -t Today "$file" > "$file.next.exp"
  fi
done
