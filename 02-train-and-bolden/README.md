# 01: Train a model, then use it in the editor

You train a small model that learns how Bold differs from Regular in
the Shapes font. Then you open the font in Runebender and use the
model on a shape that has no Bold yet.

Time: about five minutes. Training takes one to three minutes on an
Apple Silicon laptop or a desktop GPU, and longer on a CPU.

## 1. Look at the font

`fonts/ShapesFamily` has two masters. Regular has 128 glyphs, Bold has 112.
Every shape comes in eight sizes, and every glyph has the same points
in the same order in both masters. Two shapes, octagon and chevron,
are drawn in Regular only.

```sh
runebender info fonts/ShapesFamily/Shapes-Regular.ufo
runebender proof fonts/ShapesFamily/Shapes-Regular.ufo --out shapes.svg
```

## 2. Train

```sh
./02-train-and-bolden/train.sh
```

The script runs one `font-ml run train` command: Regular is the
source, Bold is the target, and the model learns the offset of every
point. The output is a folder, `models/shapes-bolden`, with three
files:

| File | What it is |
| --- | --- |
| `config.json` | The shape of the network |
| `weights.safetensors` | The trained weights, about 5 MB |
| `vocab.txt` | The glyph names and tokens it knows |

The script then scores the model against the Bold master. For each
glyph it prints the model's average error per point and a baseline:
the error from moving every point by the average offset. The model
should beat the baseline on most glyphs. A model that cannot beat a
constant has not learned anything.

A copy of this model is checked in, so if the run fails you can still
do step 3.

## 3. Use it in the editor

```sh
RUNEBENDER_MODELS=$PWD/models runebender fonts/ShapesFamily/Shapes.designspace
```

1. In the left panel, open **Local AI**. Under **Model**, choose
   `shapes-bolden`.
2. Open the glyph `octagon` in the Regular master.
3. Click **Bolden**. The model predicts a heavier octagon and installs
   it. The status line reports how many points moved.
4. Undo if you do not like it. It is an ordinary edit.

Try `chevron` next. Then draw a new shape of your own in Regular, give
it a name, and bolden that.

## 4. Judge it

Octagon and chevron are not in the training data, so this is a fair
test. On trained shapes the prediction is a clean offset. On new
shapes it is rough: edges move outward, but not evenly. That is what
a model this small, trained on this little, can do. The lesson is in
the loop, not in the result: you drew the data, you trained the
model, and it runs inside the editor on your drawings.

To get a better model, give it more drawings. Add shapes to
`tools/make-shapes`, draw Bold versions of octagon and chevron by
hand, or train for longer with `./train.sh models/shapes-bolden 3000`.

## From the command line

The same thing without the editor, writing a proposal layer instead
of editing the glyph:

```sh
font-ml run bolden --model models/shapes-bolden \
    --source fonts/ShapesFamily/Shapes-Regular.ufo --glyph octagon --write
runebender proposal list fonts/ShapesFamily/Shapes-Regular.ufo
runebender proof fonts/ShapesFamily/Shapes-Regular.ufo \
    --layer com.runebender.proposal.bolden --glyphs octagon --out octagon.svg
```

`git checkout -- fonts` removes the proposal again.
