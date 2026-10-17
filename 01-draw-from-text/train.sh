#!/bin/sh
# Train a shapes model on the sample font: a word and a box in, a shape out.
# This is what the editor's "Train from the sample shapes" button runs.
#
# Usage: ./train.sh [out-dir] [steps]
#   out-dir  where the model is written   default: ~/Runebender/models/shapes/shapes-base
#   steps    optimizer steps              default: 15000
#
# About 30 minutes on an Apple M-series laptop or a desktop GPU for the
# default; 6000 steps takes about 12 minutes and draws rougher shapes.
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
OUT=${1:-$HOME/Runebender/models/shapes/shapes-base}
STEPS=${2:-15000}

font-ml run train-draw \
    --source "$ROOT/fonts/Shapes.ufo" \
    --out "$OUT" \
    --steps "$STEPS" --dims 256 --layers 6 --heads 8 --batch 16 \
    --spare 16 --sample-words circle,star,triangle

echo
echo "Model written to $OUT"
echo "Open the editor:  runebender $ROOT/fonts/Shapes.ufo"
