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
- About 1 GB of disk for the chat model in demo 02; 3 GB for the larger one.

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

`font-ml tasks` lists what the models can do. `bolden` and `train` are
the two this repo uses.

## The demos

| | What you do | What you learn |
| --- | --- | --- |
| [01-train-and-bolden](01-train-and-bolden/) | Train a model on the Shapes font, then bolden a new shape in the editor | A model is a folder you make from your own drawings |
| [02-chat-model](02-chat-model/) | Download a small open chat model and ask it about the font | A model from Hugging Face plugs into the same folder |
| [03-nodes](03-nodes/) | The same train and bolden steps as a graph in the Nodes tab | Workflows are files you can share and rerun |
| [04-real-font](04-real-font/) | The bolden loop on a real typeface, Virtua Grotesk | What works and what does not on real drawings |

Do them in order. 01 is the one to do if you do only one.

## Models

Every model is a directory. Trained and downloaded models go in
`models/` here, which git ignores, except `models/shapes-bolden`, a
trained copy of the demo 01 model that is checked in so the editor
demos work even if your training run fails. See
[models/MODELS.md](models/MODELS.md).

The editor looks for models in `~/.runebender/models`. To point it at
this repo instead, run it with `RUNEBENDER_MODELS` set:

```sh
RUNEBENDER_MODELS=$PWD/models runebender fonts/Shapes/Shapes.designspace
```

## Fonts

- `fonts/Shapes`: a family of geometric shapes in Regular and Bold,
  written by `tools/make-shapes`. Every glyph has the same points in
  both masters, which is what a bolden model trains on. Two shapes,
  octagon and chevron, exist in Regular only. Those are the ones you
  bolden with the model you train.
- `fonts/VirtuaGrotesk`: Regular and Bold of a real typeface, for demo 04.

To regenerate Shapes after changing the generator:

```sh
cd tools/make-shapes && cargo run --release -- ../../fonts/Shapes
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
