# Build tokenizer

## **Understanding GPT-2 Tokenization**

- GPT-2 uses **Byte Pair Encoding (BPE)**, which:

  1. Converts text to a sequence of subword tokens based on frequency in the vocabulary.
  2. Maps each token to an ID, which is the input format the model expects.

- The tokenizer must:
  1. **Preprocess the input text** (e.g., handle special characters, lowercase, etc.).
  2. **Break the text into subwords/tokens** using the BPE vocabulary.
  3. **Return token IDs** to send as input to the GPT-2 model.

---

## **Plan for Tokenizer Implementation**

1. **Use Hugging Face's Pretrained GPT-2 Vocabulary**:

   - The GPT-2 vocabulary (`vocab.json`) and merges file (`merges.txt`) define the BPE rules.
   - These can be downloaded from Hugging Face:
     - [vocab.json](https://huggingface.co/gpt2/raw/main/vocab.json)
     - [merges.txt](https://huggingface.co/gpt2/raw/main/merges.txt)

2. **Load Vocabulary and Merge Rules in Rust**:

   - Parse `vocab.json` (maps tokens to IDs).
   - Parse `merges.txt` (defines the BPE merging rules).

3. **Tokenize Input Text**:

   - Convert text into byte-level encoding.
   - Apply the BPE merging rules to generate tokens.
   - Map tokens to their corresponding IDs.

4. **Expose the Tokenizer in the Canister**:
   - Add a `tokenize` function in the canister to return token IDs for a given input text.

---

## **Step-by-Step Implementation**

### **Step 1: Add Tokenizers Library**

- The easiest way to implement a GPT-2 tokenizer in Rust is to use the [Hugging Face Tokenizers](https://github.com/huggingface/tokenizers) library.
- Add the library to your `Cargo.toml`:
  ```toml
  [dependencies]
  tokenizers = "0.13.3"
  ```

---

### **Step 2: Initialize the Tokenizer**

- Download the `vocab.json` and `merges.txt` files from Hugging Face.
- Save them in the `src/tokenizer` directory of your project.

- Write a helper function to initialize the tokenizer:

  ```rust
  use tokenizers::Tokenizer;

  pub fn initialize_tokenizer() -> Tokenizer {
      Tokenizer::from_file("src/tokenizer/vocab.json").expect("Failed to load tokenizer")
  }
  ```

---

### **Step 3: Implement the Tokenize Function**

- Write a Rust function to tokenize input text:

  ```rust
  use tokenizers::Tokenizer;

  pub fn tokenize_input(input: String) -> Vec<u32> {
      // Load the GPT-2 tokenizer
      let tokenizer = initialize_tokenizer();

      // Tokenize the input string
      let encoding = tokenizer.encode(input, true).expect("Failed to tokenize input");

      // Return the token IDs
      encoding.get_ids().to_vec()
  }
  ```

- Test the function locally:
  ```rust
  fn main() {
      let input = "Hello world!".to_string();
      let tokens = tokenize_input(input);
      println!("Token IDs: {:?}", tokens);
  }
  ```

---

### **Step 4: Add the Tokenizer to the Canister**

1. **Expose the `tokenize` Function**:

   - Add the `tokenize_input` function to the canister interface:
     ```rust
     #[query]
     fn tokenize(input: String) -> Vec<u32> {
         tokenize_input(input)
     }
     ```

2. **Update the Candid File**:

   - Add the `tokenize` method to your Candid interface (`icp_gpt2.did`):
     ```candid
     service : {
         tokenize: (text) -> (vec nat) query;
     }
     ```

3. **Rebuild and Redeploy the Canister**:

   - Build the canister:
     ```bash
     dfx build
     ```
   - Deploy it:
     ```bash
     dfx deploy
     ```

4. **Test the Tokenizer**:
   - Call the `tokenize` function via the command line:
     ```bash
     dfx canister call icp_gpt2 tokenize '("Hello world!")'
     ```
   - Expected output (example):
     ```bash
     (vec {15496; 995; 0})
     ```

---

### **Step 5: Integrate Tokenizer with GPT-2 Inference**

Once the tokenizer is working:

1. Use it to preprocess input text in Rust before sending it to the GPT-2 model.
2. Modify the `model_inference` function to include the tokenization step:
   ```rust
   #[query]
   fn model_inference(input: String) -> Vec<i64> {
       let tokens = tokenize_input(input);
       let result = gpt2_infer(tokens);
       result
   }
   ```

---

### **Step 6: Optional Frontend for Interaction**

Build a simple frontend for users to enter text, tokenize it, and display the GPT-2 response.

1. **Tokenize Input**:

   - Use the backend's `tokenize` function to tokenize the user input.

2. **Perform Inference**:

   - Call the `model_inference` function with the tokenized input.

3. **Display Results**:
   - Show the generated response from GPT-2.

---

### **Challenges and Workarounds**

1. **Tokenization in Rust**:

   - Rust can be verbose for handling JSON parsing (e.g., `vocab.json` and `merges.txt`). Use existing libraries like `serde_json` to simplify this.

2. **Testing BPE Rules**:

   - Ensure your BPE merges are applied correctly. Compare the output of your tokenizer with Hugging Face’s tokenizer in Python.

3. **Fallback to Python Tokenizer**:
   - If Rust tokenization becomes a bottleneck, consider implementing tokenization in Python and passing the tokenized input to Rust.

---

### **Next Steps**

1. Implement the tokenizer using the steps above.
2. Test the entire pipeline: text → tokens → GPT-2 → generated tokens → text.
3. If you're stuck at any stage (e.g., BPE implementation in Rust), let me know—I can provide additional support or alternative solutions.

How confident are you about the Rust implementation so far? Should we dive deeper into any specific aspect, like BPE rules or integration with the model?
