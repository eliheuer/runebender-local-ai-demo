#!/bin/sh
# Draw a shape from a word into a box, as a proposal layer in the sample font.
# This is what the editor's local AI shapes tool does when you press Enter.
#
# Usage: ./draw.sh <word> [x,y,w,h] [glyph] [model-dir]
#   word       the shape, such as star
#   box        the box it fills, in font units   default: 100,100,400,400
#   glyph      the glyph to draw into            default: octagon
#   model-dir  the shapes model                  default: ~/Runebender/models/shapes/shapes-base
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
WORD=${1:?usage: draw.sh <word> [x,y,w,h] [glyph] [model-dir]}
BOX=${2:-100,100,400,400}
GLYPH=${3:-octagon}
MODEL=${4:-$HOME/Runebender/models/shapes/shapes-base}
UFO=$ROOT/fonts/Shapes.ufo

font-ml run draw --model "$MODEL" --source "$UFO" --glyph "$GLYPH" --prompt "$WORD" --box "$BOX" --write
runebender proof "$UFO" --layer com.runebender.proposal.draw --glyphs "$GLYPH" --out "$ROOT/draw.svg" > /dev/null
echo "Proof of the proposal: $ROOT/draw.svg"
echo "Remove the proposal:   git -C $ROOT checkout -- fonts"
