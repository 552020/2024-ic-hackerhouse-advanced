# Advanced Challenge for ICP AI HackerHouse

Welcome to this tutorial that is for experienced developers looking how to deploy AI models on ICP, a decentralized cloud built on blockchain technology. Architected for being general purpose, the only limit you have is your imagination. 🙂🚀

On this challenge, the main goals are:

- to introduce to IC, in case you don't know yet, showcasing great examples of what is already possible
- to help you understand the process of building a model and deploying it on chain
- to hopefully convince you the IC stack and ecosystem is a great place to have a Dev career or co-found a startup.

## Challenge Overview

This challenge will be in Rust. We have prepared a Codespace for you, so you don't waste time on local setup and can go directly to the fun part, deploying a model on chain and coding! ⌨️

In this advanced challenge you will:

- Understand the "Python to Rust" toolkit for AI models.
- Build, upload the model to a canister and intereact with it.
- Build a tokenizers tool and a UI that allows humans to interact with GPT2.

The idea is to be very open (since it's an Advanced Challenge), giving you more room on how to implement it.

## Submission Details & Requirements

For the 150 (advanced challenge) ckUSDT prize, you will need to:

- Join the Taikai platform (where we publicly handle all the submissions): https://taikai.network/icp-portugal/hackathons/ICP-AI-HACKERHOUSE
- Create a project, following the instructions on the video, namely:
  - Title saying "Advanced Challenge - Your Name"
  - Add your github that allows to see the code finishing the challenge.
  - A video recording of the Candid UI (if you only implemented backend) or the Frontend (in case you implemented it) showing the dApp allowing a human to interact with the GPT2 inference method.

## Tutorial Videos

Tutorial Videos explaining and walking through the different parts of the challenge.
Note: Feel free to listen at 1.2x speed 😛

IC Overview (recorded on a recent workshop with CS Students):

- https://www.youtube.com/watch?v=wyHAh9i1cFI

Advanced:

- Intro to Advanced Challenge and Submission (https://www.loom.com/share/750eda01b36a4b3998b75973f7830e0d)

- Github and Codespace setup
- Overview of the Toolkit and how it's structured in this repo
- Deploy GPT2 100% on chain (https://www.loom.com/share/141610561e9549748199dac2f6f9ac5a)

- Develop your Tokenizers solution (GPT-2's Byte Pair Encoding (BPE) tokenizer) (explained on Intro Video)

# General Instructions / Commands

## Getting Started

If you prefer running VS Code locally and not in the browser, click "Codespaces: ..." in the bottom left corner and select "Open in VS Code" in the menu that appears.
If prompted, proceed by installing the recommended plugins for VS Code.

### Running your Project

After the IDE has opened, run `dfx deploy` in the terminal to deploy the frontend and backend.
Click on the first green link at the end of the output to see your canister's frontend in the browser.
To interact with the backend canister, click on the second green link.
**NOTE**: When developing in GitHub Codespaces, run `./scripts/canister_urls.py` and use the links that are shown there.

For interactive development of the frontend canister, you can also start a node server by running `npm start`.
You can find your canister's frontend running under http://localhost:8080.

If you make changes to the backend canister, remember to call `dfx deploy` first; it suffices to reload the frontend canister's webpage to reflect the changes you've made.
If your environment was restarted or has been inactive over some time, you might need to run `dfx start` before running `dfx deploy`.

## Documentation and Guides

To learn more before you start working on this project, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands to familiarize yourself with `dfx`:

```bash
dfx help
dfx canister --help
```

# COPY from rust-connect-py-ai-to-ic

https://github.com/modclub-app/rust-connect-py-ai-to-ic/blob/main/README.md

Seamlessly Bridge Python AI Models with the Internet Computer.

## Overview

"Python (design and train), Rust (upload), IC (run)" - this succinct mantra captures the essence of `rust-connect-py-ai-to-ic`. Our toolkit is an open-source solution designed to enable AI researchers and developers to effortlessly upload and deploy Python-based machine learning models for inference on the Internet Computer (IC) platform.

Focused on ease of use and versatility, `rust-connect-py-ai-to-ic` aims to streamline the integration of advanced AI capabilities into the decentralized environment of the IC. This project represents a pivotal step towards harnessing the power of the IC for AI, with potential future expansions into model training and user-friendly drag-and-drop functionalities.

## Features

- **Effortless Upload**: Simplify the process of uploading Python-based AI models to the IC using Rust.
- **Inference on IC**: Run your machine learning models on a decentralized platform, leveraging the unique capabilities of the Internet Computer.
- **Future Expansion**: Potential for extending the toolkit to include model training and easy-to-use drag-and-drop features.

## Getting Started

This section guides you through the initial setup of the necessary tools and environments for this project. We are using Rust with WebAssembly, Python with PyTorch, and the Dfinity platform.

### Rust Setup

First, ensure you have Rust installed. We will then set the default toolchain to stable and add the WebAssembly target.

1. Install "wasm-opt" and wait (average time ~12 minutes)
   ```bash
   cargo install wasm-opt
   ```
2. Deploy single_call canister

   ```bash
   # In one tab
   dfx start

   # In another tab
   dfx deploy

   # you can check the right links, by running script
   ./scripts/canister_urls.py
   ```

3. Upload model
   ```bash
   ic-file-uploader icp_gpt2 append_model_bytes src/icp_gpt2/lib/models/gpt2_with_kv.onnx
   dfx canister call icp_gpt2 setup_model
   ```
4. Call your 100% on-chain model
   ```bash
   dfx canister call icp_gpt2 model_inference '(14, vec {1; 2; 3; 4; 5; 6; 7; 8; 9; 10; 11; 12})'
   ```

### Python and PyTorch Setup

Ensure you have Python installed and then set up PyTorch.

1. Install Python (if not already installed): Visit [Python's download page](https://www.python.org/downloads/).
2. Install PyTorch using pip:
   ```bash
   pip install torch
   ```

### Dfinity's DFX Setup

We will be using Dfinity's `dfx` for our development environment.

1. Install Dfinity's `dfx`: Follow the instructions on [Dfinity's SDK documentation](https://sdk.dfinity.org/docs/quickstart/quickstart.html).

## Contributing

We welcome contributions! Please read our contributing guidelines to get started.

## License

Apache 2.0/MIT
All original work licensed under either of

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT) at your option.

## Notes

Here's a Markdown document based on the provided information:

````markdown
# Uploading Machine Learning Models to an Internet Computer (IC) Canister

This guide explains how to use the `ic-file-uploader` tool to upload a machine learning model file to an IC canister. The command breakdown is as follows:

## Command Example

```bash
ic-file-uploader icp_gpt2 append_model_bytes src/icp_gpt2/lib/models/gpt2_with_kv.onnx
```
````

### Breaking It Down:

1. **`ic-file-uploader`**  
   This is the tool designed to upload large files, like machine learning models, to IC canisters. It handles chunking the file to bypass IC's message size limitations.

2. **`icp_gpt2`**  
   This is the name of your target canister (smart contract) where the model will be stored.

3. **`append_model_bytes`**  
   This is the method in the canister being invoked to append the model data in chunks.

4. **`src/icp_gpt2/lib/models/gpt2_with_kv.onnx`**  
   This is the file path to the GPT-2 model that has been converted to ONNX format.

---

## Purpose of the Command

This command is used to:

1. Take a GPT-2 model that has been converted to ONNX format.
2. Upload the model in chunks to your IC canister.
3. Enable the model to be used for inference directly on the Internet Computer.

---

## Why Use `ic-file-uploader`?

The Internet Computer (IC) has message size limitations for canisters. Large files, like machine learning models, cannot be uploaded in a single message. The `ic-file-uploader` tool solves this by:

- Splitting the file into chunks.
- Appending these chunks to the canister using the `append_model_bytes` method.

---

## Post-Upload Step

After uploading the file, you need to initialize the uploaded model for use. Run the following command to complete the setup:

```bash
dfx canister call icp_gpt2 setup_model
```

This will configure the model within the canister for inference tasks.

---

## Example Workflow

1. **Prepare the Model**  
   Convert your GPT-2 model to ONNX format and place it in the appropriate path.

2. **Upload the Model**  
   Run the `ic-file-uploader` command to upload the model in chunks.

3. **Initialize the Model**  
   Use `dfx canister call` to initialize the uploaded model for use.

---

## Notes

- Ensure that the `ic-file-uploader` tool is installed and properly configured.
- Verify the file path and canister name before executing the command.
- Large files may take some time to upload, depending on the chunk size and network speed.

This process enables you to leverage machine learning models directly on the Internet Computer, making it possible to use these models in decentralized applications.

```

```
