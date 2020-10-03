#!/bin/bash
set -e

version="$(./gradlew properties | grep '^version:' | awk '{print $2}')"
echo "Building universal jar for $version"

function download_jnilib() {
    platform="$1"
    ext="$2"
    url="https://github.com/kawamuray/wasmtime-java/releases/download/v${version}/libwasmtime_jni_${version}_${platform}.${ext}"
    echo "Downloading $url"
    curl --fail -L "$url" -o build/jni-libs/libwasmtime_jni_${version}_${platform}.${ext}
}

mkdir -p build/jni-libs
download_jnilib "linux" "so"
download_jnilib "macos" "dylib"

./gradlew universalJar
