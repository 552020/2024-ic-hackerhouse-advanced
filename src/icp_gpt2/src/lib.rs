//use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
//use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap}; //, Storable};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl};

use std::cell::RefCell;

//type Memory = VirtualMemory<DefaultMemoryImpl>;
mod onnx;
mod storage;
mod tokenizer;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    //pub static MAP: RefCell<StableBTreeMap<u8, Vec<u8>, Memory>> = RefCell::new(
    //    StableBTreeMap::init(
    //        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
    //    )
    //);
	static TOKENIZER_BYTES: RefCell<Vec<u8>> = RefCell::new(Vec::new());

    static PROCESSING_STATE: RefCell<usize> = RefCell::new(0);

    static CLEAR_STATE: RefCell<bool> = RefCell::new(false);
}

#[ic_cdk::init]
fn init() {
    // Initialize the WASI memory
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);

    // Initialize the application memory (StableBTreeMap)
    //let app_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)));
    //MAP.with(|map| {
    //    *map.borrow_mut() = StableBTreeMap::init(app_memory);
    //});A
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    // Save any necessary state before upgrade if needed
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    // Reinitialize the WASI memory after upgrade
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);

    // Reinitialize the application memory (StableBTreeMap) after upgrade
    //let app_memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)));
    //MAP.with(|map| {
    //    *map.borrow_mut() = StableBTreeMap::init(app_memory);
    //});
}


//////////////////////////////////////////////////////////////////////



const MODEL_FILE: &str = "onnx_model.onnx";

/// Clears the face detection model file in chunks.
#[ic_cdk::update]
fn clear_model_bytes() -> Option<String> {
    let is_clearing = CLEAR_STATE.with(|state| *state.borrow());
    
    if !is_clearing {
        // Start the clearing process
        CLEAR_STATE.with(|state| *state.borrow_mut() = true);
        storage::clear_bytes(MODEL_FILE);
        CLEAR_STATE.with(|state| *state.borrow_mut() = false);
        Some("Model bytes cleared successfully".to_string())
    } else {
        Some("Clearing already in progress".to_string())
    }
}

// Add a helper function to check the clearing status
#[ic_cdk::query]
fn get_clear_status() -> bool {
    CLEAR_STATE.with(|state| *state.borrow())
}

/// Appends the given chunk to the face detection model file.
/// This is used for incremental chunk uploading of large files.
#[ic_cdk::update]
fn append_model_bytes(bytes: Vec<u8>) {
    storage::append_bytes(MODEL_FILE, bytes);
}

/// Returns the length of the model bytes.
#[ic_cdk::query]
fn model_bytes_length() -> usize {
    storage::bytes_length(MODEL_FILE)
}

#[ic_cdk::update]
fn append_tokenizer_bytes(bytes: Vec<u8>) {
    TOKENIZER_BYTES.with(|b| {
        b.borrow_mut().extend(bytes);
    });
}

#[ic_cdk::update]
fn setup_tokenizer() -> Option<String> {
    // Reduce chunk size even further
    let max_chunk_size = 1; // Reduced from 5 to process just 1 byte at a time
    
    let bytes = TOKENIZER_BYTES.with(|b| b.borrow().clone());
    let current_position = PROCESSING_STATE.with(|state| *state.borrow());
    
    // Add early return if we've processed everything
    if current_position >= bytes.len() {
        PROCESSING_STATE.with(|state| *state.borrow_mut() = 0);
        return None;
    }
    
    let end = std::cmp::min(current_position + max_chunk_size, bytes.len());
    let chunk = &bytes[current_position..end];
    
    // Process just one byte and return immediately
    match tokenizer::setup_tokenizer_from_bytes(chunk) {
        Ok(_) => {
            PROCESSING_STATE.with(|state| *state.borrow_mut() = end);
            Some(format!("Processing byte {}/{} bytes. Progress: {}%", 
                end, 
                bytes.len(), 
                (end * 100) / bytes.len()))
        },
        Err(e) => Some(format!("Error processing chunk: {}", e))
    }
}

// Add a helper function to check progress
#[ic_cdk::query]
fn get_tokenizer_setup_progress() -> (usize, usize) {
    let total = TOKENIZER_BYTES.with(|b| b.borrow().len());
    let processed = PROCESSING_STATE.with(|state| *state.borrow());
    (processed, total)
}
