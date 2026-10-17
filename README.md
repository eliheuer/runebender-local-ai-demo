# Runebender local AI demo

Hello world examples for local AI in [Runebender](https://runebender.org).
Clone this repo, install Runebender, and work through the numbered
folders. Each one takes a few minutes. Nothing leaves your machine.

The point is to show the loop, not to make a good font: you train a
small model on your own drawings, and then use it from the editor. The
models are small on purpose. Build on them later.

## What you need

- A mid-range computer: an Apple Silicon laptop, or a Linux PC with a
  gaming GPU, or a plain CPU with patience. Not a datacenter.
- Rust stable, from [rustup.rs](https://rustup.rs).
- About 1 GB of disk for the chat model in demo 03; 3 GB for the larger one.

## Install

Two programs: the editor, and `font-ml`, which runs and trains the models.

```sh
cargo install --git https://github.com/eliheuer/runebender-xilem
cargo install --git https://github.com/eliheuer/font-ml --features metal    # macOS
cargo install --git https://github.com/eliheuer/font-ml --features cuda     # Linux with NVIDIA
cargo install --git https://github.com/eliheuer/font-ml                     # anything else (CPU)
```

Check both answer:

```sh
runebender --help
font-ml tasks
```

`font-ml tasks` lists what the models can do. This repo uses `draw`,
`train-draw`, `bolden`, and `train`.

## The demos

| | What you do | What you learn |
| --- | --- | --- |
| [01-draw-from-text](01-draw-from-text/) | Train a shapes model inside the editor, then drag a box, type "star", and the model fills it | A model is a folder you make from your own drawings, and it only knows what you drew |
| [02-train-and-bolden](02-train-and-bolden/) | Train a model on the Shapes font, then bolden a new shape in the editor | A model that moves points keeps a family interpolating |
| [03-chat-model](03-chat-model/) | Download a small open chat model and ask it about the font | A model from Hugging Face plugs into the same folder |
| [04-nodes](04-nodes/) | The same train and bolden steps as a graph in the Nodes tab | Workflows are files you can share and rerun |
| [05-real-font](05-real-font/) | The bolden loop on a real typeface, Virtua Grotesk | What works and what does not on real drawings |

Do them in order. 01 is the one to do if you do only one.

## Models

Every model is a directory. The editor reads them from
`~/Runebender/models/<kind>/`, with kinds `shapes`, `bolden`, and
`chat`, and from `<Font>.models/` beside a font. Models this repo
downloads or trains for the other demos go in `models/` here, which
git ignores, except `models/shapes-bolden` and `models/shapes-base`,
checked in so the editor demos work even if a training run fails. See
[models/MODELS.md](models/MODELS.md). To use this repo's folder as the
model root, run the editor with `RUNEBENDER_MODELS=$PWD/models`.

## Fonts

- `fonts/Shapes.ufo`: 16 geometric shapes, one glyph each. Demo 01
  trains on this.
- `fonts/ShapesFamily`: the same shapes in 8 sizes, as Regular and
  Bold masters with the same points in both, which is what a bolden
  model trains on. Two shapes, octagon and chevron, exist in Regular
  only. Those are the ones you bolden in demo 02.
- `fonts/VirtuaGrotesk`: Regular and Bold of a real typeface, for demo 05.

To regenerate Shapes after changing the generator:

```sh
cd tools/make-shapes && cargo run --release -- ../../fonts
```

## Reset

Demos write proposal layers into the UFOs. To get back to a clean
checkout:

```sh
git checkout -- fonts
```

## Running the workshop

[WORKSHOP.md](WORKSHOP.md) is the run of show, with timings.

## Beyond this repo

- [Local Models](https://runebender.org/docs/local-models.html) on runebender.org
- [font-ml](https://github.com/eliheuer/font-ml): the model runtime, and where new tasks go
- [runebender-xilem](https://github.com/eliheuer/runebender-xilem): the editor

## License

Code and the Shapes font: Apache-2.0 OR MIT. Virtua Grotesk: OFL-1.1.
Downloaded chat models keep their own licenses (Qwen3: Apache-2.0).
