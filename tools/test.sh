#!/usr/bin/env bash

if [ -z "$1" ]; then
	echo "No definition provided; use: $0 <doc>"
	exit 1
else
	definition="$1"
	shift
fi

cargo run -- --debug generate --template etc/static/template/treno/suite.html "$definition" --output tmp/suite.html && /Applications/Firefox.app/Contents/MacOS/firefox "file://$PWD/tmp/suite.html"

