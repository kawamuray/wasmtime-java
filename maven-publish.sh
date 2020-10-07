#!/bin/bash
set -e

version="$1"
if [ -z "$version" ]; then
    echo "Usage: $0 VERSION" >&2
    exit 1
fi

cd $(dirname $0)
./gradlew clean universalJar

filename="wasmtime-java-${version}-universal.jar"

curl --fail -L "https://github.com/kawamuray/wasmtime-java/releases/download/v${version}/${filename}" -o "build/libs/$filename"
echo -n "MD5 $filename: "
md5sum < "build/libs/$filename"

./gradlew -x universalJar publish
