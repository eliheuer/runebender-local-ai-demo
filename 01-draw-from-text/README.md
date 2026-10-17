# 01: The local AI shapes tool

You train a small model on the 16 shapes in a UFO, inside the editor.
Then you drag a box on a glyph, type a word, and the model fills the
box. The model knows only the shapes in the UFO and the words for
them. Nothing is downloaded.

Time: about fifteen minutes, twelve of them training.

## 1. Look at the data

`fonts/Shapes.ufo` holds 16 glyphs, one shape each: circle, square,
triangle, star, ring, arrow, and so on. The glyph name is the label:
the glyph named `circle` teaches the word "circle".

```sh
runebender proof fonts/Shapes.ufo --out shapes.svg
```

To train on your own shapes, draw them in any UFO and name the glyphs.
A name with a hyphen is two words: `five-star`. A suffix after a dot
is ignored, so `circle.big` is another example of "circle".

## 2. Train in the editor

```sh
runebender fonts/Shapes.ufo
```

1. Open any glyph, for example `octagon`.
2. In the toolbar, hold the shapes button and pick **Local AI shape**.
   The right panel shows the **Local AI shapes tool** section.
3. Choose **Quick** (about 12 minutes, rough shapes) or **Thorough**
   (about 30 minutes, clean shapes), then click **Train from the
   sample shapes**. The panel shows the step, the time left, and what
   the model draws right now for three words.
4. When it finishes, the model `shapes-base` is chosen for you.

The same run from a shell is `./01-draw-from-text/train.sh`. The
model lands in `~/Runebender/models/shapes/shapes-base`, three files:
`config.json`, `weights.safetensors`, and `vocab.txt`.

## 3. Use the tool

1. With the tool active, drag a box on the glyph. Hold Shift for a
   square.
2. Type a word in the field under the box, for example `star`, and
   press Enter.
3. The model draws into the box as a ghost. **Enter** keeps it as one
   undo step. **Tab** asks for another option. **Esc** drops it.

The panel lists every word the model knows. A word it does not know
is reported under the box.

## 4. Add your own shape

Draw a new shape in any glyph of the UFO, name the glyph for it, and
click **Train from this font** with the base chosen. The run continues
from the base: the new word takes a spare slot and the base's own
drawings are replayed. About two minutes. The new model is saved
beside the font in `Shapes.models/shapes/` and chosen for you.

## 5. From the shell

```sh
./01-draw-from-text/draw.sh star 100,100,400,400 octagon
```

This writes the drawing into `octagon` as a proposal layer and proofs
it to `draw.svg`. `git checkout -- fonts` removes the proposal.

## 6. Judge it

The model is tiny and trained for minutes. A thorough base draws the
sample shapes cleanly; a quick one draws them roughly; a word added in
two minutes comes out recognizable and uneven. Longer training makes
every one of them cleaner. Everything the model does comes from the
UFO.
