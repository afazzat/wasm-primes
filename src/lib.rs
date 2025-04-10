// src/lib.rs
use wasm_bindgen::prelude::*;  // WASM interoperability
use js_sys::Array;             // JS-compatible array type

/// Core Rust function to generate primes up to a given limit.
/// 
/// # Arguments
/// * `limit` - Upper bound for prime generation (must be ≥ 2).
/// 
/// # Returns
/// * `Result<Vec<u32>, &'static str>` - Vector of primes on success, error message on failure.
/// 
/// This function uses the Sieve of Eratosthenes algorithm and is testable without WASM dependencies.
fn generate_primes_rs(limit: u32) -> Result<Vec<u32>, &'static str> {
    // Validate input
    if limit < 2 {
        return Err("Limit must be at least 2");
    }

    // Initialize sieve: true = prime candidate, false = non-prime
    let mut sieve = vec![true; (limit + 1) as usize];
    sieve[0] = false;  // 0 is not prime
    sieve[1] = false;  // 1 is not prime

    // Sieve algorithm: Mark non-primes starting from 2
    for num in 2..=(limit as f64).sqrt() as u32 {
        if sieve[num as usize] {
            // Mark all multiples of `num` as non-prime
            let mut multiple = num * num;
            while multiple <= limit {
                sieve[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    // Extract primes from the sieve
    Ok(sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)  // Keep only primes
        .map(|(i, _)| i as u32)             // Convert index to number
        .collect()                          // Collect into Vec<u32>
    )
}

/// WASM-compatible wrapper for `generate_primes_rs`.
/// 
/// Converts Rust-native results to JS-compatible types (Array and JsValue).
/// 
/// # Arguments
/// * `limit` - Passed directly to `generate_primes_rs`.
/// 
/// # Returns
/// * `Result<Array, JsValue>` - JS array of primes or JS error.
#[wasm_bindgen]
pub fn generate_primes(limit: u32) -> Result<Array, JsValue> {
    generate_primes_rs(limit)
        .map(|primes| {
            // Convert Vec<u32> to JS array
            primes
                .iter()
                .map(|&n| JsValue::from(n))  // Convert each prime to JS value
                .collect::<Array>()          // Collect into JS array
        })
        .map_err(|e| JsValue::from_str(e))   // Convert error message to JS error
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests prime generation for a valid input (limit = 10)
    #[test]
    fn test_primes_up_to_10() {
        let primes = generate_primes_rs(10).unwrap();
        assert_eq!(primes, vec![2, 3, 5, 7]);  // Verify known primes
    }

    /// Tests error handling for invalid input (limit = 1)
    #[test]
    fn test_invalid_limit() {
        assert!(generate_primes_rs(1).is_err());  // Expect error
    }
}
