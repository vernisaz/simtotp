/*Generating TOTP in Rust involves implementing the algorithm described in RFC 6238. This typically uses the HMAC-SHA1 cryptographic function along with a time component.
Below is an example of Rust code for generating a TOTP.
*/
// src/main.rs
mod sha1;
mod hmac;
//use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::hmac;
//type HmacSha1 = Hmac<Sha1>;

/// Generates a TOTP code.
///
/// # Arguments
///
/// * `secret` - The secret key as a byte slice.
/// * `digits` - The number of digits for the TOTP code (e.g., 6 or 8).
/// * `step_seconds` - The time step in seconds (e.g., 30).
///
/// # Returns
///
/// An `Option<u32>` containing the TOTP code if successful, otherwise `None`.
pub fn generate_totp(secret: &[u8], digits: u32, step_seconds: u64) -> Option<u32> {
    let current_time_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();

    let time_step = current_time_seconds / step_seconds;

    // Convert the time step to an 8-byte big-endian array.
    let time_bytes = time_step.to_be_bytes();
    /*
    let mut mac = HmacSha1::new_from_slice(secret).ok()?;
    mac.update(&time_bytes);
*/
    let result = hmac(secret, &time_bytes, 64); //mac.finalize();
    
    let code = hotp_from_hmac(&result, digits); // .bytes().as_slice()

    Some(code)
}

/// Extracts an HOTP code from an HMAC result.
fn hotp_from_hmac(hmac_result: &[u8], digits: u32) -> u32 {
    let offset = (hmac_result[19] & 0xf) as usize;
    let otp = 
        ((hmac_result[offset] as u32) & 0x7f) << 24 |
        ((hmac_result[offset + 1] as u32) & 0xff) << 16 |
        ((hmac_result[offset + 2] as u32) & 0xff) << 8 |
        ((hmac_result[offset + 3] as u32) & 0xff)
    ;

    let power_of_10 = 10u32.pow(digits);
    otp % power_of_10
}

fn main() {
    let test = hmac(b"key", b"The quick brown fox jumps over the lazy dog", 64);
    eprintln!("code 0x{}", simweb::to_hex(&test));
    // Example usage
    let secret = b"Some secret is here";
    let digits = 6;
    let step = 30;

    match generate_totp(secret, digits, step) {
        Some(code) => {
            println!("Current TOTP code: {:0>width$}", code, width = digits as usize);
        }
        None => {
            println!("Failed to generate TOTP code.");
        }
    }
}
/*
Explanation
 * generate_totp function:
   * It first gets the current system time in seconds since the Unix epoch.
   * It calculates the current time step by dividing the current time by the step_seconds (e.g., 30).
   * The time step is converted into an 8-byte big-endian array, which is the message used in the HMAC calculation.
   * An HmacSha1 instance is created with the secret key.
   * The update method adds the time step to the HMAC context.
   * finalize computes the HMAC digest.
   * The digest is then passed to the hotp_from_hmac function to extract the final passcode.
 * hotp_from_hmac function:
   * This function implements the "dynamic truncation" part of the algorithm.
   * It takes the last 4 bits of the HMAC digest (at index 19) to determine an offset into the digest.
   * It then takes 4 bytes starting from that offset.
   * The most significant bit of the first byte is masked off to ensure the result is a positive integer.
   * These 4 bytes are combined into a single u32 value.
   * Finally, a modulo operation is performed with a power of 10 to get the desired number of digits.
This example provides a clear, self-contained implementation of the TOTP algorithm in Rust. For a more robust solution in a real-world application, you might consider using a dedicated library like otp-rs which handles more details, such as base32 secret key decoding and error handling.
*/