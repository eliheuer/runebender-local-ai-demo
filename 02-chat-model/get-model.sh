#!/bin/sh
# Download a Qwen3 chat model (GGUF) and its tokenizer into the models
# directory, and check both against known SHA-256 sums.
#
# Usage: ./get-model.sh [size] [models-dir]
#   size        0.6b (default), 1.7b, or 4b
#   models-dir  default: ../models
#
# Sizes:
#   0.6b  805 MB   Q8_0   runs anywhere; shows the loop, answers are poor
#   1.7b  1.3 GB   Q4_K_M better, still often wrong about the font
#   4b    2.5 GB   Q4_K_M what Runebender's Chat panel was built with
#
# Sources: https://huggingface.co/ggml-org (GGUF files, converted from
# Qwen's releases) and https://huggingface.co/Qwen (tokenizer.json).
# License: Apache-2.0 (Qwen3).
set -eu

SIZE=${1:-0.6b}
ROOT=$(cd "$(dirname "$0")/.." && pwd)
MODELS=${2:-$ROOT/models}
DIR=$MODELS/qwen3-$SIZE

case $SIZE in
  0.6b)
    GGUF_URL=https://huggingface.co/ggml-org/Qwen3-0.6B-GGUF/resolve/main/Qwen3-0.6B-Q8_0.gguf
    GGUF_SHA=361cc68159042c36ebff7715dc5a2e4612153e88f3e9c9c234820849d6dc9e1d
    TOK_URL=https://huggingface.co/Qwen/Qwen3-0.6B/resolve/main/tokenizer.json ;;
  1.7b)
    GGUF_URL=https://huggingface.co/ggml-org/Qwen3-1.7B-GGUF/resolve/main/Qwen3-1.7B-Q4_K_M.gguf
    GGUF_SHA=d2387ca2dbfee2ffabce7120d3770dadca0b293052bc2f0e138fdc940d9bc7b5
    TOK_URL=https://huggingface.co/Qwen/Qwen3-1.7B/resolve/main/tokenizer.json ;;
  4b)
    GGUF_URL=https://huggingface.co/ggml-org/Qwen3-4B-GGUF/resolve/main/Qwen3-4B-Q4_K_M.gguf
    GGUF_SHA=ab27b9bfa375a178d6cba48f3ad892b94b7739659dcc7aae8058ce0ffed6b328
    TOK_URL=https://huggingface.co/Qwen/Qwen3-4B/resolve/main/tokenizer.json ;;
  *)
    echo "unknown size '$SIZE': use 0.6b, 1.7b, or 4b" >&2; exit 2 ;;
esac
# All three Qwen3 sizes share one tokenizer.
TOK_SHA=aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}

fetch() {
    url=$1; dest=$2; want=$3
    if [ -f "$dest" ] && [ "$(sha256 "$dest")" = "$want" ]; then
        echo "ok       $dest"
        return
    fi
    echo "fetching $url"
    curl -L --fail --retry 3 -C - -o "$dest" "$url"
    got=$(sha256 "$dest")
    if [ "$got" != "$want" ]; then
        echo "checksum mismatch for $dest" >&2
        echo "  want $want" >&2
        echo "  got  $got" >&2
        exit 1
    fi
    echo "verified $dest"
}

mkdir -p "$DIR"
fetch "$GGUF_URL" "$DIR/model.gguf" "$GGUF_SHA"
fetch "$TOK_URL" "$DIR/tokenizer.json" "$TOK_SHA"
echo
echo "Model ready in $DIR"
echo "Try it:"
echo "  font-ml chat --model $DIR --font $ROOT/fonts/Shapes/Shapes.designspace \\"
echo "      --prompt 'Use the tools to read this font. How many glyphs does it have?'"
