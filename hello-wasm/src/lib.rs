use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> i32 {
    alert(&format!("Hello, {}!", name));
    return 5;
}

const WASM_MEMORY_BUFFER_SIZE: usize = 2;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

// Function to store the passed value at index 0,
// in our buffer
#[wasm_bindgen]
pub fn store_value_in_wasm_memory_buffer_index_zero(value: u8) {
  unsafe {
    WASM_MEMORY_BUFFER[0] = value;
  }
}

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = WASM_MEMORY_BUFFER.as_ptr();
  }

  return pointer;
}

// Function to read from index 1 of our buffer
// And return the value at the index
#[wasm_bindgen]
pub fn read_wasm_memory_buffer_and_return_index_one() -> u8 {
  let value: u8;
  unsafe {
    value = WASM_MEMORY_BUFFER[1];
  }
  return value;
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}