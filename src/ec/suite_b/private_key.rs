#![deny(dead_code)]

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

//! Functionality shared by operations on private keys (ECC keygen and
//! ECDSA signing).

use super::{ops::*, verify_affine_point_is_on_the_curve};
use crate::{
    arithmetic::montgomery::R,
    error,
    limb::self
};

pub fn affine_from_jacobian(
    ops: &PrivateKeyOps,
    p: &Point,
) -> Result<(Elem<R>, Elem<R>), error::Unspecified> {
    let z = ops.common.point_z(p);

    // Since we restrict our private key to the range [1, n), the curve has
    // prime order, and we verify that the peer's point is on the curve,
    // there's no way that the result can be at infinity. But, use `assert!`
    // instead of `debug_assert!` anyway
    assert!(ops.common.elem_verify_is_not_zero(&z).is_ok());

    let x = ops.common.point_x(p);
    let y = ops.common.point_y(p);

    let zz_inv = ops.elem_inverse_squared(&z);

    let x_aff = ops.common.elem_product(&x, &zz_inv);

    // `y_aff` is needed to validate the point is on the curve. It is also
    // needed in the non-ECDH case where we need to output it.
    let y_aff = {
        let zzzz_inv = ops.common.elem_squared(&zz_inv);
        let zzz_inv = ops.common.elem_product(&z, &zzzz_inv);
        ops.common.elem_product(&y, &zzz_inv)
    };

    // If we validated our inputs correctly and then computed (x, y, z), then
    // (x, y, z) will be on the curve. See
    // `verify_affine_point_is_on_the_curve_scaled` for the motivation.
    verify_affine_point_is_on_the_curve(ops.common, (&x_aff, &y_aff))?;

    Ok((x_aff, y_aff))
}

#[allow(dead_code)]
pub fn big_endian_affine_from_jacobian(
    ops: &PrivateKeyOps,
    x_out: Option<&mut [u8]>,
    y_out: Option<&mut [u8]>,
    p: &Point,
) -> Result<(), error::Unspecified> {
    let (x_aff, y_aff) = affine_from_jacobian(ops, p)?;
    let num_limbs = ops.common.num_limbs;
    if let Some(x_out) = x_out {
        let x = ops.common.elem_unencoded(&x_aff);
        limb::big_endian_from_limbs(&x.limbs[..num_limbs], x_out);
    }
    if let Some(y_out) = y_out {
        let y = ops.common.elem_unencoded(&y_aff);
        limb::big_endian_from_limbs(&y.limbs[..num_limbs], y_out);
    }

    Ok(())
}
