#!/usr/bin/env bash


panic() {
  printf 'panic: \n %s' "$1"
  exit 1
}

publish () {
  publish_path="$(realpath "$1")"
  name="$(basename "$publish_path")"

  printf 'publishing: %s\n\n' "$name"
  printf '3 second sleep.\nTo abort press ^C\n'
  sleep 3

  pushd "$publish_path" || panic "error directory $publish_path not found"
  cargo publish
  popd || panic "popd failed"
}

publish ./seekr-migration
publish ./seekr-macro
publish ./.



