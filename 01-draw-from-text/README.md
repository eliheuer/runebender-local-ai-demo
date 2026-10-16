# 01: Draw a shape from a sentence

You train a small model on the shapes in a UFO, then type "draw a
circle 64 units wide in the lower left" in the editor and the model
draws it into the open glyph. The model knows only the shapes in the
UFO and the words for them. Nothing is downloaded.

Time: about fifteen minutes, ten of them training.

## 1. Look at the data

`fonts/Shapes/Shapes-Regular.ufo` holds 16 shapes in 8 sizes each. The
glyph name is the label: `circle`, `circle.300`, and `circle.700` all
teach the word "circle". The trainer measures each drawing, so sizes do
not matter; several sizes are several examples of one word.

```sh
runebender proof fonts/Shapes/Shapes-Regular.ufo --out shapes.svg
```

To train on your own shapes, draw them in any UFO and name the glyphs.
A name with a hyphen is two words: `five-star`.

## 2. Train

```sh
./01-draw-from-text/train.sh
```

This runs `font-ml run train-draw` and writes `models/shapes-draw`:
three files, `config.json`, `weights.safetensors`, and `vocab.txt`.
Open `vocab.txt` to see every word the model can read. The training
run prints a validation loss every 500 steps; lower is better, and the
checkpoint with the lowest value is the one kept.

On an M4 Pro laptop the default run of 6000 steps takes about 11
minutes and ends with a validation loss near 2.2. The checked-in
`models/shapes-draw` was trained longer, 15000 steps, and draws
cleaner shapes; use it if you want to skip the wait.

## 3. Draw in the editor

```sh
RUNEBENDER_MODELS=$PWD/models runebender fonts/Shapes/Shapes.designspace
```

1. In the left panel, open **Local AI**. Under **Model**, choose
   `shapes-draw`.
2. Open any glyph, for example `octagon`.
3. Under **Shapes**, type a sentence in the **Draw** box and click
   **Draw: this glyph**.
4. The drawing lands in the glyph as a proposal. **Install** keeps it,
   **Discard** drops it. Install is one undo step.

Sentences the model understands:

- `draw a circle 64 units wide in the lower left`
- `a star 300 wide in the center`
- `hexagon 200 in the upper right`
- `make a ring that is 160 units wide at the top`

A width is rounded to the nearest 8 units. A word that is not in
`vocab.txt` is reported in the status line and ignored.

## 4. Draw from the shell

```sh
./01-draw-from-text/draw.sh "a square 400 units wide in the center" octagon
```

This writes the drawing into `octagon` as a proposal layer and proofs
it to `draw.svg`. `git checkout -- fonts` removes the proposal.

## 5. Judge it

The model is tiny and trained for minutes. Shapes come out recognizable
and rough. Longer training makes them cleaner:

```sh
./01-draw-from-text/train.sh models/shapes-draw 12000
```

Everything the model does comes from the UFO. Draw a heart, name it
`heart`, train again, and "a heart 200 wide" works.
