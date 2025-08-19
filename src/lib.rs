#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use serde::{Deserialize, Serialize};
use spin::Mutex;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// --- Input and Output Structures ---

#[derive(Serialize, Deserialize)]
struct CalculationRequest {
    a: i32,
    b: i32,
    text: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CalculationResponse {
    message: String,
    calculation_result: i32,
}

// --- Global variable to hold output ---
static OUTPUT_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// This is the main entry point for the Uomi agent.
#[no_mangle]
pub extern "C" fn run(ptr: *const u8, len: usize) {
    let input_slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    let input_json = core::str::from_utf8(input_slice).unwrap_or("");

    let request: CalculationRequest = serde_json::from_str(input_json).unwrap_or(CalculationRequest {
        a: 0,
        b: 0,
        text: Some("Default text".to_string()),
    });

    let result = request.a + request.b;
    let message_text = request.text.unwrap_or_else(|| "No text provided".to_string());

    let response = CalculationResponse {
        message: format!("Your meme message is '{}' and the result is:", message_text),
        calculation_result: result,
    };

    let result_string = serde_json::to_string(&response).unwrap();
    
    // Lock the buffer and update its contents
    *OUTPUT_BUFFER.lock() = result_string.into_bytes();
}

/// Returns a pointer to the output buffer.
#[no_mangle]
pub extern "C" fn get_output_ptr() -> *const u8 {
    OUTPUT_BUFFER.lock().as_ptr()
}

/// Returns the length of the output buffer.
#[no_mangle]
pub extern "C" fn get_output_len() -> usize {
    OUTPUT_BUFFER.lock().len()
}

/// Required panic handler for `no_std` environments.
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
