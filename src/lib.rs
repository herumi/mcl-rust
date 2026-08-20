//! mcl_rust is a Rust binding of [mcl](https://github.com/herumi/mcl),
//! a portable and fast pairing-based cryptography library.
//!
//! It provides the optimal ate pairing `e: G1 x G2 -> GT` over BN curves and
//! BLS12 curves, where
//! - [`Fp`] : a finite field of prime order `p`, over which the elliptic curve `E` is defined.
//! - [`Fr`] : a finite field of prime order `r`.
//! - [`G1`] : the cyclic subgroup of `E(Fp)` of order `r`.
//! - [`G2`] : the cyclic subgroup of `E'(Fp^2)` of order `r`, where `E'` is a
//!   twist of `E`.
//! - [`GT`] : the cyclic subgroup of `Fp12` of order `r`.
//!
//! `G1` and `G2` are treated as additive groups and `GT` is treated as a multiplicative group.
//!
//! Call [`init`] once before using any other function.
//!
//! # Example
//! ```
//! use mcl_rust::*;
//! assert!(init(CurveType::BLS12_381));
//! let mut p = G1::zero();
//! let mut q = G2::zero();
//! p.set_hash_of(b"abc");
//! q.set_hash_of(b"abc");
//! let mut e = GT::zero();
//! pairing(&mut e, &p, &q);
//! assert!(!e.is_zero());
//! ```
#![no_std]

extern crate alloc;
// wasm builds fp.cpp standalone, so there is no C++ runtime to link.
#[cfg(not(target_arch = "wasm32"))]
extern crate link_cplusplus;

// On wasm32-unknown-unknown there is no libc. compiler-builtins supplies
// mem*/strlen, but malloc/free/strcmp are not provided, so route mcl's only
// heap users (large mulVec, alloc-then-free within one call) to the Rust
// global allocator. wasm32-wasi gets these from wasi-libc instead.
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
mod wasm_libc {
    use alloc::alloc::{alloc, dealloc, Layout};

    const ALIGN: usize = 16;
    const HEADER: usize = 16; // keep 16-byte payload alignment

    #[no_mangle]
    pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
        let total = size + HEADER;
        let layout = Layout::from_size_align(total, ALIGN).unwrap();
        let p = alloc(layout);
        if p.is_null() {
            return p;
        }
        *(p as *mut usize) = size;
        p.add(HEADER)
    }

    #[no_mangle]
    pub unsafe extern "C" fn free(ptr: *mut u8) {
        if ptr.is_null() {
            return;
        }
        let base = ptr.sub(HEADER);
        let size = *(base as *const usize);
        let layout = Layout::from_size_align(size + HEADER, ALIGN).unwrap();
        dealloc(base, layout);
    }

    #[no_mangle]
    pub unsafe extern "C" fn strcmp(a: *const u8, b: *const u8) -> i32 {
        let mut i = 0isize;
        loop {
            let ca = *a.offset(i);
            let cb = *b.offset(i);
            if ca != cb {
                return ca as i32 - cb as i32;
            }
            if ca == 0 {
                return 0;
            }
            i += 1;
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn fill_random(buf: &mut [u8]) {
    getrandom::getrandom(buf).expect("getrandom failed");
}

// wasm32-unknown-unknown has no entropy source, so import one from the host
// (the JS glue backs `env.mclRustFillRandom` with crypto.getRandomValues).
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
extern "C" {
    fn mclRustFillRandom(buf: *mut u8, len: usize);
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn fill_random(buf: &mut [u8]) {
    unsafe { mclRustFillRandom(buf.as_mut_ptr(), buf.len()) }
}

use alloc::string::String;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::ops::{Add, AddAssign};
use core::ops::{Div, DivAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Sub, SubAssign};
use core::primitive::str;

#[allow(non_snake_case)]
extern "C" {
    // global functions
    fn mclBn_init(curve: i32, compiledTimeVar: i32) -> i32;
    fn mclBn_getVersion() -> u32;
    fn mclBn_getFrByteSize() -> u32;
    fn mclBn_getFpByteSize() -> u32;
    fn mclBn_getCurveOrder(buf: *mut u8, maxBufSize: usize) -> usize;
    fn mclBn_getFieldOrder(buf: *mut u8, maxBufSize: usize) -> usize;
    fn mclBn_pairing(z: *mut GT, x: *const G1, y: *const G2);
    fn mclBn_millerLoop(z: *mut GT, x: *const G1, y: *const G2);
    fn mclBn_finalExp(y: *mut GT, x: *const GT);

    // Fr
    fn mclBnFr_isEqual(x: *const Fr, y: *const Fr) -> i32;
    fn mclBnFr_isValid(x: *const Fr) -> i32;
    fn mclBnFr_isZero(x: *const Fr) -> i32;
    fn mclBnFr_isOne(x: *const Fr) -> i32;
    fn mclBnFr_isOdd(x: *const Fr) -> i32;
    fn mclBnFr_isNegative(x: *const Fr) -> i32;
    fn mclBnFr_cmp(x: *const Fr, y: *const Fr) -> i32;

    fn mclBnFr_setStr(x: *mut Fr, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnFr_getStr(buf: *mut u8, maxBufSize: usize, x: *const Fr, ioMode: i32) -> usize;
    fn mclBnFr_serialize(buf: *mut u8, maxBufSize: usize, x: *const Fr) -> usize;
    fn mclBnFr_deserialize(x: *mut Fr, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnFr_setInt32(x: *mut Fr, v: i32);
    fn mclBnFr_setLittleEndian(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFr_setLittleEndianMod(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFr_setHashOf(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;

    fn mclBnFr_add(z: *mut Fr, x: *const Fr, y: *const Fr);
    fn mclBnFr_sub(z: *mut Fr, x: *const Fr, y: *const Fr);
    fn mclBnFr_neg(y: *mut Fr, x: *const Fr);

    fn mclBnFr_mul(z: *mut Fr, x: *const Fr, y: *const Fr);
    fn mclBnFr_div(z: *mut Fr, x: *const Fr, y: *const Fr);
    fn mclBnFr_inv(y: *mut Fr, x: *const Fr);
    fn mclBnFr_sqr(y: *mut Fr, x: *const Fr);
    fn mclBnFr_squareRoot(y: *mut Fr, x: *const Fr) -> i32;

    // Fp
    fn mclBnFp_isEqual(x: *const Fp, y: *const Fp) -> i32;
    fn mclBnFp_isValid(x: *const Fp) -> i32;
    fn mclBnFp_isZero(x: *const Fp) -> i32;
    fn mclBnFp_isOne(x: *const Fp) -> i32;
    fn mclBnFp_isOdd(x: *const Fp) -> i32;
    fn mclBnFp_isNegative(x: *const Fp) -> i32;
    fn mclBnFp_cmp(x: *const Fp, y: *const Fp) -> i32;

    fn mclBnFp_setStr(x: *mut Fp, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnFp_getStr(buf: *mut u8, maxBufSize: usize, x: *const Fp, ioMode: i32) -> usize;
    fn mclBnFp_serialize(buf: *mut u8, maxBufSize: usize, x: *const Fp) -> usize;
    fn mclBnFp_deserialize(x: *mut Fp, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnFp_setInt32(x: *mut Fp, v: i32);
    fn mclBnFp_setLittleEndian(x: *mut Fp, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFp_setLittleEndianMod(x: *mut Fp, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFp_setHashOf(x: *mut Fp, buf: *const u8, bufSize: usize) -> i32;

    fn mclBnFp_add(z: *mut Fp, x: *const Fp, y: *const Fp);
    fn mclBnFp_sub(z: *mut Fp, x: *const Fp, y: *const Fp);
    fn mclBnFp_neg(y: *mut Fp, x: *const Fp);

    fn mclBnFp_mul(z: *mut Fp, x: *const Fp, y: *const Fp);
    fn mclBnFp_div(z: *mut Fp, x: *const Fp, y: *const Fp);
    fn mclBnFp_inv(y: *mut Fp, x: *const Fp);
    fn mclBnFp_sqr(y: *mut Fp, x: *const Fp);
    fn mclBnFp_squareRoot(y: *mut Fp, x: *const Fp) -> i32;

    // Fp2
    fn mclBnFp2_isEqual(x: *const Fp2, y: *const Fp2) -> i32;
    fn mclBnFp2_isZero(x: *const Fp2) -> i32;

    fn mclBnFp2_serialize(buf: *mut u8, maxBufSize: usize, x: *const Fp2) -> usize;
    fn mclBnFp2_deserialize(x: *mut Fp2, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnFp2_add(z: *mut Fp2, x: *const Fp2, y: *const Fp2);
    fn mclBnFp2_sub(z: *mut Fp2, x: *const Fp2, y: *const Fp2);
    fn mclBnFp2_neg(y: *mut Fp2, x: *const Fp2);

    fn mclBnFp2_mul(z: *mut Fp2, x: *const Fp2, y: *const Fp2);
    fn mclBnFp2_div(z: *mut Fp2, x: *const Fp2, y: *const Fp2);
    fn mclBnFp2_inv(y: *mut Fp2, x: *const Fp2);
    fn mclBnFp2_sqr(y: *mut Fp2, x: *const Fp2);
    fn mclBnFp2_squareRoot(y: *mut Fp2, x: *const Fp2) -> i32;

    // G1
    fn mclBnG1_isEqual(x: *const G1, y: *const G1) -> i32;
    fn mclBnG1_isValid(x: *const G1) -> i32;
    fn mclBnG1_isZero(x: *const G1) -> i32;

    fn mclBnG1_setStr(x: *mut G1, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnG1_getStr(buf: *mut u8, maxBufSize: usize, x: *const G1, ioMode: i32) -> usize;
    fn mclBnG1_serialize(buf: *mut u8, maxBufSize: usize, x: *const G1) -> usize;
    fn mclBnG1_deserialize(x: *mut G1, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnG1_add(z: *mut G1, x: *const G1, y: *const G1);
    fn mclBnG1_sub(z: *mut G1, x: *const G1, y: *const G1);
    fn mclBnG1_neg(y: *mut G1, x: *const G1);

    fn mclBnG1_dbl(y: *mut G1, x: *const G1);
    fn mclBnG1_mul(z: *mut G1, x: *const G1, y: *const Fr);
    fn mclBnG1_normalize(y: *mut G1, x: *const G1);
    fn mclBnG1_hashAndMapTo(x: *mut G1, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnG1_mulVec(z: *mut G1, x: *const G1, y: *const Fr, n: usize);

    // G2
    fn mclBnG2_isEqual(x: *const G2, y: *const G2) -> i32;
    fn mclBnG2_isValid(x: *const G2) -> i32;
    fn mclBnG2_isZero(x: *const G2) -> i32;

    fn mclBnG2_setStr(x: *mut G2, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnG2_getStr(buf: *mut u8, maxBufSize: usize, x: *const G2, ioMode: i32) -> usize;
    fn mclBnG2_serialize(buf: *mut u8, maxBufSize: usize, x: *const G2) -> usize;
    fn mclBnG2_deserialize(x: *mut G2, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnG2_add(z: *mut G2, x: *const G2, y: *const G2);
    fn mclBnG2_sub(z: *mut G2, x: *const G2, y: *const G2);
    fn mclBnG2_neg(y: *mut G2, x: *const G2);

    fn mclBnG2_dbl(y: *mut G2, x: *const G2);
    fn mclBnG2_mul(z: *mut G2, x: *const G2, y: *const Fr);
    fn mclBnG2_normalize(y: *mut G2, x: *const G2);
    fn mclBnG2_hashAndMapTo(x: *mut G2, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnG2_mulVec(z: *mut G2, x: *const G2, y: *const Fr, n: usize);

    // GT
    fn mclBnGT_isEqual(x: *const GT, y: *const GT) -> i32;
    fn mclBnGT_isZero(x: *const GT) -> i32;
    fn mclBnGT_isOne(x: *const GT) -> i32;

    fn mclBnGT_setStr(x: *mut GT, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnGT_getStr(buf: *mut u8, maxBufSize: usize, x: *const GT, ioMode: i32) -> usize;
    fn mclBnGT_serialize(buf: *mut u8, maxBufSize: usize, x: *const GT) -> usize;
    fn mclBnGT_deserialize(x: *mut GT, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnGT_setInt32(x: *mut GT, v: i32);

    fn mclBnGT_add(z: *mut GT, x: *const GT, y: *const GT);
    fn mclBnGT_sub(z: *mut GT, x: *const GT, y: *const GT);
    fn mclBnGT_neg(y: *mut GT, x: *const GT);

    fn mclBnGT_mul(z: *mut GT, x: *const GT, y: *const GT);
    fn mclBnGT_div(z: *mut GT, x: *const GT, y: *const GT);
    fn mclBnGT_inv(y: *mut GT, x: *const GT);
    fn mclBnGT_sqr(y: *mut GT, x: *const GT);

    fn mclBnGT_pow(z: *mut GT, x: *const GT, y: *const Fr);
}

/// Curve type passed to [`init`].
#[derive(PartialEq, Copy, Clone)]
pub enum CurveType {
    /// BN curve over a 254-bit prime.
    BN254 = 0,
    /// BN curve over a 381-bit prime (BN381_1).
    BN381 = 1,
    /// BN curve over a 254-bit prime whose order has high 2-adicity (BN_SNARK1).
    SNARK = 4,
    /// BLS12-381 curve.
    BLS12_381 = 5,
    /// BLS12-377 curve.
    BLS12_377 = 8,
    /// BN P256 curve defined in the TCG Algorithm Registry.
    #[allow(non_camel_case_types)]
    BN_P256 = 9,
}

const MCLBN_FP_UNIT_SIZE: usize = 6;
const MCLBN_FR_UNIT_SIZE: usize = 4;
const MCLBN_COMPILED_TIME_VAR: i32 = MCLBN_FR_UNIT_SIZE as i32 * 10 + MCLBN_FP_UNIT_SIZE as i32;

macro_rules! common_impl {
    ($t:ty, $is_equal_fn:ident, $is_zero_fn:ident) => {
        impl PartialEq for $t {
            fn eq(&self, rhs: &Self) -> bool {
                unsafe { $is_equal_fn(self, rhs) == 1 }
            }
        }
        impl $t {
            /// Returns the zero value.
            /// For `G1`/`G2` this is the point at infinity, which is the identity of the group.
            pub fn zero() -> $t {
                Default::default()
            }
            /// This function used to return an uninitialized value, which was undefined behavior.
            /// It now returns a zero-initialized value, so it is equivalent to `zero()`. Use `zero()` instead.
            #[deprecated(since = "1.2.0", note = "use `zero()` instead")]
            pub unsafe fn uninit() -> $t {
                Default::default()
            }
            /// Sets `self` to zero.
            pub fn clear(&mut self) {
                *self = <$t>::zero()
            }
            /// Returns `true` if `self` is zero.
            pub fn is_zero(&self) -> bool {
                unsafe { $is_zero_fn(self) == 1 }
            }
        }
    };
}
macro_rules! is_valid_impl {
    ($t:ty, $is_valid_fn:ident) => {
        impl $t {
            /// Returns `true` if `self` is a valid element.
            /// For `G1`/`G2`, checks that the point is on the curve (and that it has the correct order
            /// if order verification is enabled, which is the default for BLS12 curves).
            pub fn is_valid(&self) -> bool {
                unsafe { $is_valid_fn(self) == 1 }
            }
        }
    };
}

macro_rules! serialize_impl {
    ($t:ty, $size:expr, $serialize_fn:ident, $deserialize_fn:ident) => {
        impl $t {
            /// Deserializes `buf` into `self` and returns `true` on success.
            /// For `G1`/`G2`, deserialization fails if the data does not represent a valid point;
            /// the order of the point is also checked if order verification is enabled, which is the default for BLS12 curves.
            pub fn deserialize(&mut self, buf: &[u8]) -> bool {
                unsafe { $deserialize_fn(self, buf.as_ptr(), buf.len()) > 0 }
            }
            /// Returns the serialized bytes of `self`.
            /// `Fp`/`Fr` are fixed-size little-endian byte sequences,
            /// `G1`/`G2` are fixed-size compressed points, and `GT` is the concatenation of 12 `Fp` values.
            ///
            /// # Panics
            /// Panics if serialization fails (e.g. the library is not initialized).
            pub fn serialize(&self) -> Vec<u8> {
                let size = unsafe { $size } as usize;
                let mut buf: Vec<u8> = Vec::with_capacity(size);
                let n: usize;
                unsafe {
                    n = $serialize_fn(buf.as_mut_ptr(), size, self);
                }
                if n == 0 {
                    panic!("serialize");
                }
                if n > size {
                    panic!("serialize returned an invalid length");
                }
                unsafe {
                    buf.set_len(n);
                }
                buf
            }
        }
    };
}

macro_rules! str_impl {
    ($t:ty, $maxBufSize:expr, $get_str_fn:ident, $set_str_fn:ident) => {
        impl $t {
            /// Returns the value represented by `s` in the given `base` (2, 10, or 16), or `None` if parsing fails.
            /// See [`set_str`](Self::set_str) for the details.
            pub fn from_str(s: &str, base: i32) -> Option<$t> {
                let mut v = <$t>::zero();
                if v.set_str(s, base) {
                    return Some(v);
                }
                None
            }
            /// Sets `self` to the value represented by `s` in the given `base` (2, 10, or 16) and returns `true` on success.
            /// A field value greater than the field order is masked and truncated.
            /// `G1`/`G2` use the format `"0"` (the point at infinity) or `"1 <x> <y>"` (an affine point);
            /// setting fails if the point is not on the curve (and, by default for BLS12 curves, if it does not have the correct order).
            pub fn set_str(&mut self, s: &str, base: i32) -> bool {
                unsafe { $set_str_fn(self, s.as_ptr(), s.len(), base) == 0 }
            }
            /// Returns the string representation of `self` according to `io_mode`: 2 (binary), 10 (decimal), or 16 (hexadecimal).
            ///
            /// # Panics
            /// Panics if the conversion fails.
            pub fn get_str(&self, io_mode: i32) -> String {
                let mut buf = MaybeUninit::<[u8; $maxBufSize]>::uninit();
                let n = unsafe {
                    $get_str_fn(buf.as_mut_ptr().cast::<u8>(), $maxBufSize, self, io_mode)
                };
                if n == 0 {
                    panic!("mclBnFr_getStr");
                }
                if n > $maxBufSize {
                    panic!("mclBnFr_getStr returned an invalid length");
                }
                let bytes = unsafe { core::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), n) };
                core::str::from_utf8(bytes)
                    .expect("getStr returned invalid UTF-8")
                    .into()
            }
        }
    };
}

macro_rules! int_impl {
    ($t:ty, $set_int_fn:ident, $is_one_fn:ident) => {
        impl $t {
            /// Returns the value of the integer `x`.
            pub fn from_int(x: i32) -> $t {
                let mut v = <$t>::zero();
                v.set_int(x);
                v
            }
            /// Sets `self` to the integer `x`.
            pub fn set_int(&mut self, x: i32) {
                unsafe {
                    $set_int_fn(self, x);
                }
            }
            /// Returns `true` if `self` is one.
            pub fn is_one(&self) -> bool {
                unsafe { $is_one_fn(self) == 1 }
            }
        }
    };
}

macro_rules! base_field_impl {
    ($t:ty,  $set_little_endian_fn:ident, $set_little_endian_mod_fn:ident, $set_hash_of_fn:ident, $is_odd_fn:ident, $is_negative_fn:ident, $cmp_fn:ident, $square_root_fn:ident) => {
        impl $t {
            /// Sets `self` to the little-endian value of `buf` masked as
            /// follows, where `L` is the bit length of the field order `q`:
            /// `x &= (1 << L) - 1`, and then `x &= (1 << (L - 1)) - 1` if `x >= q` still holds. Always returns `true`.
            pub fn set_little_endian(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_little_endian_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            /// Sets `self` to the little-endian value of `buf` reduced modulo the field order.
            ///  Returns `false` if `buf` is longer than twice the byte size of `Self`.
            pub fn set_little_endian_mod(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_little_endian_mod_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            /// Sets `self` to a hash of `buf` (SHA-256 if the field is at most 256 bits, SHA-512 otherwise)
            /// masked in the same way as [`set_little_endian`](Self::set_little_endian).
            /// This function is for backward compatibility only; prefer hashing the input yourself and calling
            /// [`set_little_endian_mod`](Self::set_little_endian_mod) with the hashed value.
            pub fn set_hash_of(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_hash_of_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            /// Sets `self` to a value chosen by a cryptographically secure pseudo-random number generator.
            pub fn set_by_csprng(&mut self) {
                let mut buf = [0u8; core::mem::size_of::<$t>()];
                fill_random(&mut buf);
                if !self.set_little_endian_mod(&buf) {
                    panic!("set_by_csprng");
                }
            }
            /// Returns `true` if `self` is odd.
            pub fn is_odd(&self) -> bool {
                unsafe { $is_odd_fn(self) == 1 }
            }
            /// Returns `true` if `self >= (q + 1) / 2`, where `q` is the field order.
            pub fn is_negative(&self) -> bool {
                unsafe { $is_negative_fn(self) == 1 }
            }
            /// Compares `self` and `rhs` as unsigned integers and returns -1 if `self < rhs`, 0 if `self == rhs`, and 1 if `self > rhs`.
            /// NOTE: this may require two Montgomery conversions.
            pub fn cmp(&self, rhs: &$t) -> i32 {
                unsafe { $cmp_fn(self, rhs) }
            }
            /// Sets `y` to one of the square roots of `x` and returns `true` if it exists.
            pub fn square_root(y: &mut $t, x: &$t) -> bool {
                unsafe { $square_root_fn(y, x) == 0 }
            }
        }
    };
}

macro_rules! add_op_impl {
    ($t:ty, $add_fn:ident, $sub_fn:ident, $neg_fn:ident) => {
        impl $t {
            /// `z = x + y`.
            pub fn add(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $add_fn(z, x, y) }
            }
            /// `z = x - y`.
            pub fn sub(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $sub_fn(z, x, y) }
            }
            /// `y = -x`.
            pub fn neg(y: &mut $t, x: &$t) {
                unsafe { $neg_fn(y, x) }
            }
        }
        impl<'a> Add for &'a $t {
            type Output = $t;
            fn add(self, other: &$t) -> $t {
                let mut v = <$t>::zero();
                <$t>::add(&mut v, &self, &other);
                v
            }
        }
        impl<'a> AddAssign<&'a $t> for $t {
            fn add_assign(&mut self, other: &$t) {
                let z: *mut $t = self;
                unsafe {
                    $add_fn(z, z as *const $t, other as *const $t);
                }
            }
        }
        impl<'a> Sub for &'a $t {
            type Output = $t;
            fn sub(self, other: &$t) -> $t {
                let mut v = <$t>::zero();
                <$t>::sub(&mut v, &self, &other);
                v
            }
        }
        impl<'a> SubAssign<&'a $t> for $t {
            fn sub_assign(&mut self, other: &$t) {
                let z: *mut $t = self;
                unsafe {
                    $sub_fn(z, z as *const $t, other as *const $t);
                }
            }
        }
    };
}

macro_rules! field_mul_op_impl {
    ($t:ty, $mul_fn:ident, $div_fn:ident, $inv_fn:ident, $sqr_fn:ident) => {
        impl $t {
            /// `z = x * y`.
            pub fn mul(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $mul_fn(z, x, y) }
            }
            /// `z = x / y`.
            pub fn div(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $div_fn(z, x, y) }
            }
            /// `y = 1 / x`. See the documentation of [`GT`] for the behavior on `GT`.
            pub fn inv(y: &mut $t, x: &$t) {
                unsafe { $inv_fn(y, x) }
            }
            /// `y = x * x`.
            pub fn sqr(y: &mut $t, x: &$t) {
                unsafe { $sqr_fn(y, x) }
            }
        }
        impl<'a> Mul for &'a $t {
            type Output = $t;
            fn mul(self, other: &$t) -> $t {
                let mut v = <$t>::zero();
                <$t>::mul(&mut v, &self, &other);
                v
            }
        }
        impl<'a> MulAssign<&'a $t> for $t {
            fn mul_assign(&mut self, other: &$t) {
                let z: *mut $t = self;
                unsafe {
                    $mul_fn(z, z as *const $t, other as *const $t);
                }
            }
        }
        impl<'a> Div for &'a $t {
            type Output = $t;
            fn div(self, other: &$t) -> $t {
                let mut v = <$t>::zero();
                <$t>::div(&mut v, &self, &other);
                v
            }
        }
        impl<'a> DivAssign<&'a $t> for $t {
            fn div_assign(&mut self, other: &$t) {
                let z: *mut $t = self;
                unsafe {
                    $div_fn(z, z as *const $t, other as *const $t);
                }
            }
        }
    };
}

macro_rules! ec_impl {
    ($t:ty, $dbl_fn:ident, $mul_fn:ident, $normalize_fn:ident, $set_hash_and_map_fn:ident, $mul_vec_fn:ident) => {
        impl $t {
            /// `y = 2x` (doubling).
            pub fn dbl(y: &mut $t, x: &$t) {
                unsafe { $dbl_fn(y, x) }
            }
            /// `z = x * y` (scalar multiplication).
            pub fn mul(z: &mut $t, x: &$t, y: &Fr) {
                unsafe { $mul_fn(z, x, y) }
            }
            /// Sets `y` to the point `x` converted from Jacobian
            /// coordinates `[x:y:z]` to `[x:y:1]` (or `[*:*:0]` if `x` is zero).
            /// The represented point does not change.
            pub fn normalize(y: &mut $t, x: &$t) {
                unsafe { $normalize_fn(y, x) }
            }
            /// Sets `self` to the point obtained by hashing `buf` and mapping the hash value to the group.
            pub fn set_hash_of(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_hash_and_map_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            /// `z = sum of x[i] * y[i]` (multi-scalar multiplication).
            /// `x` and `y` must have the same length; `x.len()` elements are read from `y`.
            pub fn mul_vec(z: &mut $t, x: &[$t], y: &[Fr]) {
                unsafe { $mul_vec_fn(z, x.as_ptr(), y.as_ptr(), x.len()) }
            }
        }
    };
}

/// An element of the finite field `Fp` of prime order `p`, over which the elliptic curve is defined.
/// The value is stored in Montgomery representation.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fp {
    d: [u64; MCLBN_FP_UNIT_SIZE],
}
impl Fp {
    /// Returns the decimal string of the order `p` of `Fp`.
    pub fn get_order() -> String {
        get_field_order()
    }
}
common_impl![Fp, mclBnFp_isEqual, mclBnFp_isZero];
is_valid_impl![Fp, mclBnFp_isValid];
serialize_impl![
    Fp,
    mclBn_getFpByteSize(),
    mclBnFp_serialize,
    mclBnFp_deserialize
];
str_impl![Fp, 128, mclBnFp_getStr, mclBnFp_setStr];
int_impl![Fp, mclBnFp_setInt32, mclBnFp_isOne];
base_field_impl![
    Fp,
    mclBnFp_setLittleEndian,
    mclBnFp_setLittleEndianMod,
    mclBnFp_setHashOf,
    mclBnFp_isOdd,
    mclBnFp_isNegative,
    mclBnFp_cmp,
    mclBnFp_squareRoot
];
add_op_impl![Fp, mclBnFp_add, mclBnFp_sub, mclBnFp_neg];
field_mul_op_impl![Fp, mclBnFp_mul, mclBnFp_div, mclBnFp_inv, mclBnFp_sqr];

/// An element `x = d[0] + d[1] i` of the field extension `Fp2 = Fp[i]` of degree 2, where `i^2 = -1`.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fp2 {
    pub d: [Fp; 2],
}
common_impl![Fp2, mclBnFp2_isEqual, mclBnFp2_isZero];
serialize_impl![
    Fp2,
    mclBn_getFpByteSize() * 2,
    mclBnFp2_serialize,
    mclBnFp2_deserialize
];
add_op_impl![Fp2, mclBnFp2_add, mclBnFp2_sub, mclBnFp2_neg];
field_mul_op_impl![Fp2, mclBnFp2_mul, mclBnFp2_div, mclBnFp2_inv, mclBnFp2_sqr];
impl Fp2 {
    /// Sets `y` to one of the square roots of `x` and returns `true` if it exists.
    pub fn square_root(y: &mut Fp2, x: &Fp2) -> bool {
        unsafe { mclBnFp2_squareRoot(y, x) == 0 }
    }
}

/// An element of the finite field `Fr` of prime order `r`, where `r` is the order of `G1`, `G2`, and `GT`.
/// The value is stored in Montgomery representation.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fr {
    d: [u64; MCLBN_FR_UNIT_SIZE],
}
impl Fr {
    /// Returns the decimal string of the order `r` of `Fr`.
    pub fn get_order() -> String {
        get_curve_order()
    }
}
common_impl![Fr, mclBnFr_isEqual, mclBnFr_isZero];
is_valid_impl![Fr, mclBnFr_isValid];
serialize_impl![
    Fr,
    mclBn_getFrByteSize(),
    mclBnFr_serialize,
    mclBnFr_deserialize
];
str_impl![Fr, 128, mclBnFr_getStr, mclBnFr_setStr];
int_impl![Fr, mclBnFr_setInt32, mclBnFr_isOne];
base_field_impl![
    Fr,
    mclBnFr_setLittleEndian,
    mclBnFr_setLittleEndianMod,
    mclBnFr_setHashOf,
    mclBnFr_isOdd,
    mclBnFr_isNegative,
    mclBnFr_cmp,
    mclBnFr_squareRoot
];
add_op_impl![Fr, mclBnFr_add, mclBnFr_sub, mclBnFr_neg];
field_mul_op_impl![Fr, mclBnFr_mul, mclBnFr_div, mclBnFr_inv, mclBnFr_sqr];

/// An element of the cyclic group `G1` of order `r` on the elliptic curve `E(Fp)`,
/// represented as `[x:y:z]` in Jacobian coordinates.
/// `G1` is an additive group.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct G1 {
    pub x: Fp,
    pub y: Fp,
    pub z: Fp,
}
common_impl![G1, mclBnG1_isEqual, mclBnG1_isZero];
is_valid_impl![G1, mclBnG1_isValid];
serialize_impl![
    G1,
    mclBn_getFpByteSize(),
    mclBnG1_serialize,
    mclBnG1_deserialize
];
str_impl![G1, 128 * 3, mclBnG1_getStr, mclBnG1_setStr];
add_op_impl![G1, mclBnG1_add, mclBnG1_sub, mclBnG1_neg];
ec_impl![
    G1,
    mclBnG1_dbl,
    mclBnG1_mul,
    mclBnG1_normalize,
    mclBnG1_hashAndMapTo,
    mclBnG1_mulVec
];

/// An element of the cyclic group `G2` of order `r` on the elliptic curve `E'(Fp2)`,
/// where `E'` is a twist of `E`, represented as `[x:y:z]` in Jacobian coordinates.
/// `G2` is an additive group.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct G2 {
    pub x: Fp2,
    pub y: Fp2,
    pub z: Fp2,
}
common_impl![G2, mclBnG2_isEqual, mclBnG2_isZero];
is_valid_impl![G2, mclBnG2_isValid];
serialize_impl![
    G2,
    mclBn_getFpByteSize() * 2,
    mclBnG2_serialize,
    mclBnG2_deserialize
];
str_impl![G2, 128 * 3 * 2, mclBnG2_getStr, mclBnG2_setStr];
add_op_impl![G2, mclBnG2_add, mclBnG2_sub, mclBnG2_neg];
ec_impl![
    G2,
    mclBnG2_dbl,
    mclBnG2_mul,
    mclBnG2_normalize,
    mclBnG2_hashAndMapTo,
    mclBnG2_mulVec
];

/// An element of `Fp12`, used to represent the cyclic group
/// `GT = { x in Fp12 | x^r = 1 }` of order `r`.
///
/// `GT` is a multiplicative group: the group operations are
/// [`mul`](GT::mul), [`div`](GT::div), [`sqr`](GT::sqr), and [`pow`](GT::pow).
/// [`add`](GT::add), [`sub`](GT::sub), and [`neg`](GT::neg) operate in `Fp12`,
/// so their results are generally not elements of `GT`.
/// [`inv`](GT::inv) computes the conjugate `a - b w` of `x = a + b w` in `Fp12 = Fp6[w]`,
/// which equals `1 / x` only when `x` is an element of `GT`.
#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct GT {
    d: [Fp; 12],
}
common_impl![GT, mclBnGT_isEqual, mclBnGT_isZero];
serialize_impl![
    GT,
    mclBn_getFpByteSize() * 12,
    mclBnGT_serialize,
    mclBnGT_deserialize
];
str_impl![GT, 128 * 12, mclBnGT_getStr, mclBnGT_setStr];
int_impl![GT, mclBnGT_setInt32, mclBnGT_isOne];
add_op_impl![GT, mclBnGT_add, mclBnGT_sub, mclBnGT_neg];
field_mul_op_impl![GT, mclBnGT_mul, mclBnGT_div, mclBnGT_inv, mclBnGT_sqr];
impl GT {
    /// `z = x^y`.
    pub fn pow(z: &mut GT, x: &GT, y: &Fr) {
        unsafe { mclBnGT_pow(z, x, y) }
    }
}

/// Returns the version of the underlying mcl library as `0xABC`, which
/// means version A.BC.
pub fn get_version() -> u32 {
    unsafe { mclBn_getVersion() }
}

/// Initializes the mcl library for `curve` and returns `true` on success.
/// Call this function before calling any other function.
/// This function is not thread-safe.
pub fn init(curve: CurveType) -> bool {
    unsafe { mclBn_init(curve as i32, MCLBN_COMPILED_TIME_VAR) == 0 }
}

/// Returns the byte size of a serialized `Fr`.
pub fn get_fr_serialized_size() -> u32 {
    unsafe { mclBn_getFrByteSize() as u32 }
}

/// Returns the byte size of a serialized `Fp`.
pub fn get_fp_serialized_size() -> u32 {
    unsafe { mclBn_getFpByteSize() as u32 }
}

/// Returns the byte size of a serialized (compressed) `G1` point.
pub fn get_g1_serialized_size() -> u32 {
    get_fp_serialized_size()
}

/// Returns the byte size of a serialized (compressed) `G2` point.
pub fn get_g2_serialized_size() -> u32 {
    get_fp_serialized_size() * 2
}

/// Returns the byte size of a serialized `GT`.
pub fn get_gt_serialized_size() -> u32 {
    get_fp_serialized_size() * 12
}

macro_rules! get_str_impl {
    ($get_str_fn:ident) => {{
        const BUF_SIZE: usize = 256;
        let mut buf = MaybeUninit::<[u8; BUF_SIZE]>::uninit();
        let n = unsafe { $get_str_fn(buf.as_mut_ptr().cast::<u8>(), BUF_SIZE) };
        if n == 0 {
            panic!("get_str");
        }
        if n > BUF_SIZE {
            panic!("get_str returned an invalid length");
        }
        let bytes = unsafe { core::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), n) };
        core::str::from_utf8(bytes)
            .expect("get_str returned invalid UTF-8")
            .into()
    }};
}

/// Returns the decimal string of the order `p` of `Fp`, over which the elliptic curve is defined.
pub fn get_field_order() -> String {
    get_str_impl![mclBn_getFieldOrder]
}

/// Returns the decimal string of the order `r` of the curve, which is the order of `Fr`.
pub fn get_curve_order() -> String {
    get_str_impl![mclBn_getCurveOrder]
}

/// `z = e(x, y)`, where `e: G1 x G2 -> GT` is the optimal ate pairing.
///
/// `e(x, y) = final_exp(miller_loop(x, y))`.
pub fn pairing(z: &mut GT, x: &G1, y: &G2) {
    unsafe {
        mclBn_pairing(z, x, y);
    }
}

/// `z = MillerLoop(x, y)`, the Miller loop part of the pairing.
///
/// `final_exp` satisfies `e(P1, Q1) e(P2, Q2) = final_exp(MillerLoop(P1, Q1) * MillerLoop(P2, Q2))`,
/// so products of pairings can share one final exponentiation.
pub fn miller_loop(z: &mut GT, x: &G1, y: &G2) {
    unsafe {
        mclBn_millerLoop(z, x, y);
    }
}

/// `y = finalExp(x)`, the final exponentiation part of the pairing.
pub fn final_exp(y: &mut GT, x: &GT) {
    unsafe {
        mclBn_finalExp(y, x);
    }
}
