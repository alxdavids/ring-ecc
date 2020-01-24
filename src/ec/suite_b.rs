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

//! Elliptic curve operations on P-256 & P-384.

use self::ops::*;
use crate::{arithmetic::montgomery::*, error, limb::LimbMask};

// NIST SP 800-56A Step 3: "If q is an odd prime p, verify that
// yQ**2 = xQ**3 + axQ + b in GF(p), where the arithmetic is performed modulo
// p."
//
// That is, verify that (x, y) is on the curve, which is true iif:
//
//     y**2 == x**3 + a*x + b (mod q)
//
// Or, equivalently, but more efficiently:
//
//     y**2 == (x**2 + a)*x + b  (mod q)
//
pub fn verify_affine_point_is_on_the_curve(
    ops: &CommonOps,
    (x, y): (&Elem<R>, &Elem<R>),
) -> Result<(), error::Unspecified> {
    verify_affine_point_is_on_the_curve_scaled(ops, (x, y), &ops.a, &ops.b)
}

// Handles the common logic of point-is-on-the-curve checks for both affine and
// Jacobian cases.
//
// When doing the check that the point is on the curve after a computation,
// to avoid fault attacks or mitigate potential bugs, it is better for security
// to use `verify_affine_point_is_on_the_curve` on the affine coordinates,
// because it provides some protection against faults that occur in the
// computation of the inverse of `z`. See the paper and presentation "Fault
// Attacks on Projective-to-Affine Coordinates Conversion" by Diana Maimuţ,
// Cédric Murdica, David Naccache, Mehdi Tibouchi. That presentation concluded
// simply "Check the validity of the result after conversion to affine
// coordinates." (It seems like a good idea to verify that
// z_inv * z == 1 mod q too).
//
// In the case of affine coordinates (x, y), `a_scaled` and `b_scaled` are
// `a` and `b`, respectively. In the case of Jacobian coordinates (x, y, z),
// the computation and comparison is the same, except `a_scaled` and `b_scaled`
// are (z**4 * a) and (z**6 * b), respectively. Thus, performance is another
// reason to prefer doing the check on the affine coordinates, as Jacobian
// computation requires 3 extra multiplications and 2 extra squarings.
//
// An example of a fault attack that isn't mitigated by a point-on-the-curve
// check after multiplication is given in "Sign Change Fault Attacks On
// Elliptic Curve Cryptosystems" by Johannes Blömer, Martin Otto, and
// Jean-Pierre Seifert.
fn verify_affine_point_is_on_the_curve_scaled(
    ops: &CommonOps,
    (x, y): (&Elem<R>, &Elem<R>),
    a_scaled: &Elem<R>,
    b_scaled: &Elem<R>,
) -> Result<(), error::Unspecified> {
    let lhs = ops.elem_squared(y);

    let mut rhs = ops.elem_squared(x);
    ops.elem_add(&mut rhs, a_scaled);
    ops.elem_mul(&mut rhs, x);
    ops.elem_add(&mut rhs, b_scaled);

    if ops.elems_are_equal(&lhs, &rhs) != LimbMask::True {
        return Err(error::Unspecified);
    }

    Ok(())
}

pub mod ops;

pub mod private_key;
mod public_key;
