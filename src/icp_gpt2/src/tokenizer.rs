use anyhow::{anyhow, Result};
use std::cell::RefCell;
use tokenizers::Tokenizer;

thread_local! {
    static TOKENIZER: RefCell<Option<Tokenizer>> = RefCell::new(None);
}

pub fn setup_tokenizer_from_bytes(bytes: &[u8]) -> Result<()> {
    let tokenizer =
        Tokenizer::from_bytes(bytes).map_err(|e| anyhow!("Failed to create tokenizer: {}", e))?;

    TOKENIZER.with(|t| {
        *t.borrow_mut() = Some(tokenizer);
    });

    Ok(())
}

pub fn encode(text: &str) -> Result<Vec<i64>> {
    TOKENIZER.with(|t| {
        let tokenizer = t.borrow();
        let tokenizer = tokenizer
            .as_ref()
            .ok_or_else(|| anyhow!("Tokenizer not initialized"))?;

        let encoding = tokenizer
            .encode(text, false)
            .map_err(|e| anyhow!("Failed to encode text: {}", e))?;
        Ok(encoding.get_ids().iter().map(|&id| id as i64).collect())
    })
}

pub fn decode(tokens: &[i64]) -> Result<String> {
    TOKENIZER.with(|t| {
        let tokenizer = t.borrow();
        let tokenizer = tokenizer
            .as_ref()
            .ok_or_else(|| anyhow!("Tokenizer not initialized"))?;

        let tokens: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        tokenizer
            .decode(&tokens, true)
            .map_err(|e| anyhow!("Failed to decode tokens: {}", e))
    })
}
