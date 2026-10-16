# 04: The same loop on a real typeface

Shapes is a toy. Virtua Grotesk is a real family with a drawn Regular
and Bold, in `fonts/VirtuaGrotesk`. The loop is the same; the results
show what the method can and cannot do on real drawings.

Time: ten minutes, or longer if you train.

## 1. Train on it

```sh
font-ml run train \
    --source fonts/VirtuaGrotesk/VirtuaGrotesk-Regular.ufo \
    --target fonts/VirtuaGrotesk/VirtuaGrotesk-Bold.ufo \
    --out models/virtua-bolden --colors any --recenter --steps 2000
```

This uses the default model size, about 12M parameters and 50 MB on
disk, and takes ten to twenty minutes on a laptop GPU. Stop early with
`--minutes 5` for a rough model.

## 2. Score it

```sh
font-ml eval --model models/virtua-bolden \
    --regular fonts/VirtuaGrotesk/VirtuaGrotesk-Regular.ufo \
    --bold fonts/VirtuaGrotesk/VirtuaGrotesk-Bold.ufo --fit-stems
```

`--fit-stems` adds a column that matters on real type: whether the
predicted stems land on the weight the Bold master already carries.
Average point error can look fine while stems are wrong, and stems
are what decide if a glyph reads as the right weight.

## 3. Use it in the editor

```sh
RUNEBENDER_MODELS=$PWD/models runebender fonts/VirtuaGrotesk/VirtuaGrotesk.designspace
```

Pick `virtua-bolden` under **Local AI**, open a glyph in Regular, and
click **Bolden**. Straight stems come out better than curves; `H` is
a good first glyph and `eight` a hard one. Open the same glyph in the
Bold master to compare against the drawn answer, or run `font-ml eval`
on that glyph to see whether the model beats a uniform shift there.

## What to take away

The method keeps point structure by construction: it moves points,
and cannot add, remove, or reorder them, so interpolation is safe. The
quality of the move is the open question, and the answer today is
"sometimes". A small model trained on one family is a drawing
assistant that you judge, not a tool that finishes a master.
