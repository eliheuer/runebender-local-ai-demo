# Models

Every model is a directory in this folder. Git ignores the folder
except this file, `shapes-draw`, and `shapes-bolden`.

| Directory | Kind | Size | Where it comes from |
| --- | --- | --- | --- |
| `shapes-draw` | outline, draw | 22 MB | Checked in. Trained by `01-draw-from-text/train.sh` on `fonts/Shapes.ufo`, 15000 steps, 256 dims, 6 layers |
| `shapes-bolden` | outline, bolden | 5 MB | Checked in. Trained by `02-train-and-bolden/train.sh` on `fonts/ShapesFamily`, 1200 steps, 128 dims, 4 layers |
| `shapes-bolden-nodes` | outline, bolden | 5 MB | Written by `04-nodes/shapes-train.nodes.json` |
| `qwen3-0.6b` | chat, GGUF | 805 MB | `03-chat-model/get-model.sh`, from [ggml-org/Qwen3-0.6B-GGUF](https://huggingface.co/ggml-org/Qwen3-0.6B-GGUF) |
| `qwen3-1.7b` | chat, GGUF | 1.3 GB | `get-model.sh 1.7b`, from [ggml-org/Qwen3-1.7B-GGUF](https://huggingface.co/ggml-org/Qwen3-1.7B-GGUF) |
| `qwen3-4b` | chat, GGUF | 2.5 GB | `get-model.sh 4b`, from [ggml-org/Qwen3-4B-GGUF](https://huggingface.co/ggml-org/Qwen3-4B-GGUF) |
| `virtua-bolden` | outline, bolden | 50 MB | Trained by you in demo 05 |

Rule for this repo: a trained model under 100 MB may be checked in
when a demo depends on it. Anything larger is trained by a script here
or downloaded from a named URL with a checksum.

An outline model is `config.json`, `weights.safetensors`, and
`vocab.txt`; `font-ml describe <dir>` reads it. A chat model is
`model.gguf` and `tokenizer.json`. The editor reads both kinds from
`$RUNEBENDER_MODELS`, or `~/.runebender/models` when that is unset.
