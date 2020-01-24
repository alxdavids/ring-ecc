use super::ELEM_MAX_BYTES;

#[derive(Copy, Clone)]
pub struct PublicKey {
    bytes: [u8; PUBLIC_KEY_MAX_LEN],
    len: usize,
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

/// The maximum length, in bytes, of an encoded public key.
pub const PUBLIC_KEY_MAX_LEN: usize = 1 + (2 * ELEM_MAX_BYTES);
