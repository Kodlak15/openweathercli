#!/usr/bin/env bash

# Setup script for the openweathermap.org CLI
# Builds the executable and creates a symbolic link to some directory in PATH
# Make sure OUTDIR is set to some directory in your PATH

EXEPATH="$(find "$HOME" -name "openweathercli" -type f | grep "release")"
OUTDIR="$HOME/bin"

setup() {
	cargo build --release

	if [[ ! -d "$OUTDIR" ]]; then
		mkdir -p "$OUTDIR"
	fi

	ln -sf "$EXEPATH" "$OUTDIR/owcli"
}

setup @
