#!/bin/sh
# Train a draw model on the Shapes font: a sentence in, a shape out.
#
# Usage: ./train.sh [out-dir] [steps]
#   out-dir  where the model is written   default: ../models/shapes-draw
#   steps    optimizer steps              default: 6000
#
# About five minutes on an Apple M-series laptop or a desktop GPU.
# Fewer steps give a rougher model; more steps give a cleaner one.
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
OUT=${1:-$ROOT/models/shapes-draw}
STEPS=${2:-6000}

font-ml run train-draw \
    --source "$ROOT/fonts/Shapes/Shapes-Regular.ufo" \
    --out "$OUT" \
    --steps "$STEPS" --dims 256 --layers 6 --heads 8 --batch 16

echo
echo "Model written to $OUT"
echo "Open the editor:  RUNEBENDER_MODELS=$ROOT/models runebender $ROOT/fonts/Shapes/Shapes.designspace"
