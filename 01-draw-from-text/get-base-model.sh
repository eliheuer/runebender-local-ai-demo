#!/bin/sh
# Download the trained shapes base model into ~/Runebender/models/shapes
# and check it against a known SHA-256, so you can use the tool without
# training first.
#
# Usage: ./get-base-model.sh [models-dir]     default: ~/Runebender/models
#
# Source: https://github.com/eliheuer/runebender-local-ai-demo/releases/tag/models-v1
set -eu

MODELS=${1:-$HOME/Runebender/models}
DIR=$MODELS/shapes
URL=https://github.com/eliheuer/runebender-local-ai-demo/releases/download/models-v1/shapes-base.zip
SHA=0cb07184aca8bca1a9a4c7f57aa95b80edbad2128505066d559708948bfd28fc

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}

if [ -f "$DIR/shapes-base/weights.safetensors" ]; then
    echo "already here: $DIR/shapes-base"
    exit 0
fi
mkdir -p "$DIR"
TMP=$(mktemp -d)
echo "fetching $URL (20 MB)"
curl -L --fail --retry 3 -o "$TMP/shapes-base.zip" "$URL"
got=$(sha256 "$TMP/shapes-base.zip")
if [ "$got" != "$SHA" ]; then
    echo "checksum mismatch" >&2
    echo "  want $SHA" >&2
    echo "  got  $got" >&2
    exit 1
fi
unzip -q "$TMP/shapes-base.zip" -d "$DIR"
rm -rf "$TMP"
echo "verified and unpacked: $DIR/shapes-base"
echo "Open the editor:  runebender fonts/Shapes.ufo   (pick Local AI Shapes in the shapes dropdown)"
