#![allow(dead_code)]
/// Align `size` by rounding it up to a multiple of 4.
pub fn align_size(size: usize) -> usize {
    size + (4 - (size % 4)) % 4
}

/// Read a little-endian 32-bit unsigned integer from the buffer.
pub fn read_u32(buffer: &[u8]) -> u32 {
    buffer
        .iter()
        .take(4)
        .enumerate()
        .fold(0, |result, (i, byte)| result + ((*byte as u32) << (i * 8)))
}

/// Write a little-endian 32-bit unsigned integer to the buffer.
pub fn write_u32(buffer: &mut [u8], value: u32) {
    for (i, byte) in buffer.iter_mut().take(4).enumerate() {
        *byte = (value >> (i * 8)) as u8;
    }
}
