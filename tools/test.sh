#!/usr/bin/env bash

if [ -z "$BROWSER" ]; then
	if [ $(uname) = Linux ]; then
		BROWSER=firefox
	else
		BROWSER=/Applications/Firefox.app/Contents/MacOS/firefox
	fi
fi

if [ -z "$1" ]; then
	echo "No definition provided; use: $0 <doc>"
	exit 1
else
	spec="$1"
	shift
fi

name="$(basename "$spec")"

cargo run -- --debug generate \
	--template etc/static/template/treno/suite.html "$spec" \
	--output tmp && "$BROWSER" "file://$PWD/tmp/${name%%.*}.html"
