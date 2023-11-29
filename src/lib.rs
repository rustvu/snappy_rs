//! Wrapper for the snappy library's C API.
//!
//! Based on snappy-c.h from the snappy source code.
//! Alternative approach would be to use
//! [bindgen](https://rust-lang.github.io/rust-bindgen/) to generate
//! the bindings.

use std::fmt;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum snappy_status {
    SNAPPY_OK = 0,
    SNAPPY_INVALID_INPUT = 1,
    SNAPPY_BUFFER_TOO_SMALL = 2,
}

extern "C" {
    fn snappy_compress(
        input: *const u8,
        input_length: usize,
        compressed: *mut u8,
        compressed_length: *mut usize,
    ) -> snappy_status;
    fn snappy_uncompress(
        compressed: *const u8,
        compressed_length: usize,
        uncompressed: *mut u8,
        uncompressed_length: *mut usize,
    ) -> snappy_status;
    fn snappy_max_compressed_length(source_length: usize) -> usize;
    fn snappy_uncompressed_length(
        compressed: *const u8,
        compressed_length: usize,
        result: *mut usize,
    ) -> snappy_status;

    #[allow(dead_code)]
    fn snappy_validate_compressed_buffer(
        compressed: *const u8,
        compressed_length: usize,
    ) -> snappy_status;
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidInput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Compresses the input buffer into the output buffer.
pub fn compress(input: &[u8]) -> Result<Vec<u8>> {
    unsafe {
        let max_len = snappy_max_compressed_length(input.len());
        let mut out = Vec::with_capacity(max_len);
        let mut out_len = out.capacity();

        let status = snappy_compress(
            input.as_ptr(),
            input.len(),
            out.as_mut_ptr(),
            &mut out_len,
        );
        match status {
            snappy_status::SNAPPY_INVALID_INPUT => Err(Error::InvalidInput),
            snappy_status::SNAPPY_OK => {
                out.set_len(out_len);
                Ok(out)
            }
            _ => panic!("Unexpected snappy_status: {:?}", status),
        }
    }
}

/// Uncompresses the input buffer into the output buffer.
pub fn uncompress(input: &[u8]) -> Result<Vec<u8>> {
    unsafe {
        let mut out_len = 0;
        let status =
            snappy_uncompressed_length(input.as_ptr(), input.len(), &mut out_len);
        match status {
            snappy_status::SNAPPY_INVALID_INPUT => Err(Error::InvalidInput),
            snappy_status::SNAPPY_OK => {
                let mut out = Vec::with_capacity(out_len);
                let status = snappy_uncompress(
                    input.as_ptr(),
                    input.len(),
                    out.as_mut_ptr(),
                    &mut out_len,
                );
                match status {
                    snappy_status::SNAPPY_INVALID_INPUT => Err(Error::InvalidInput),
                    snappy_status::SNAPPY_OK => {
                        out.set_len(out_len);
                        Ok(out)
                    }
                    _ => panic!("Unexpected snappy_status: {:?}", status),
                }
            }
            _ => panic!("Unexpected snappy_status: {:?}", status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let input = b"Hello world!";
        let compressed = compress(input).unwrap();
        assert_eq!(uncompress(&compressed).unwrap(), input);
    }
}
