# 03: The same steps as a node graph

Demo 01 as a workflow file: boxes and wires that the editor draws in
its Nodes tab and that run from the shell. A workflow is a file you
can share, and it runs the same way for everyone.

Time: five minutes, including a training run.

## The files

| File | What it does |
| --- | --- |
| `shapes-train.nodes.json` | Font and Bold master into Train; the new model into Bolden over every glyph; Compare against Bold |
| `shapes-bolden.nodes.json` | The checked-in `shapes-bolden` model into Bolden; Compare; Install |

Node types come from `font-ml tasks`. The editor carries no task
names of its own, so a new task in `font-ml` is a new node with no
editor change.

## 1. Check and run from the shell

```sh
runebender nodes check 03-nodes/shapes-train.nodes.json
runebender nodes run 03-nodes/shapes-train.nodes.json \
    --font fonts/Shapes/Shapes.designspace --models models --json
```

`check` reads the file and reports every problem. `run` executes each
node in order, skipping any whose inputs have not changed since the
last run, and prints one JSON report. The Train node writes
`models/shapes-bolden-nodes`.

## 2. Open it in the editor

```sh
RUNEBENDER_MODELS=$PWD/models runebender fonts/Shapes/Shapes.designspace
```

In **Local AI**, under **Nodes**, the two files appear because they
sit beside the font. Click one to open the **Nodes** tab, then run it.
The graph shows each node's status as it goes.

## 3. Change it

Edit a value in the file, for example `steps` on the Train node, or
drag a wire in the editor. Run again. Only the nodes after the change
rerun.

`runebender nodes types` lists every node and its inputs and outputs.
