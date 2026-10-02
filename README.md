<picture>
<img src="https://raw.githubusercontent.com/Me163/rusty_llama/main/demo.gif" />
</picture>

# Rusty Llama Webapp
A simple webapp to showcase the ability to write a simple chatbot webapp using only Rust, TailwindCSS and an Open Source language model such as a variant of GPT, LLaMA, etc.

Inference is handled in-process by [mistral.rs](https://github.com/EricLBuehler/mistral.rs). The UI is a [Leptos](https://leptos.dev) 0.8 app served by Actix.

## Setup Instructions

### Hardware
By default, the project enables Apple's Metal backend through the `metal` feature on the `mistralrs` dependency in `Cargo.toml`. On an NVIDIA machine, replace `metal` with `cuda`. For a CPU-only build, remove the feature list.

The Metal feature compiles GPU shaders while the server crate builds. Xcode 26 and later do not ship that compiler inside Xcode. It is a separate component, the Metal Toolchain. Install it once:

```
xcodebuild -runFirstLaunch
xcodebuild -downloadComponent MetalToolchain
xcrun metal --version
```

`xcodebuild -runFirstLaunch` updates Xcode's system packages and asks for an administrator password. Let it finish. `xcrun metal --version` should then print a version. Until it does, `cargo leptos watch` stops while compiling mistral.rs shaders.

### Rust Toolchain
Stable Rust 1.88 or newer is required, along with the `wasm32-unknown-unknown` target and `cargo-leptos`:

```
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

### Model
The default is [Qwen3.8 27B](https://huggingface.co/bartowski/Qwen3.8-27B-GGUF), the `Q4_K_M` GGUF (about 17 GB). `.env` sets `MODEL_REPO` and `MODEL_FILE`. mistral.rs downloads that file from Hugging Face into `~/.cache/huggingface` the first time the server starts, then reuses the cache. No separate download step is required. A Hugging Face token is only needed for gated repos. This one is public.

To use a different quant, change `MODEL_FILE` to another filename in that repo, such as `Qwen3.8-27B-Q6_K.gguf`. To use a file you already have, set `MODEL_PATH` to a `.gguf` file or a directory of GGUF shards. `MODEL_PATH` overrides the Hugging Face settings. Older GGML `.bin` files are not supported.

If a file has no embedded chat template, set `CHAT_TEMPLATE` to a Jinja template string or a path to a `.json` template file.

### Run
To run the project locally,

1. `cargo leptos watch` in the project directory. This compiles Tailwind from `input.css` into the site CSS.
1. In your browser, open [http://localhost:3000/](http://localhost:3000/).

The first server build compiles mistral.rs and its Metal kernels, so it takes a while.

## Example Models
* [Qwen3.8-27B-Q4_K_M.gguf](https://huggingface.co/bartowski/Qwen3.8-27B-GGUF) (default)
* [Qwen3.8-27B-Q6_K.gguf](https://huggingface.co/bartowski/Qwen3.8-27B-GGUF) (higher quality, about 24 GB)

<picture>
<img src="https://raw.githubusercontent.com/Me163/rusty_llama/main/metal_llama.png" />
</picture>
