
#!/bin/bash

if [ $# -eq 0 ]
    then
        echo "Usage ./build.sh <build | run>"
        exit 1
fi

FILEPATH="$(readlink -f "$0")"
BASEDIR="$(dirname "$FILEPATH")"
echo "BASEDIR: $BASEDIR"

if [ "$1" == "run" ]; then
    if [[ ! -d "$BASEDIR/target/debug" ]] &&  [[ ! -d "$BASEDIR/target/release" ]] ; then
	    echo "do cargo build first so weather_gov.yml gets copied to the build"
	    exit 1
	fi
fi
cd src

if cargo "$@"; then
    echo "Copy weather_gov.yml" 
    [ -d "$BASEDIR/target/debug" ] && cp "$BASEDIR/src/weather_gov.yml" "$BASEDIR/target/debug/"
    [ -d "$BASEDIR/target/release" ] && cp "$BASEDIR/src/weather_gov.yml" "$BASEDIR/target/release/"
fi

