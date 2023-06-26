<picture>
<img src="https://raw.githubusercontent.com/Me163/rusty_llama/main/screenshot.png" width="300" hspace="10"/>
<img src="https://raw.githubusercontent.com/Me163/rusty_llama/main/metal_llama.png" height="300" hspace="10"/>
</picture>

# Rusty Llama Webapp
A simple webapp to showcase the ability to write a simple chatbot webapp using only Rust, TailwindCSS and an Open Source language model such as a variant of GPT, LLaMA, etc.

## Setup Instructions
You'll need to use the nightly Rust toolchain, and install the `wasm32-unknown-unknown` target as well as the Trunk and `cargo-leptos` tools:
```
rust toolchain install nightly
rust target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```

You'll also need to download a model (in GGML format) of your choice that is [supported by the Rustformers/llm Crate](https://huggingface.co/models?search=ggml).

In the root of the project directory, you'll find a `.env` file where an enviroment variable called `MODEL_PATH` is defined. Replace the value with the full path to the desired model file.

To run the project locally, `cargo leptos watch` in the project directory. Then in your browser navigte to [http://localhost:3000/?](http://localhost:3000/?)

## Tested Models

* [Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML)
