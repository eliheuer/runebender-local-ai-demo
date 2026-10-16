#!/bin/sh
# Train a small bolden model on the Shapes family, then score it.
#
# Usage: ./train.sh [out-dir] [steps]
#   out-dir  where the model is written   default: ../models/shapes-bolden
#   steps    optimizer steps              default: 1200
#
# About three minutes on an Apple M-series laptop or a desktop GPU,
# longer on a plain CPU. Pass fewer steps for a quicker, rougher model.
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
OUT=${1:-$RO../models/shapes-bolden}
STEPS=${2:-1200}
FONTS=$ROOT/fonts/ShapesFamily

font-ml run train \
    --source "$FONTS/Shapes-Regular.ufo" \
    --target "$FONTS/Shapes-Bold.ufo" \
    --out "$OUT" \
    --colors any --recenter \
    --steps "$STEPS" --dims 128 --layers 4 --heads 4 --batch 16

echo
echo "Scoring the model against the Bold master (lower is better; beat the baseline):"
font-ml eval --model "$OUT" \
    --regular "$FONTS/Shapes-Regular.ufo" \
    --bold "$FONTS/Shapes-Bold.ufo"

echo
echo "Model written to $OUT"
echo "Open the editor:  RUNEBENDER_MODELS=$ROOT/models runebender $FONTS/Shapes.designspace"
