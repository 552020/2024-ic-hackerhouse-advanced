from transformers import GPT2Tokenizer
import torch

# Load GPT-2 tokenizer
tokenizer = GPT2Tokenizer.from_pretrained("gpt2")


# Function to inspect tokens for a given text
def inspect_tokens(text):
    # Tokenize without converting to tensor first
    tokens = tokenizer.tokenize(text)
    # Get the token IDs
    token_ids = tokenizer.encode(text, return_tensors="pt")

    print(f"\nInput text: {text}")
    print(f"Tokens: {tokens}")
    print(
        f"Token IDs: {token_ids.tolist()[0]}"
    )  # Convert tensor to list for readability
    print(f"Decoded text: {tokenizer.decode(token_ids[0])}")


# Function to decode pre-defined token IDs
def decode_tokens(token_ids):
    decoded_text = tokenizer.decode(token_ids)
    print(f"\nToken IDs: {token_ids}")
    print(f"Decoded Text: {decoded_text}")


# Test with different input texts
print("Inspecting Tokenization:")
inspect_tokens("<|endoftext|>")  # Special GPT-2 token
inspect_tokens("Hi. How are you?")
inspect_tokens("Tell me a haiku about the stars.")

# Test decoding predefined responses
print("\nTesting Decoding of Predefined Token IDs:")
response_1 = [
    387,
    13,
    198,
    11,
    410,
    393,
    345,
    338,
    11,
    11,
    198,
    11,
    13,
]  # Arbitrary example
response_2 = [15902, 11, 198, 314, 13, 357, 46, 11, 198, 11, 198, 7, 11, 198]
decode_tokens(response_1)
decode_tokens(response_2)
