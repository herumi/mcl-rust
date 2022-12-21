#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::ops::{Add, AddAssign};
use core::ops::{Div, DivAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Sub, SubAssign};
use core::primitive::str;

#[link(name = "mcl", kind = "static")]
#[link(name = "mclbn384_256", kind = "static")]
#[link(name = "stdc++")]
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

    fn mclBnFr_setStr(x: *mut Fr, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnFr_getStr(buf: *mut u8, maxBufSize: usize, x: *const Fr, ioMode: i32) -> usize;
    fn mclBnFr_serialize(buf: *mut u8, maxBufSize: usize, x: *const Fr) -> usize;
    fn mclBnFr_deserialize(x: *mut Fr, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnFr_setInt32(x: *mut Fr, v: i32);
    fn mclBnFr_setLittleEndian(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFr_setLittleEndianMod(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFr_setHashOf(x: *mut Fr, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFr_setByCSPRNG(x: *mut Fr);

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

    fn mclBnFp_setStr(x: *mut Fp, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnFp_getStr(buf: *mut u8, maxBufSize: usize, x: *const Fp, ioMode: i32) -> usize;
    fn mclBnFp_serialize(buf: *mut u8, maxBufSize: usize, x: *const Fp) -> usize;
    fn mclBnFp_deserialize(x: *mut Fp, buf: *const u8, bufSize: usize) -> usize;

    fn mclBnFp_setInt32(x: *mut Fp, v: i32);
    fn mclBnFp_setLittleEndian(x: *mut Fp, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFp_setLittleEndianMod(x: *mut Fp, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFp_setHashOf(x: *mut Fp, buf: *const u8, bufSize: usize) -> i32;
    fn mclBnFp_setByCSPRNG(x: *mut Fp);

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

    fn mclBnFp2_setStr(x: *mut Fp2, buf: *const u8, bufSize: usize, ioMode: i32) -> i32;
    fn mclBnFp2_getStr(buf: *mut u8, maxBufSize: usize, x: *const Fp2, ioMode: i32) -> usize;
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

pub enum CurveType {
    BN254 = 0,
    BN381 = 1,
    SNARK = 4,
    BLS12_381 = 5,
    BLS12_377 = 8,
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
            pub fn zero() -> $t {
                Default::default()
            }
            pub unsafe fn uninit() -> $t {
                let u = MaybeUninit::<$t>::uninit();
                let v = unsafe { u.assume_init() };
                v
            }
            pub fn clear(&mut self) {
                *self = <$t>::zero()
            }
            pub fn is_zero(&self) -> bool {
                unsafe { $is_zero_fn(self) == 1 }
            }
        }
    };
}
macro_rules! is_valid_impl {
    ($t:ty, $is_valid_fn:ident) => {
        impl $t {
            pub fn is_valid(&self) -> bool {
                unsafe { $is_valid_fn(self) == 1 }
            }
        }
    };
}

macro_rules! serialize_impl {
    ($t:ty, $size:expr, $serialize_fn:ident, $deserialize_fn:ident) => {
        impl $t {
            pub fn deserialize(&mut self, buf: &[u8]) -> bool {
                unsafe { $deserialize_fn(self, buf.as_ptr(), buf.len()) > 0 }
            }
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
            pub fn from_str(s: &str, base: i32) -> Option<$t> {
                let mut v = unsafe { <$t>::uninit() };
                if v.set_str(s, base) {
                    return Some(v);
                }
                None
            }
            pub fn set_str(&mut self, s: &str, base: i32) -> bool {
                unsafe { $set_str_fn(self, s.as_ptr(), s.len(), base) == 0 }
            }
            pub fn get_str(&self, io_mode: i32) -> String {
                let u = MaybeUninit::<[u8; $maxBufSize]>::uninit();
                let mut buf = unsafe { u.assume_init() };
                let n: usize;
                unsafe {
                    n = $get_str_fn(buf.as_mut_ptr(), buf.len(), self, io_mode);
                }
                if n == 0 {
                    panic!("mclBnFr_getStr");
                }
                unsafe { core::str::from_utf8_unchecked(&buf[0..n]).into() }
            }
        }
    };
}

macro_rules! int_impl {
    ($t:ty, $set_int_fn:ident, $is_one_fn:ident) => {
        impl $t {
            pub fn from_int(x: i32) -> $t {
                let mut v = unsafe { <$t>::uninit() };
                v.set_int(x);
                v
            }
            pub fn set_int(&mut self, x: i32) {
                unsafe {
                    $set_int_fn(self, x);
                }
            }
            pub fn is_one(&self) -> bool {
                unsafe { $is_one_fn(self) == 1 }
            }
        }
    };
}

macro_rules! base_field_impl {
    ($t:ty,  $set_little_endian_fn:ident, $set_little_endian_mod_fn:ident, $set_hash_of_fn:ident, $set_by_csprng_fn:ident, $is_odd_fn:ident, $is_negative_fn:ident, $square_root_fn:ident) => {
        impl $t {
            pub fn set_little_endian(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_little_endian_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            pub fn set_little_endian_mod(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_little_endian_mod_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            pub fn set_hash_of(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_hash_of_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
            pub fn set_by_csprng(&mut self) {
                unsafe { $set_by_csprng_fn(self) }
            }
            pub fn is_odd(&self) -> bool {
                unsafe { $is_odd_fn(self) == 1 }
            }
            pub fn is_negative(&self) -> bool {
                unsafe { $is_negative_fn(self) == 1 }
            }
            pub fn square_root(y: &mut $t, x: &$t) -> bool {
                unsafe { $square_root_fn(y, x) == 0 }
            }
        }
    };
}

macro_rules! add_op_impl {
    ($t:ty, $add_fn:ident, $sub_fn:ident, $neg_fn:ident) => {
        impl $t {
            pub fn add(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $add_fn(z, x, y) }
            }
            pub fn sub(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $sub_fn(z, x, y) }
            }
            pub fn neg(y: &mut $t, x: &$t) {
                unsafe { $neg_fn(y, x) }
            }
        }
        impl<'a> Add for &'a $t {
            type Output = $t;
            fn add(self, other: &$t) -> $t {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::add(&mut v, &self, &other);
                v
            }
        }
        impl<'a> AddAssign<&'a $t> for $t {
            fn add_assign(&mut self, other: &$t) {
                // how can I write this?
                // unsafe { <$t>::add(&mut self, &self, &other); }
                let mut v = unsafe { <$t>::uninit() };
                <$t>::add(&mut v, &self, &other);
                *self = v;
            }
        }
        impl<'a> Sub for &'a $t {
            type Output = $t;
            fn sub(self, other: &$t) -> $t {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::sub(&mut v, &self, &other);
                v
            }
        }
        impl<'a> SubAssign<&'a $t> for $t {
            fn sub_assign(&mut self, other: &$t) {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::sub(&mut v, &self, &other);
                *self = v;
            }
        }
    };
}

macro_rules! field_mul_op_impl {
    ($t:ty, $mul_fn:ident, $div_fn:ident, $inv_fn:ident, $sqr_fn:ident) => {
        impl $t {
            pub fn mul(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $mul_fn(z, x, y) }
            }
            pub fn div(z: &mut $t, x: &$t, y: &$t) {
                unsafe { $div_fn(z, x, y) }
            }
            pub fn inv(y: &mut $t, x: &$t) {
                unsafe { $inv_fn(y, x) }
            }
            pub fn sqr(y: &mut $t, x: &$t) {
                unsafe { $sqr_fn(y, x) }
            }
        }
        impl<'a> Mul for &'a $t {
            type Output = $t;
            fn mul(self, other: &$t) -> $t {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::mul(&mut v, &self, &other);
                v
            }
        }
        impl<'a> MulAssign<&'a $t> for $t {
            fn mul_assign(&mut self, other: &$t) {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::mul(&mut v, &self, &other);
                *self = v;
            }
        }
        impl<'a> Div for &'a $t {
            type Output = $t;
            fn div(self, other: &$t) -> $t {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::div(&mut v, &self, &other);
                v
            }
        }
        impl<'a> DivAssign<&'a $t> for $t {
            fn div_assign(&mut self, other: &$t) {
                let mut v = unsafe { <$t>::uninit() };
                <$t>::div(&mut v, &self, &other);
                *self = v;
            }
        }
    };
}

macro_rules! ec_impl {
    ($t:ty, $dbl_fn:ident, $mul_fn:ident, $normalize_fn:ident, $set_hash_and_map_fn:ident) => {
        impl $t {
            pub fn dbl(y: &mut $t, x: &$t) {
                unsafe { $dbl_fn(y, x) }
            }
            pub fn mul(z: &mut $t, x: &$t, y: &Fr) {
                unsafe { $mul_fn(z, x, y) }
            }
            pub fn normalize(y: &mut $t, x: &$t) {
                unsafe { $normalize_fn(y, x) }
            }
            pub fn set_hash_of(&mut self, buf: &[u8]) -> bool {
                unsafe { $set_hash_and_map_fn(self, buf.as_ptr(), buf.len()) == 0 }
            }
        }
    };
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fp {
    d: [u64; MCLBN_FP_UNIT_SIZE],
}
impl Fp {
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
str_impl![Fp, 1024, mclBnFp_getStr, mclBnFp_setStr];
int_impl![Fp, mclBnFp_setInt32, mclBnFp_isOne];
base_field_impl![
    Fp,
    mclBnFp_setLittleEndian,
    mclBnFp_setLittleEndianMod,
    mclBnFp_setHashOf,
    mclBnFp_setByCSPRNG,
    mclBnFp_isOdd,
    mclBnFp_isNegative,
    mclBnFp_squareRoot
];
add_op_impl![Fp, mclBnFp_add, mclBnFp_sub, mclBnFp_neg];
field_mul_op_impl![Fp, mclBnFp_mul, mclBnFp_div, mclBnFp_inv, mclBnFp_sqr];

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fp2 {
    d: [Fp; 2],
}
common_impl![Fp2, mclBnFp2_isEqual, mclBnFp2_isZero];
serialize_impl![
    Fp2,
    mclBn_getFpByteSize() * 2,
    mclBnFp2_serialize,
    mclBnFp2_deserialize
];
str_impl![Fp2, 1024, mclBnFp2_getStr, mclBnFp2_setStr];
add_op_impl![Fp2, mclBnFp2_add, mclBnFp2_sub, mclBnFp2_neg];
field_mul_op_impl![Fp2, mclBnFp2_mul, mclBnFp2_div, mclBnFp2_inv, mclBnFp2_sqr];
impl Fp2 {
    pub fn square_root(y: &mut Fp2, x: &Fp2) -> bool {
        unsafe { mclBnFp2_squareRoot(y, x) == 0 }
    }
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fr {
    d: [u64; MCLBN_FR_UNIT_SIZE],
}
impl Fr {
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
str_impl![Fr, 1024, mclBnFr_getStr, mclBnFr_setStr];
int_impl![Fr, mclBnFr_setInt32, mclBnFr_isOne];
base_field_impl![
    Fr,
    mclBnFr_setLittleEndian,
    mclBnFr_setLittleEndianMod,
    mclBnFr_setHashOf,
    mclBnFr_setByCSPRNG,
    mclBnFr_isOdd,
    mclBnFr_isNegative,
    mclBnFr_squareRoot
];
add_op_impl![Fr, mclBnFr_add, mclBnFr_sub, mclBnFr_neg];
field_mul_op_impl![Fr, mclBnFr_mul, mclBnFr_div, mclBnFr_inv, mclBnFr_sqr];

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
str_impl![G1, 1024, mclBnG1_getStr, mclBnG1_setStr];
add_op_impl![G1, mclBnG1_add, mclBnG1_sub, mclBnG1_neg];
ec_impl![
    G1,
    mclBnG1_dbl,
    mclBnG1_mul,
    mclBnG1_normalize,
    mclBnG1_hashAndMapTo
];

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
str_impl![G2, 1024, mclBnG2_getStr, mclBnG2_setStr];
add_op_impl![G2, mclBnG2_add, mclBnG2_sub, mclBnG2_neg];
ec_impl![
    G2,
    mclBnG2_dbl,
    mclBnG2_mul,
    mclBnG2_normalize,
    mclBnG2_hashAndMapTo
];

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
str_impl![GT, 1024, mclBnGT_getStr, mclBnGT_setStr];
int_impl![GT, mclBnGT_setInt32, mclBnGT_isOne];
add_op_impl![GT, mclBnGT_add, mclBnGT_sub, mclBnGT_neg];
field_mul_op_impl![GT, mclBnGT_mul, mclBnGT_div, mclBnGT_inv, mclBnGT_sqr];
impl GT {
    pub fn pow(z: &mut GT, x: &GT, y: &Fr) {
        unsafe { mclBnGT_pow(z, x, y) }
    }
}

pub fn get_version() -> u32 {
    unsafe { mclBn_getVersion() }
}

pub fn init(curve: CurveType) -> bool {
    unsafe { mclBn_init(curve as i32, MCLBN_COMPILED_TIME_VAR) == 0 }
}

pub fn get_fr_serialized_size() -> u32 {
    unsafe { mclBn_getFrByteSize() as u32 }
}

pub fn get_fp_serialized_size() -> u32 {
    unsafe { mclBn_getFpByteSize() as u32 }
}

pub fn get_g1_serialized_size() -> u32 {
    get_fp_serialized_size()
}

pub fn get_g2_serialized_size() -> u32 {
    get_fp_serialized_size() * 2
}

pub fn get_gt_serialized_size() -> u32 {
    get_fp_serialized_size() * 12
}

macro_rules! get_str_impl {
    ($get_str_fn:ident) => {{
        let u = MaybeUninit::<[u8; 256]>::uninit();
        let mut buf = unsafe { u.assume_init() };
        let n: usize;
        unsafe {
            n = $get_str_fn(buf.as_mut_ptr(), buf.len());
        }
        if n == 0 {
            panic!("get_str");
        }
        unsafe { core::str::from_utf8_unchecked(&buf[0..n]).into() }
    }};
}

pub fn get_field_order() -> String {
    get_str_impl![mclBn_getFieldOrder]
}

pub fn get_curve_order() -> String {
    get_str_impl![mclBn_getCurveOrder]
}

pub fn pairing(z: &mut GT, x: &G1, y: &G2) {
    unsafe {
        mclBn_pairing(z, x, y);
    }
}

pub fn miller_loop(z: &mut GT, x: &G1, y: &G2) {
    unsafe {
        mclBn_millerLoop(z, x, y);
    }
}

pub fn final_exp(y: &mut GT, x: &GT) {
    unsafe {
        mclBn_finalExp(y, x);
    }
}
