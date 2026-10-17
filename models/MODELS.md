# Models

Every model is a directory. None is in git: a model's weights change
with every retrain and git keeps every version forever. Each row says
where the model comes from instead.

| Directory | Kind | Size | Where it comes from |
| --- | --- | --- | --- |
| `~/Runebender/models/shapes/shapes-base` | shapes (draw, box format) | 21 MB | `01-draw-from-text/get-base-model.sh` downloads it from this repo's [models-v1 release](https://github.com/eliheuer/runebender-local-ai-demo/releases/tag/models-v1) with a checksum. Or train it with `train.sh`: 15000 steps, 16 spare word slots |
| `shapes-bolden` | outline, bolden | 5 MB | Trained by `02-train-and-bolden/train.sh` on `fonts/ShapesFamily`, 1200 steps, 128 dims, 4 layers, about a minute |
| `shapes-bolden-nodes` | outline, bolden | 5 MB | Written by `04-nodes/shapes-train.nodes.json` |
| `qwen3-0.6b` | chat, GGUF | 805 MB | `03-chat-model/get-model.sh`, from [ggml-org/Qwen3-0.6B-GGUF](https://huggingface.co/ggml-org/Qwen3-0.6B-GGUF) |
| `qwen3-1.7b` | chat, GGUF | 1.3 GB | `get-model.sh 1.7b`, from [ggml-org/Qwen3-1.7B-GGUF](https://huggingface.co/ggml-org/Qwen3-1.7B-GGUF) |
| `qwen3-4b` | chat, GGUF | 2.5 GB | `get-model.sh 4b`, from [ggml-org/Qwen3-4B-GGUF](https://huggingface.co/ggml-org/Qwen3-4B-GGUF) |
| `virtua-bolden` | outline, bolden | 50 MB | Trained by you in demo 05 |

Rule for this repo: no model weights in git. A model is trained by a
script here or downloaded from a named URL with a checksum. The base
will move to Hugging Face under the `fontgarden` account once it is
good enough to publish.

An outline model is `config.json`, `weights.safetensors`, and
`vocab.txt`; `font-ml describe <dir>` reads it. A chat model is
`model.gguf` and `tokenizer.json`. The editor reads both kinds from
`$RUNEBENDER_MODELS`, or `~/.runebender/models` when that is unset.
