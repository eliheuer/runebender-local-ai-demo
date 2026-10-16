# 02: A chat model from Hugging Face

You download a small open-weight language model and ask it about the
font. The model runs on your machine through `font-ml`. It can read
the font, draw proofs, and propose edits. It cannot change the font
unless you install what it proposes.

Time: five minutes plus the download.

## 1. Get the model

```sh
./02-chat-model/get-model.sh
```

This fetches Qwen3-0.6B as a GGUF file, plus its tokenizer, from
Hugging Face into `models/qwen3-0.6b`, and checks both against known
SHA-256 sums. It is about 800 MB.

A chat model directory is two files:

| File | What it is |
| --- | --- |
| `model.gguf` | The quantized weights |
| `tokenizer.json` | How text becomes tokens |

That is the whole install. There is no account, no API key, and no
service.

## 2. Talk to it from the shell

```sh
font-ml chat --model models/qwen3-0.6b --font fonts/Shapes/Shapes.designspace \
    --prompt 'Use the tools to read this font. How many glyphs does it have?'
```

The output is a stream of JSON events: tokens as they arrive, each
tool call the model makes, and the final messages. Watch for a
`project_info` or `font_info` call followed by a tool result: that is
the model reading the font.

## 3. Talk to it in the editor

```sh
RUNEBENDER_MODELS=$PWD/models runebender fonts/Shapes/Shapes.designspace
```

Open the **Chat** tab in the left panel, pick `qwen3-0.6b`, and ask it
to read a glyph and make a proof.

## 4. Judge it

A 0.6B model is enough to show the loop and not enough to be useful.
In our runs it called the right tool and then misread the answer:
asked how many glyphs the font has, it read a result that said 48 and
answered 8. Expect that. The larger sizes do better:

```sh
./02-chat-model/get-model.sh 1.7b    # 1.3 GB
./02-chat-model/get-model.sh 4b      # 2.5 GB, what the Chat panel was built with
```

The 4B model needs about 4 GB of memory to run and is the one to use
for a real session. Everything else is the same: the same folder
layout, the same tools, the same editor.

## Where models come from

Chat models are the one kind of model in this repo that you download
rather than train. They are general models; nothing about them knows
fonts. What they know about your font, they read through the tools at
run time. Outline models, the kind demo 01 trains, are specific to
font sources, and today there are none on Hugging Face that fit
Runebender's format. You train those yourself.
