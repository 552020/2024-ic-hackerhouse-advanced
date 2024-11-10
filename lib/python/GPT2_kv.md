# **GPT-2 Key-Value (KV) Attention Manipulation and ONNX Export**

This script demonstrates advanced interaction with a GPT-2 model, focusing on:

1. **Using and manipulating past key-values** for efficient text generation.
2. **Exporting the GPT-2 model to ONNX format** for optimized inference.
3. **Inspecting GPT-2’s attention mechanism and iterative text generation process.**

---

## **1. Key Concepts in the Script**

### **1.1 Past Key-Values (KV)**

- GPT-2 uses past key-values to store intermediate computation results from previous tokens.
- This enables **faster generation** by avoiding re-computation for earlier tokens.

### **1.2 Text Generation**

- Input text is tokenized and passed to the GPT-2 model.
- The script demonstrates **iterative generation**, where:
  1. Tokens are generated one at a time.
  2. Past key-values and attention masks are updated iteratively.

### **1.3 ONNX Export**

- ONNX (Open Neural Network Exchange) allows deploying models in environments optimized for inference (e.g., on-chain, edge devices).
- The script exports the GPT-2 model with key-value caching.

---

## **2. Key Sections of the Script**

### **2.1 Model Initialization**

The script loads a pre-trained GPT-2 model and tokenizer:

```python
from transformers import GPT2LMHeadModel, GPT2Tokenizer

model = GPT2LMHeadModel.from_pretrained("gpt2")
tokenizer = GPT2Tokenizer.from_pretrained("gpt2")
```

### **2.2 Initial Input Encoding and Model Run**

- Input text (`"What is your favorite"`) is tokenized.
- The initial run generates:
  - **Logits** (probabilities for the next token).
  - **Past Key-Values** (intermediate results for re-use).

```python
inputs = tokenizer(input_text, return_tensors="pt")
outputs = model(input_ids, attention_mask=attention_mask, use_cache=True)
past_key_values = outputs.past_key_values
```

---

### **2.3 Iterative Text Generation**

The script generates additional tokens step by step:

- **New input tokens** (e.g., `"sport?"`) are tokenized.
- **Attention masks and past key-values** are updated and reused to improve efficiency.
- The script generates 15 tokens in a loop:

```python
for i in range(15):
    outputs = model(new_input_ids, past_key_values=past_key_values, use_cache=True)
    next_token = torch.argmax(outputs.logits[:, -1, :]).item()
    output_ids.append(next_token)
```

#### **Example Generated Output**:

```plaintext
What is your favorite sport?
```

---

### **2.4 Model Wrapper for ONNX Export**

A `GPT2Wrapper` class is created to manage:

1. **Custom key-value handling**.
2. **Freezing the model’s parameters for inference-only use.**
3. **Exporting to ONNX format** for efficient deployment.

```python
torch.onnx.export(
    wrapper,
    (new_input_ids, extended_attention_mask, past_key_values_flat),
    "gpt2_with_kv.onnx",
    ...
)
```

---

## **3. Output Walkthrough**

### **3.1 Initial Outputs**

- **Logits**: The model predicts probabilities for the next token.
- **Past Key-Values**: Intermediate states stored in a tuple, reused for faster inference.

### **3.2 Iterative Outputs**

Generated tokens and corresponding text:

```plaintext
Iteration 0: Generated token: 198 -> "\n"
Iteration 1: Generated token: 2061 -> "What"
Iteration 2: Generated token: 318 -> " is"
...
Iteration 6: Generated token: 30 -> "?"
```

---

## **4. ONNX Model Verification**

The exported ONNX model is verified using `onnxruntime`:

1. **Load the model**:
   ```python
   ort_session = ort.InferenceSession("gpt2_with_kv.onnx")
   ```
2. **Run inference** with inputs including `past_key_values`.
3. Verify the output token and updated past key-values:
   ```python
   outputs = ort_session.run(None, onnx_inputs)
   print(f"Output ID: {outputs[0]}")
   ```

---

## **5. Applications**

- **Efficient Text Generation**:
  - Leverages past key-values to generate tokens iteratively without recomputing previous states.
- **Deployable Models**:
  - The ONNX-exported GPT-2 can be deployed in production environments (e.g., ICP or cloud).
- **Low-Latency Inference**:
  - Optimized for fast token generation in real-time applications.

---

## **6. Notes and Considerations**

1. **Attention Mask**:

   - Dynamically updated to include past tokens for accurate context.
   - Important for handling new input sequences.

2. **Past Key-Values**:

   - Reduces computation by reusing cached intermediate results.
   - Key for iterative token generation in constrained environments.

3. **ONNX Compatibility**:
   - ONNX export requires specific input-output mapping (handled via `dynamic_axes`).
   - Careful attention to tensor shapes is necessary for compatibility.

---

Would you like me to expand any specific section or clarify further details about this script?
