// Copyright 2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Error reporting.

use untrusted;

#[cfg(feature = "std")]
extern crate std;

/// An error with absolutely no details.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unspecified;

impl Unspecified {
    fn description_() -> &'static str {
        "ring_ecc::error::Unspecified"
    }
}

// This is required for the implementation of `std::error::Error`.
impl core::fmt::Display for Unspecified {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(Self::description_())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Unspecified {
    #[inline]
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        Self::description_()
    }
}

impl From<untrusted::EndOfInput> for Unspecified {
    fn from(_: untrusted::EndOfInput) -> Self {
        Unspecified
    }
}

impl From<core::array::TryFromSliceError> for Unspecified {
    fn from(_: core::array::TryFromSliceError) -> Self {
        Unspecified
    }
}

/// An error parsing or validating a key.
///
/// The `Display` implementation and `<KeyRejected as Error>::description()`
/// will return a string that will help you better understand why a key was
/// rejected change which errors are reported in which situations while
/// minimizing the likelihood that any applications will be broken.
///
/// Here is an incomplete list of reasons a key may be unsupported:
///
/// * Invalid or Inconsistent Components: A component of the key has an invalid
///   value, or the mathematical relationship between two (or more) components
///   required for a valid key does not hold.
///
/// * The encoding of the key is invalid. Perhaps the key isn't in the correct
///   format; e.g. it may be Base64 ("PEM") encoded, in which case   the Base64
///   encoding needs to be undone first.
///
/// * The encoding includes a versioning mechanism and that mechanism indicates
///   that the key is encoded in a version of the encoding that isn't supported.
///   This might happen for multi-prime RSA keys (keys with more than two
///   private   prime factors), which aren't supported, for example.
///
/// * Too small or too Large: One of the primary components of the key is too
///   small or two large. Too-small keys are rejected for security reasons. Some
///   unnecessarily large keys are rejected for performance reasons.
///
///  * Wrong algorithm: The key is not valid for the algorithm in which it was
///    being used.
///
///  * Unexpected errors: Report this as a bug.
#[derive(Copy, Clone, Debug)]
pub struct KeyRejected(&'static str);

impl KeyRejected {
    /// The value returned from <Self as std::error::Error>::description()
    pub fn description_(&self) -> &'static str {
        self.0
    }
}

#[cfg(feature = "std")]
impl std::error::Error for KeyRejected {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        self.description_()
    }
}

impl core::fmt::Display for KeyRejected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.description_())
    }
}

impl From<KeyRejected> for Unspecified {
    fn from(_: KeyRejected) -> Self {
        Unspecified
    }
}
