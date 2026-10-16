# Workshop run of show

Ninety minutes. Everyone has cloned this repo and installed
`runebender` and `font-ml` before the session; the install takes
fifteen minutes of compiling and should not happen live.

| Time | Segment | What the room does |
| --- | --- | --- |
| 0:00 | Why local | Five minutes of talk: a model is a folder, nothing leaves the machine, you can train on your own drawings. |
| 0:05 | Check installs | `font-ml tasks`, `runebender --help`. Fix PATH problems now. |
| 0:10 | Demo 01 | Everyone runs `01-draw-from-text/train.sh`. While it trains: what a UFO is, what the glyph names mean, what the model will and will not know. Then type sentences in the Shapes box. |
| 0:25 | Demo 02, train | Everyone runs `train.sh`. While it runs: what Regular and Bold are, what point-compatible means, look at the Shapes proof. |
| 0:20 | Demo 02, editor | Open the font with `RUNEBENDER_MODELS`, bolden `octagon`, undo, bolden `chevron`. Draw a shape, bolden it. |
| 0:35 | Judge | Read the eval table together. Model versus baseline. Why a new shape is rough. What more data would do. |
| 0:45 | Demo 03 | `get-model.sh` was run before the session. Chat from the shell, then in the editor. Show it reading the font and getting it wrong. Show 4B if the presenter has it. |
| 1:00 | Demo 04 | Open the Nodes tab, run `shapes-train`, change `steps`, run again. Show the same file running from the shell. |
| 1:15 | Demo 05 | Presenter only: Virtua Grotesk with a model trained beforehand. `H` then `eight`. Score against the other master. |
| 1:25 | What next | font-ml tasks that are not built yet are the list of things to build. Where to send a model. |

## Before the session

- Run every demo once on the presenter machine the day before.
- Run `./03-chat-model/get-model.sh` and `./03-chat-model/get-model.sh 4b`.
- Train `models/virtua-bolden` for demo 05.
- Have the eval tables from your runs in a text file in case a live run is slow.

## If something fails

- Training fails: `models/shapes-bolden` is checked in. Use it.
- No model in the editor: check `RUNEBENDER_MODELS` points at `models`, and that the model directory sits directly under it.
- `font-ml` missing: `~/.cargo/bin` is not on PATH.
- A GPU error: add `--device cpu` to the command, or set the same in the script.
- A download fails: the script resumes where it stopped; run it again.
