#!/bin/sh
# Draw a shape from a sentence into a glyph, as a proposal layer.
#
# Usage: ./draw.sh "<sentence>" [glyph] [model-dir]
#   sentence   what to draw, such as "a circle 64 units wide in the lower left"
#   glyph      the glyph to draw into        default: octagon
#   model-dir  the draw model                default: ../models/shapes-draw
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
PROMPT=${1:?usage: draw.sh "<sentence>" [glyph] [model-dir]}
GLYPH=${2:-octagon}
MODEL=${3:-$ROOT/models/shapes-draw}
UFO=$ROOT/fonts/Shapes.ufo

font-ml run draw --model "$MODEL" --source "$UFO" --glyph "$GLYPH" --prompt "$PROMPT" --write
runebender proof "$UFO" --layer com.runebender.proposal.draw --glyphs "$GLYPH" --out "$ROOT/draw.svg" > /dev/null
echo "Proof of the proposal: $ROOT/draw.svg"
echo "Remove the proposal:   git -C $ROOT checkout -- fonts"
