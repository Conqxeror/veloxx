//! SIMD-optimized string comparison for fast group-by and filtering

#[cfg(all(feature = "simd", not(target_arch = "wasm32")))]
pub fn simd_eq_str(a: &str, b: &str) -> bool {
    use std::arch::x86_64::*;
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    
    // First check if lengths are equal
    if a_bytes.len() != b_bytes.len() {
        return false;
    }
    
    let len = a_bytes.len();
    let mut i = 0;
    
    // Compare 16-byte chunks with SIMD
    while i + 16 <= len {
        unsafe {
            let a_chunk = _mm_loadu_si128(a_bytes[i..].as_ptr() as *const __m128i);
            let b_chunk = _mm_loadu_si128(b_bytes[i..].as_ptr() as *const __m128i);
            let cmp = _mm_cmpeq_epi8(a_chunk, b_chunk);
            if _mm_movemask_epi8(cmp) != 0xFFFF {
                return false;
            }
        }
        i += 16;
    }
    
    // Fallback for remaining bytes
    a_bytes[i..] == b_bytes[i..]
}

#[cfg(not(all(feature = "simd", not(target_arch = "wasm32"))))]
pub fn simd_eq_str(a: &str, b: &str) -> bool {
    a == b
}
