
#!/bin/bash

if [ $# -eq 0 ]
    then
        echo "Usage ./build.sh <run | build>"
        exit 1
fi

FILEPATH="$(readlink -f "$0")"
BASEDIR="$(dirname "$FILEPATH")"
echo "BASEDIR: $BASEDIR"
cd src

if cargo "$@"; then
    [ -d "$BASEDIR/target/debug" ] && cp "$BASEDIR/src/weather_gov.yml" "$BASEDIR/target/debug/"
    [ -d "$BASEDIR/target/release" ] && cp "$BASEDIR/weather_gov.yml" "$BASEDIR/target/release/"
fi

