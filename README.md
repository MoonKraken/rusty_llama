<picture>
<img src="https://raw.githubusercontent.com/Me163/rusty_llama/main/demo.gif" />
</picture>

# Rusty Llama Webapp
A simple webapp to showcase the ability to write a simple chatbot webapp using only Rust, TailwindCSS and an Open Source language model such as a variant of GPT, LLaMA, etc.

## Setup Instructions

### Hardware
By default, the project has Apple's Metal acceleration enabled. If you are not on a macOS system, you may need to disable the `metal` feature in `Cargo.toml`. Similarly, if you are on a system with an Nvidia GPU, you may need to add CUDA as a feature (I haven't tested this, anyone who does so feel free to PR an update to this readme).

### Rust Toolchain
You'll need to use the nightly Rust toolchain, and install the `wasm32-unknown-unknown` target as well as the Trunk and `cargo-leptos` tools:
```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```
### Model
You'll also need to download a model (in GGML format) of your choice that is [supported by the Rustformers/llm Crate](https://huggingface.co/models?search=ggml).

In the root of the project directory, you'll find a `.env` file where an environment variable called `MODEL_PATH` is defined. Replace the value with the full path to the desired model file.

### TailwindCSS
Install TailwindCSS with `npm install -D tailwindcss`

### Run
To run the project locally, 
1. run `npx tailwindcss -i ./input.css -o ./style/output.css --watch` in a terminal - this will build `style/output.css` and automatically rebuild when a change is detected in `input.css`
1. `cargo leptos watch` in the project directory. 
1. In in your browser, navigate to [http://localhost:3000/?](http://localhost:3000/?)

## Tested Models
* [Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML)
* [Wizard-Vicuna-7B-Uncensored.ggmlv3.q4_K_S.bin](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML)
* [Wizard-Vicuna-30B-Uncensored.ggmlv3.q2_K.bin](https://huggingface.co/TheBloke/Wizard-Vicuna-30B-Uncensored-GGML)

<picture>
<img src="https://raw.githubusercontent.com/Me163/rusty_llama/main/metal_llama.png" />
</picture>
