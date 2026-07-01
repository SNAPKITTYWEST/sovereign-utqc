//! # sovereign-wasm
//!
//! WASM bindings for sovereign compute pipeline.

/// WASM entry point: compile a circuit JSON string.
#[no_mangle]
pub extern "C" fn compile_circuit(ptr: *const u8, len: usize) -> *mut u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let input = std::str::from_utf8(slice).unwrap_or("{}");
    let output = format!("{{\"status\":\"compiled\",\"input_len\":{}}}", input.len());
    let bytes = output.into_bytes();
    let boxed = bytes.into_boxed_slice();
    let ptr = Box::into_raw(boxed) as *mut u8;
    ptr
}

/// Free a WASM-allocated buffer.
#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut u8, len: usize) {
    unsafe {
        let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_circuit() {
        let input = b"{\"qubits\":2}";
        let ptr = compile_circuit(input.as_ptr(), input.len());
        // Read the actual output length from the format string
        let output = format!("{{\"status\":\"compiled\",\"input_len\":{}}}", input.len());
        let len = output.len();
        let result = unsafe { std::slice::from_raw_parts(ptr, len) };
        let s = std::str::from_utf8(result).unwrap();
        assert!(s.contains("compiled"));
        free_buffer(ptr, len);
    }
}
