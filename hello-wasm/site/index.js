import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
    var ret_from_rust = js.greet("WebAssembly with npm");
    alert(ret_from_rust);

    /**
     * Part one: Write in Wasm, Read in JS
     */
     console.log("Write in Wasm, Read in JS, Index 0:");
  
     // First, let's have wasm write to our buffer
     js.store_value_in_wasm_memory_buffer_index_zero(24);
   
     // Next, let's create a Uint8Array of our wasm memory
     let wasmMemory = new Uint8Array(js.wasm_memory().buffer);
   
     // Then, let's get the pointer to our buffer that is within wasmMemory
     let bufferPointer = js.get_wasm_memory_buffer_pointer();
   
     // Then, let's read the written value at index zero of the buffer,
     // by accessing the index of wasmMemory[bufferPointer + bufferIndex]
     console.log(wasmMemory[bufferPointer + 0]); // Should log "24"
   
     /**
      * Part two: Write in JS, Read in Wasm
      */
     console.log("Write in JS, Read in Wasm, Index 1:");
   
     // First, let's write to index one of our buffer
     wasmMemory[bufferPointer + 1] = 15;
   
     // Then, let's have wasm read index one of the buffer,
     // and return the result
     console.log(js.read_wasm_memory_buffer_and_return_index_one()); // Should log "15"
   
  });
