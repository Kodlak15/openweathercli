#!/usr/bin/env bash

# Setup script for the openweathermap.org CLI
# Builds the executable, downloads icons, and creates a symbolic link to some directory in PATH
# To prevent icons from being downloaded run with --noicons
# Make sure OUTDIR is set to some directory in your PATH

WORKDIR="$(find "$HOME" -name "openweathercli" -type d)"

get_icons() {
	icondir="$WORKDIR/assets/icons"

	if [[ ! -d "$icondir" ]]; then
	    mkdir -p "$icondir"
	fi
	
	declare -a icons=(
	    "01d"
	    "01n"
	    "02d"
	    "02n"
	    "03d"
	    "03n"
	    "04d"
	    "04n"
	    "09d"
	    "09n"
	    "10d"
	    "10n"
	    "11d"
	    "11n"
	    "13d"
	    "13n"
	    "50d"
	    "50n"
	)
	
	if [[ $(find "$icondir" | wc -l) -lt 18 && ! "$1" == "--noicons" ]]; then
	    for icon in "${icons[@]}"; do
	        curl "https://openweathermap.org/img/wn/$icon@2x.png" --output "$icondir/$icon.png"
	    done
	fi
}

setup() {
	get_icons "$1"

	cargo build --release

	outdir="$HOME/bin"

	if [[ ! -d "$outdir" ]]; then
		mkdir -p "$outdir"
	fi

	exepath="$(find "$HOME" -name "openweathercli" -type f | grep "release")"

	ln -sf "$exepath" "$outdir/owcli"
}

setup "$@"
