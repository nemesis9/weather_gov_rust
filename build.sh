
#!/bin/bash

DIR="$(dirname "$0")"
echo "DIR: $DIR"
cd src

if cargo "$@"; then
    [ -d "$DIR/target/debug" ] && cp "$DIR/src/weather_gov.yml" "$DIR/target/debug/"
    [ -d "$DIR/target/release" ] && cp "$DIR/weather_gov.yml" "$DIR/target/release/"
fi

