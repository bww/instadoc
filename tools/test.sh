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
	definition="$1"
	shift
fi

cargo run -- --debug generate --template etc/static/template/treno/suite.html "$definition" --output tmp/suite.html && "$BROWSER" "file://$PWD/tmp/suite.html"

