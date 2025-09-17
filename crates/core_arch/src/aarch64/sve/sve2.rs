// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `crates/stdarch-gen2/spec/` and run the following command to re-generate this file:
//
// ```
// cargo run --bin=stdarch-gen2 -- crates/stdarch-gen2/spec
// ```
#![feature(core_intrinsics)]
#![feature(rustc_attrs)]
#![feature(link_llvm_intrinsics)]
#![feature(adt_const_params)]
#![allow(internal_features)] 
#![allow(improper_ctypes)]
#![feature(unchecked_shifts)]
#![feature(portable_simd)]
#![feature(simd_ffi)]
#![feature(aarch64_target_feature)]
#[cfg(test)]
#[cfg(target_arch = "aarch64")]
// extern crate core_arch;
// extern crate stdarch_test;
// use stdarch_test::assert_instr;
use std::arch::aarch64::*;
use std::ffi;
use std::mem;
//use crate::core_arch::arch::aarch64::*;
use std::arch::aarch64::float32x4_t;
use std::arch::aarch64::float64x2_t;
use std::arch::aarch64::int8x16_t;
use std::arch::aarch64::int16x8_t;
use std::arch::aarch64::int32x4_t;
use std::arch::aarch64::int64x2_t;
use crate::std::marker::ConstParamTy;
use std::intrinsics::simd::simd_reinterpret;
use std::intrinsics::simd::simd_select;
use std::intrinsics::simd::simd_cast;
#[derive(Copy, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svbool_t(bool);

impl Clone for svbool_t {
    fn clone(&self) -> Self {
        *self // 简单的复制自身
    }
}
/*macro_rules! static_assert_range {
    ($value:expr, $min:expr, $max:expr) => {
        const _: () = if $value < $min || $value > $max {
            panic!("Value {} is out of range. Expected between {} and {}", $value, $min, $max);
        } else {
            ()
        };
    };
}*/
macro_rules! static_assert_range {
    ($val:expr, $min:expr, $max:expr) => {{
        const _: [(); $val as usize - $min as usize] = [(); 0]; // 紧凑的范围断言
    }};
}

// === 基本数据类型定义 ===

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svint8_t(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svuint8_t(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svint16_t(i16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svuint16_t(u16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svfloat32_t(f32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svint32_t(i32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svuint32_t(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svfloat64_t(f64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svint64_t(i64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svuint64_t(u64);

// === 多向量数据类型定义 ===

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(32)]
#[allow(non_camel_case_types)]
pub struct svint8x2_t(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(32)]
#[allow(non_camel_case_types)]
pub struct svuint8x2_t(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svint16x2_t(i16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svuint16x2_t(u16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svfloat32x2_t(f32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svint32x2_t(i32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svuint32x2_t(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svfloat64x2_t(f64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svint64x2_t(i64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svuint64x2_t(u64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(48)]
#[allow(non_camel_case_types)]
pub struct svint8x3_t(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(48)]
#[allow(non_camel_case_types)]
pub struct svuint8x3_t(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(24)]
#[allow(non_camel_case_types)]
pub struct svint16x3_t(i16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(24)]
#[allow(non_camel_case_types)]
pub struct svuint16x3_t(u16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(12)]
#[allow(non_camel_case_types)]
pub struct svfloat32x3_t(f32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(12)]
#[allow(non_camel_case_types)]
pub struct svint32x3_t(i32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(12)]
#[allow(non_camel_case_types)]
pub struct svuint32x3_t(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(6)]
#[allow(non_camel_case_types)]
pub struct svfloat64x3_t(f64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(6)]
#[allow(non_camel_case_types)]
pub struct svint64x3_t(i64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(6)]
#[allow(non_camel_case_types)]
pub struct svuint64x3_t(u64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(64)]
#[allow(non_camel_case_types)]
pub struct svint8x4_t(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(64)]
#[allow(non_camel_case_types)]
pub struct svuint8x4_t(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(32)]
#[allow(non_camel_case_types)]
pub struct svint16x4_t(i16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(32)]
#[allow(non_camel_case_types)]
pub struct svuint16x4_t(u16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svfloat32x4_t(f32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svint32x4_t(i32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svuint32x4_t(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svfloat64x4_t(f64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svint64x4_t(i64);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svuint64x4_t(u64);

// === 内部数据类型定义 ===

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2i8(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct nxv4i8(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct nxv8i8(i8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2i16(i16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct nxv4i16(i16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2i32(i32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2u8(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct nxv4u8(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct nxv8u8(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2u16(u16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct nxv4u16(u16);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2u32(u32);

// === 内部谓词类型定义 ===

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svbool2_t(bool);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svbool4_t(bool);

#[derive(Copy, Clone, PartialEq, Debug)]
#[rustc_scalable_vector(8)]
#[allow(non_camel_case_types)]
pub struct svbool8_t(bool);

// === FFI 绑定 ===

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svptrue_b8() -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svptrue_b8() -> svbool_t;
    }
    _svptrue_b8()
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svptrue_b16() -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svptrue_b16() -> svbool_t;
    }
    _svptrue_b16()
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svptrue_b32() -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svptrue_b32() -> svbool_t;
    }
    _svptrue_b32()
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svptrue_b64() -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svptrue_b64() -> svbool_t;
    }
    _svptrue_b64()
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_s8(value: i8) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_s8(value: i8) -> svint8_t;
    }
    _svdup_n_s8(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_s8(pg: svbool_t, t: svint8_t, f: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_s8(pg: svbool_t, t: svint8_t, f: svint8_t) -> svint8_t;
    }
    _svsel_s8(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_u8(value: u8) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_u8(value: u8) -> svuint8_t;
    }
    _svdup_n_u8(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_u8(pg: svbool_t, t: svuint8_t, f: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_u8(pg: svbool_t, t: svuint8_t, f: svuint8_t) -> svuint8_t;
    }
    _svsel_u8(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_s16(value: i16) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_s16(value: i16) -> svint16_t;
    }
    _svdup_n_s16(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_s16(pg: svbool_t, t: svint16_t, f: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_s16(pg: svbool_t, t: svint16_t, f: svint16_t) -> svint16_t;
    }
    _svsel_s16(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_u16(value: u16) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_u16(value: u16) -> svuint16_t;
    }
    _svdup_n_u16(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_u16(pg: svbool_t, t: svuint16_t, f: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_u16(pg: svbool_t, t: svuint16_t, f: svuint16_t) -> svuint16_t;
    }
    _svsel_u16(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_f32(value: f32) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_f32(value: f32) -> svfloat32_t;
    }
    _svdup_n_f32(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_f32(pg: svbool_t, t: svfloat32_t, f: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_f32(pg: svbool_t, t: svfloat32_t, f: svfloat32_t) -> svfloat32_t;
    }
    _svsel_f32(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_s32(value: i32) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_s32(value: i32) -> svint32_t;
    }
    _svdup_n_s32(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_s32(pg: svbool_t, t: svint32_t, f: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_s32(pg: svbool_t, t: svint32_t, f: svint32_t) -> svint32_t;
    }
    _svsel_s32(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_u32(value: u32) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_u32(value: u32) -> svuint32_t;
    }
    _svdup_n_u32(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_u32(pg: svbool_t, t: svuint32_t, f: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_u32(pg: svbool_t, t: svuint32_t, f: svuint32_t) -> svuint32_t;
    }
    _svsel_u32(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_f64(value: f64) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_f64(value: f64) -> svfloat64_t;
    }
    _svdup_n_f64(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_f64(pg: svbool_t, t: svfloat64_t, f: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_f64(pg: svbool_t, t: svfloat64_t, f: svfloat64_t) -> svfloat64_t;
    }
    _svsel_f64(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_s64(value: i64) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_s64(value: i64) -> svint64_t;
    }
    _svdup_n_s64(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_s64(pg: svbool_t, t: svint64_t, f: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_s64(pg: svbool_t, t: svint64_t, f: svint64_t) -> svint64_t;
    }
    _svsel_s64(pg, t, f)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_u64(value: u64) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_u64(value: u64) -> svuint64_t;
    }
    _svdup_n_u64(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_u64(pg: svbool_t, t: svuint64_t, f: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_u64(pg: svbool_t, t: svuint64_t, f: svuint64_t) -> svuint64_t;
    }
    _svsel_u64(pg, t, f)
}

// 多向量类型的 dup 和 sel 函数（示例，实际需要更多）
#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svdup_n_s8x2(value: i8) -> svint8x2_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svdup_n_s8x2(value: i8) -> svint8x2_t;
    }
    _svdup_n_s8x2(value)
}

#[inline(never)]
#[target_feature(enable = "sve")]
pub unsafe fn svsel_s8x2(pg: svbool_t, t: svint8x2_t, f: svint8x2_t) -> svint8x2_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsel_s8x2(pg: svbool_t, t: svint8x2_t, f: svint8x2_t) -> svint8x2_t;
    }
    _svsel_s8x2(pg, t, f)
}

// 其他多向量类型的函数类似，需要为每种类型提供相应的实现

// === 枚举类型定义 ===

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy, Debug)]
#[non_exhaustive]
pub enum svpattern {
    SV_POW2 = 0,
    SV_VL1 = 1,
    SV_VL2 = 2,
    SV_VL3 = 3,
    SV_VL4 = 4,
    SV_VL5 = 5,
    SV_VL6 = 6,
    SV_VL7 = 7,
    SV_VL8 = 8,
    SV_VL16 = 9,
    SV_VL32 = 10,
    SV_VL64 = 11,
    SV_VL128 = 12,
    SV_VL256 = 13,
    SV_MUL4 = 29,
    SV_MUL3 = 30,
    SV_ALL = 31,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy, Debug)]
#[non_exhaustive]
pub enum svprfop {
    SV_PLDL1KEEP = 0,
    SV_PLDL1STRM = 1,
    SV_PLDL2KEEP = 2,
    SV_PLDL2STRM = 3,
    SV_PLDL3KEEP = 4,
    SV_PLDL3STRM = 5,
    SV_PSTL1KEEP = 8,
    SV_PSTL1STRM = 9,
    SV_PSTL2KEEP = 10,
    SV_PSTL2STRM = 11,
    SV_PSTL3KEEP = 12,
    SV_PSTL3STRM = 13,
}

pub trait AsUnsigned {
    type Unsigned: ?Sized;
    unsafe fn as_unsigned(self) -> Self::Unsigned;
}

pub trait AsSigned {
    type Signed: ?Sized;
    unsafe fn as_signed(self) -> Self::Signed;
}

impl AsUnsigned for svint8_t {
    type Unsigned = svuint8_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint8_t {
        svuint8_t(self.0 as u8)
    }
}

impl AsSigned for svuint8_t {
    type Signed = svint8_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_signed(self) -> svint8_t {
        svint8_t(self.0 as i8)
    }
}

impl AsUnsigned for svint16_t {
    type Unsigned = svuint16_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint16_t {
        svuint16_t(self.0 as u16)
    }
}

impl AsSigned for svuint16_t {
    type Signed = svint16_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_signed(self) -> svint16_t {
        svint16_t(self.0 as i16)
    }
}

impl AsUnsigned for svint32_t {
    type Unsigned = svuint32_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint32_t {
        svuint32_t(self.0 as u32)
    }
}

impl AsSigned for svuint32_t {
    type Signed = svint32_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_signed(self) -> svint32_t {
        svint32_t(self.0 as i32)
    }
}

impl AsUnsigned for svint64_t {
    type Unsigned = svuint64_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint64_t {
        svuint64_t(self.0 as u64)
    }
}

impl AsSigned for svuint64_t {
    type Signed = svint64_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_signed(self) -> svint64_t {
        svint64_t(self.0 as i64)
    }
}

impl AsUnsigned for svuint8_t {
    type Unsigned = svuint8_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint8_t {
        self
    }
}

// impl AsSigned for svuint8_t {
//     type Signed = svint8_t;

//     #[inline]
//     #[target_feature(enable = "sve")]
//     unsafe fn as_signed(self) -> svint8_t {
//         svint8_t(self.0 as i8)
//     }
// }

impl AsUnsigned for svuint16_t {
    type Unsigned = svuint16_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint16_t {
        self
    }
}

// impl AsSigned for svuint16_t {
//     type Signed = svint16_t;

//     #[inline]
//     #[target_feature(enable = "sve")]
//     unsafe fn as_signed(self) -> svint16_t {
//         svint16_t(self.0 as i16)
//     }
// }

impl AsUnsigned for svuint32_t {
    type Unsigned = svuint32_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint32_t {
        self
    }
}

// impl AsSigned for svuint32_t {
//     type Signed = svint32_t;

//     #[inline]
//     #[target_feature(enable = "sve")]
//     unsafe fn as_signed(self) -> svint32_t {
//         svint32_t(self.0 as i32)
//     }
// }

impl AsUnsigned for svuint64_t {
    type Unsigned = svuint64_t;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> svuint64_t {
        self
    }
}

// impl AsSigned for svuint64_t {
//     type Signed = svint64_t;

//     #[inline]
//     #[target_feature(enable = "sve")]
//     unsafe fn as_signed(self) -> svint64_t {
//         svint64_t(self.0 as i64)
//     }
// }

impl AsUnsigned for nxv2i8 {
    type Unsigned = nxv2u8;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv2u8 {
        nxv2u8(self.0 as u8) // 将 i8 转换为 u8
    }
}

impl AsUnsigned for nxv4i8 {
    type Unsigned = nxv4u8;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv4u8 {
        nxv4u8(self.0 as u8) // 将 i8 转换为 u8
    }
}

impl AsUnsigned for nxv8i8 {
    type Unsigned = nxv8u8;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv8u8 {
        nxv8u8(self.0 as u8) // 将 i8 转换为 u8
    }
}

impl AsUnsigned for nxv2i16 {
    type Unsigned = nxv2u16;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv2u16 {
        nxv2u16(self.0 as u16) // 将 i16 转换为 u16
    }
}

impl AsUnsigned for nxv4i16 {
    type Unsigned = nxv4u16;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv4u16 {
        nxv4u16(self.0 as u16) // 将 i16 转换为 u16
    }
}

impl AsUnsigned for nxv2i32 {
    type Unsigned = nxv2u32;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv2u32 {
        nxv2u32(self.0 as u32) // 将 i32 转换为 u32
    }
}

impl AsUnsigned for nxv2u8 {
    type Unsigned = nxv2u8;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv2u8 {
        self // 无需转换
    }
}

impl AsUnsigned for nxv4u8 {
    type Unsigned = nxv4u8;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv4u8 {
        self // 无需转换
    }
}

impl AsUnsigned for nxv8u8 {
    type Unsigned = nxv8u8;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv8u8 {
        self // 无需转换
    }
}

impl AsUnsigned for nxv2u16 {
    type Unsigned = nxv2u16;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv2u16 {
        self // 无需转换
    }
}

impl AsUnsigned for nxv4u16 {
    type Unsigned = nxv4u16;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv4u16 {
        self // 无需转换
    }
}

impl AsUnsigned for nxv2u32 {
    type Unsigned = nxv2u32;

    #[inline]
    #[target_feature(enable = "sve")]
    unsafe fn as_unsigned(self) -> nxv2u32 {
        self // 无需转换
    }
}

impl AsSigned for *const u8 {
    type Signed = *const i8;

    #[inline]
    unsafe fn as_signed(self) -> Self::Signed {
        self as *const i8 // 将 *const u8 转换为 *const i8
    }
}

impl AsSigned for *const u16 {
    type Signed = *const i16;

    #[inline]
    unsafe fn as_signed(self) -> Self::Signed {
        self as *const i16 // 将 *const u16 转换为 *const i16
    }
}

impl AsSigned for *const u32 {
    type Signed = *const i32;

    #[inline]
    unsafe fn as_signed(self) -> Self::Signed {
        self as *const i32 // 将 *const u32 转换为 *const i32
    }
}

impl AsSigned for *const u64 {
    type Signed = *const i64;

    #[inline]
    unsafe fn as_signed(self) -> Self::Signed {
        self as *const i64 // 将 *const u64 转换为 *const i64
    }
}

impl AsUnsigned for i8 {
    type Unsigned = u8;

    #[inline]
    unsafe fn as_unsigned(self) -> u8 {
        self as u8
    }
}

impl AsUnsigned for i16 {
    type Unsigned = u16;

    #[inline]
    unsafe fn as_unsigned(self) -> u16 {
        self as u16
    }
}

impl AsUnsigned for i32 {
    type Unsigned = u32;

    #[inline]
    unsafe fn as_unsigned(self) -> u32 {
        self as u32
    }
}

impl AsUnsigned for i64 {
    type Unsigned = u64;

    #[inline]
    unsafe fn as_unsigned(self) -> u64 {
        self as u64
    }
}

impl AsUnsigned for svint8x2_t {
    type Unsigned = svuint8x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint8x2_t {
        svuint8x2_t(self.0 as u8)
    }
}

impl AsUnsigned for svint16x2_t {
    type Unsigned = svuint16x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint16x2_t {
        svuint16x2_t(self.0 as u16)
    }
}

impl AsUnsigned for svint32x2_t {
    type Unsigned = svuint32x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint32x2_t {
        svuint32x2_t(self.0 as u32)
    }
}

impl AsUnsigned for svint64x2_t {
    type Unsigned = svuint64x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint64x2_t {
        svuint64x2_t(self.0 as u64)
    }
}

impl AsUnsigned for svint8x3_t {
    type Unsigned = svuint8x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint8x3_t {
        svuint8x3_t(self.0 as u8)
    }
}

impl AsUnsigned for svint16x3_t {
    type Unsigned = svuint16x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint16x3_t {
        svuint16x3_t(self.0 as u16)
    }
}

impl AsUnsigned for svint32x3_t {
    type Unsigned = svuint32x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint32x3_t {
        svuint32x3_t(self.0 as u32)
    }
}

impl AsUnsigned for svint64x3_t {
    type Unsigned = svuint64x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint64x3_t {
        svuint64x3_t(self.0 as u64)
    }
}

impl AsUnsigned for svint8x4_t {
    type Unsigned = svuint8x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint8x4_t {
        svuint8x4_t(self.0 as u8)
    }
}

impl AsUnsigned for svint16x4_t {
    type Unsigned = svuint16x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint16x4_t {
        svuint16x4_t(self.0 as u16)
    }
}

impl AsUnsigned for svint32x4_t {
    type Unsigned = svuint32x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint32x4_t {
        svuint32x4_t(self.0 as u32)
    }
}

impl AsUnsigned for svint64x4_t {
    type Unsigned = svuint64x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint64x4_t {
        svuint64x4_t(self.0 as u64)
    }
}

impl AsUnsigned for svuint8x2_t {
    type Unsigned = svuint8x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint8x2_t {
        svuint8x2_t(self.0)  // 已经是 u8 类型，所以直接返回
    }
}

impl AsUnsigned for svuint16x2_t {
    type Unsigned = svuint16x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint16x2_t {
        svuint16x2_t(self.0)  // 已经是 u16 类型，所以直接返回
    }
}

impl AsUnsigned for svuint32x2_t {
    type Unsigned = svuint32x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint32x2_t {
        svuint32x2_t(self.0)  // 已经是 u32 类型，所以直接返回
    }
}

impl AsUnsigned for svuint64x2_t {
    type Unsigned = svuint64x2_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint64x2_t {
        svuint64x2_t(self.0)  // 已经是 u64 类型，所以直接返回
    }
}

impl AsUnsigned for svuint8x3_t {
    type Unsigned = svuint8x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint8x3_t {
        svuint8x3_t(self.0)  // 已经是 u8 类型，所以直接返回
    }
}

impl AsUnsigned for svuint16x3_t {
    type Unsigned = svuint16x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint16x3_t {
        svuint16x3_t(self.0)  // 已经是 u16 类型，所以直接返回
    }
}

impl AsUnsigned for svuint32x3_t {
    type Unsigned = svuint32x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint32x3_t {
        svuint32x3_t(self.0)  // 已经是 u32 类型，所以直接返回
    }
}

impl AsUnsigned for svuint64x3_t {
    type Unsigned = svuint64x3_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint64x3_t {
        svuint64x3_t(self.0)  // 已经是 u64 类型，所以直接返回
    }
}

impl AsUnsigned for svuint8x4_t {
    type Unsigned = svuint8x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint8x4_t {
        svuint8x4_t(self.0)  // 已经是 u8 类型，所以直接返回
    }
}

impl AsUnsigned for svuint16x4_t {
    type Unsigned = svuint16x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint16x4_t {
        svuint16x4_t(self.0)  // 已经是 u16 类型，所以直接返回
    }
}

impl AsUnsigned for svuint32x4_t {
    type Unsigned = svuint32x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint32x4_t {
        svuint32x4_t(self.0)  // 已经是 u32 类型，所以直接返回
    }
}

impl AsUnsigned for svuint64x4_t {
    type Unsigned = svuint64x4_t;

    #[inline]
    unsafe fn as_unsigned(self) -> svuint64x4_t {
        svuint64x4_t(self.0)  // 已经是 u64 类型，所以直接返回
    }
}

impl AsSigned for svuint8x2_t {
    type Signed = svint8x2_t;
    #[inline]
    unsafe fn as_signed(self) -> svint8x2_t {
        svint8x2_t(self.0 as i8)
    }
}

impl AsSigned for svuint8x3_t {
    type Signed = svint8x3_t;
    #[inline]
    unsafe fn as_signed(self) -> svint8x3_t {
        svint8x3_t(self.0 as i8)
    }
}

impl AsSigned for svuint8x4_t {
    type Signed = svint8x4_t;
    #[inline]
    unsafe fn as_signed(self) -> svint8x4_t {
        svint8x4_t(self.0 as i8)
    }
}

impl AsSigned for svuint16x2_t {
    type Signed = svint16x2_t;
    #[inline]
    unsafe fn as_signed(self) -> svint16x2_t {
        svint16x2_t(self.0 as i16)
    }
}

impl AsSigned for svuint16x3_t {
    type Signed = svint16x3_t;
    #[inline]
    unsafe fn as_signed(self) -> svint16x3_t {
        svint16x3_t(self.0 as i16)
    }
}

impl AsSigned for svuint16x4_t {
    type Signed = svint16x4_t;
    #[inline]
    unsafe fn as_signed(self) -> svint16x4_t {
        svint16x4_t(self.0 as i16)
    }
}

impl AsSigned for svuint32x2_t {
    type Signed = svint32x2_t;
    #[inline]
    unsafe fn as_signed(self) -> svint32x2_t {
        svint32x2_t(self.0 as i32)
    }
}

impl AsSigned for svuint32x3_t {
    type Signed = svint32x3_t;
    #[inline]
    unsafe fn as_signed(self) -> svint32x3_t {
        svint32x3_t(self.0 as i32)
    }
}

impl AsSigned for svuint32x4_t {
    type Signed = svint32x4_t;
    #[inline]
    unsafe fn as_signed(self) -> svint32x4_t {
        svint32x4_t(self.0 as i32)
    }
}

impl AsSigned for svuint64x2_t {
    type Signed = svint64x2_t;
    #[inline]
    unsafe fn as_signed(self) -> svint64x2_t {
        svint64x2_t(self.0 as i64)
    }
}

impl AsSigned for svuint64x3_t {
    type Signed = svint64x3_t;
    #[inline]
    unsafe fn as_signed(self) -> svint64x3_t {
        svint64x3_t(self.0 as i64)
    }
}

impl AsSigned for svuint64x4_t {
    type Signed = svint64x4_t;
    #[inline]
    unsafe fn as_signed(self) -> svint64x4_t {
        svint64x4_t(self.0 as i64)
    }
}

impl AsSigned for u8 {
    type Signed = i8;

    unsafe fn as_signed(self) -> Self::Signed {
        self as i8
    }
}

impl AsSigned for u16 {
    type Signed = i16;

    unsafe fn as_signed(self) -> Self::Signed {
        self as i16
    }
}

impl AsSigned for u32 {
    type Signed = i32;

    unsafe fn as_signed(self) -> Self::Signed {
        self as i32
    }
}

impl AsSigned for u64 {
    type Signed = i64;

    unsafe fn as_signed(self) -> Self::Signed {
        self as i64
    }
}

impl From<svbool_t> for svbool8_t {
    fn from(val: svbool_t) -> Self {
        extern "C" {
            #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
            fn convert_to_svbool8(val: svbool_t) -> svbool8_t;
        }
        unsafe { convert_to_svbool8(val) }
    }
}

impl From<svbool8_t> for svbool_t {
    fn from(val: svbool8_t) -> Self {
        extern "C" {
            #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
            fn convert_from_svbool8(val: svbool8_t) -> svbool_t;
        }
        unsafe { convert_from_svbool8(val) }
    }
}

impl From<svbool_t> for svbool4_t {
    fn from(val: svbool_t) -> Self {
        extern "C" {
            #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
            fn convert_to_svbool4(val: svbool_t) -> svbool4_t;
        }
        unsafe { convert_to_svbool4(val) }
    }
}

impl From<svbool4_t> for svbool_t {
    fn from(val: svbool4_t) -> Self {
        extern "C" {
            #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
            fn convert_from_svbool4(val: svbool4_t) -> svbool_t;
        }
        unsafe { convert_from_svbool4(val) }
    }
}

impl From<svbool_t> for svbool2_t {
    fn from(val: svbool_t) -> Self {
        extern "C" {
            #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
            fn convert_to_svbool2(val: svbool_t) -> svbool2_t;
        }
        unsafe { convert_to_svbool2(val) }
    }
}

impl From<svbool2_t> for svbool_t {
    fn from(val: svbool2_t) -> Self {
        extern "C" {
            #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
            fn convert_from_svbool2(val: svbool2_t) -> svbool_t;
        }
        unsafe { convert_from_svbool2(val) }
    }
}

// f32 向量获取函数
pub unsafe fn svget2_f32<const INDEX: usize>(tuple: svfloat32x2_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_f32(tuple: svfloat32x2_t, index: i32) -> svfloat32_t;
    }
    unsafe { _svget2_f32(tuple, INDEX as i32) }
}

// f64 向量获取函数
pub unsafe fn svget2_f64<const INDEX: usize>(tuple: svfloat64x2_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_f64(tuple: svfloat64x2_t, index: i32) -> svfloat64_t;
    }
    unsafe { _svget2_f64(tuple, INDEX as i32) }
}

// s8 向量获取函数
pub unsafe fn svget2_s8<const INDEX: usize>(tuple: svint8x2_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_s8(tuple: svint8x2_t, index: i32) -> svint8_t;
    }
    unsafe { _svget2_s8(tuple, INDEX as i32) }
}

// s16 向量获取函数
pub unsafe fn svget2_s16<const INDEX: usize>(tuple: svint16x2_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_s16(tuple: svint16x2_t, index: i32) -> svint16_t;
    }
    unsafe { _svget2_s16(tuple, INDEX as i32) }
}

// s32 向量获取函数
pub unsafe fn svget2_s32<const INDEX: usize>(tuple: svint32x2_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_s32(tuple: svint32x2_t, index: i32) -> svint32_t;
    }
    unsafe { _svget2_s32(tuple, INDEX as i32) }
}

// s64 向量获取函数
pub unsafe fn svget2_s64<const INDEX: usize>(tuple: svint64x2_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_s64(tuple: svint64x2_t, index: i32) -> svint64_t;
    }
    unsafe { _svget2_s64(tuple, INDEX as i32) }
}

// u8 向量获取函数（如果需要的话）
pub unsafe fn svget2_u8<const INDEX: usize>(tuple: svuint8x2_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_u8(tuple: svuint8x2_t, index: i32) -> svuint8_t;
    }
    unsafe { _svget2_u8(tuple, INDEX as i32) }
}

// u16 向量获取函数（如果需要的话）
pub unsafe fn svget2_u16<const INDEX: usize>(tuple: svuint16x2_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_u16(tuple: svuint16x2_t, index: i32) -> svuint16_t;
    }
    unsafe { _svget2_u16(tuple, INDEX as i32) }
}

// u32 向量获取函数（如果需要的话）
pub unsafe fn svget2_u32<const INDEX: usize>(tuple: svuint32x2_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_u32(tuple: svuint32x2_t, index: i32) -> svuint32_t;
    }
    unsafe { _svget2_u32(tuple, INDEX as i32) }
}

// u64 向量获取函数（如果需要的话）
pub unsafe fn svget2_u64<const INDEX: usize>(tuple: svuint64x2_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svget2_u64(tuple: svuint64x2_t, index: i32) -> svuint64_t;
    }
    unsafe { _svget2_u64(tuple, INDEX as i32) }
}




#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svaba_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svaba_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svaba_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svaba_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svaba_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svaba_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svaba_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saba))]
pub unsafe fn svaba_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svaba_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_u8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svaba_u8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svaba_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_u16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svaba_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svaba_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svaba_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svaba_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaba_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svaba_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaba))]
pub unsafe fn svaba_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svaba_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalb))]
pub unsafe fn svabalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalb_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalb))]
pub unsafe fn svabalb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svabalb_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalb))]
pub unsafe fn svabalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalb_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalb))]
pub unsafe fn svabalb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svabalb_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalb))]
pub unsafe fn svabalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalb_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalb))]
pub unsafe fn svabalb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svabalb_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalb))]
pub unsafe fn svabalb_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalb_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalb_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalb))]
pub unsafe fn svabalb_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svabalb_u16(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalb))]
pub unsafe fn svabalb_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalb_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalb))]
pub unsafe fn svabalb_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svabalb_u32(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalb))]
pub unsafe fn svabalb_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalb_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalb))]
pub unsafe fn svabalb_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svabalb_u64(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalt))]
pub unsafe fn svabalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalt))]
pub unsafe fn svabalt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svabalt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalt))]
pub unsafe fn svabalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalt))]
pub unsafe fn svabalt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svabalt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalt))]
pub unsafe fn svabalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabalt))]
pub unsafe fn svabalt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svabalt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalt))]
pub unsafe fn svabalt_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalt_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svabalt_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalt))]
pub unsafe fn svabalt_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svabalt_u16(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalt))]
pub unsafe fn svabalt_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalt_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svabalt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalt))]
pub unsafe fn svabalt_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svabalt_u32(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalt))]
pub unsafe fn svabalt_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabalt_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svabalt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabalt))]
pub unsafe fn svabalt_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svabalt_u64(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlb))]
pub unsafe fn svabdlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlb))]
pub unsafe fn svabdlb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svabdlb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlb))]
pub unsafe fn svabdlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlb))]
pub unsafe fn svabdlb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svabdlb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlb))]
pub unsafe fn svabdlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlb))]
pub unsafe fn svabdlb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svabdlb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlb))]
pub unsafe fn svabdlb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlb))]
pub unsafe fn svabdlb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svabdlb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlb))]
pub unsafe fn svabdlb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlb))]
pub unsafe fn svabdlb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svabdlb_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlb))]
pub unsafe fn svabdlb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlb))]
pub unsafe fn svabdlb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svabdlb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlt))]
pub unsafe fn svabdlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlt))]
pub unsafe fn svabdlt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svabdlt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlt))]
pub unsafe fn svabdlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlt))]
pub unsafe fn svabdlt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svabdlt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlt))]
pub unsafe fn svabdlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sabdlt))]
pub unsafe fn svabdlt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svabdlt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlt))]
pub unsafe fn svabdlt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svabdlt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlt))]
pub unsafe fn svabdlt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svabdlt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlt))]
pub unsafe fn svabdlt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svabdlt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlt))]
pub unsafe fn svabdlt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svabdlt_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlt))]
pub unsafe fn svabdlt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svabdlt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svabdlt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uabdlt))]
pub unsafe fn svabdlt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svabdlt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadalp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svadalp_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint8_t) -> svint16_t {
    svadalp_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s16_z(pg: svbool_t, op1: svint16_t, op2: svint8_t) -> svint16_t {
    svadalp_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadalp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svadalp_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint16_t) -> svint32_t {
    svadalp_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s32_z(pg: svbool_t, op1: svint32_t, op2: svint16_t) -> svint32_t {
    svadalp_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadalp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svadalp_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint32_t) -> svint64_t {
    svadalp_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sadalp))]
pub unsafe fn svadalp_s64_z(pg: svbool_t, op1: svint64_t, op2: svint32_t) -> svint64_t {
    svadalp_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadalp_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svadalp_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    svadalp_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    svadalp_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadalp_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svadalp_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    svadalp_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    svadalp_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadalp_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svadalp_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    svadalp_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uadalp))]
pub unsafe fn svadalp_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    svadalp_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclb))]
pub unsafe fn svadclb_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadclb_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svadclb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclb))]
pub unsafe fn svadclb_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svadclb_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclb))]
pub unsafe fn svadclb_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadclb_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svadclb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclb))]
pub unsafe fn svadclb_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svadclb_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclt))]
pub unsafe fn svadclt_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadclt_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svadclt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclt))]
pub unsafe fn svadclt_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svadclt_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclt))]
pub unsafe fn svadclt_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svadclt_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svadclt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(adclt))]
pub unsafe fn svadclt_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svadclt_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svaddhnb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svaddhnb_s16(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svaddhnb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svaddhnb_s32(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svaddhnb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svaddhnb_s64(op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svaddhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svaddhnb_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svaddhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svaddhnb_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svaddhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnb))]
pub unsafe fn svaddhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svaddhnb_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svaddhnt_s16(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svaddhnt_s16(even, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svaddhnt_s32(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svaddhnt_s32(even, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svaddhnt_s64(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svaddhnt_s64(even, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svaddhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svaddhnt_u16(even, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svaddhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svaddhnt_u32(even, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svaddhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addhnt))]
pub unsafe fn svaddhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svaddhnt_u64(even, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlb))]
pub unsafe fn svaddlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlb))]
pub unsafe fn svaddlb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svaddlb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlb))]
pub unsafe fn svaddlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlb))]
pub unsafe fn svaddlb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svaddlb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlb))]
pub unsafe fn svaddlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlb))]
pub unsafe fn svaddlb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svaddlb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlb))]
pub unsafe fn svaddlb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlb))]
pub unsafe fn svaddlb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svaddlb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlb))]
pub unsafe fn svaddlb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlb))]
pub unsafe fn svaddlb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svaddlb_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlb))]
pub unsafe fn svaddlb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlb))]
pub unsafe fn svaddlb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svaddlb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlbt))]
pub unsafe fn svaddlbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svaddlbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlbt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlbt))]
pub unsafe fn svaddlbt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svaddlbt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlbt))]
pub unsafe fn svaddlbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svaddlbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlbt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlbt))]
pub unsafe fn svaddlbt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svaddlbt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlbt))]
pub unsafe fn svaddlbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svaddlbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlbt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlbt))]
pub unsafe fn svaddlbt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svaddlbt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlt))]
pub unsafe fn svaddlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlt))]
pub unsafe fn svaddlt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svaddlt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlt))]
pub unsafe fn svaddlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlt))]
pub unsafe fn svaddlt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svaddlt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlt))]
pub unsafe fn svaddlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddlt))]
pub unsafe fn svaddlt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svaddlt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlt))]
pub unsafe fn svaddlt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddlt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlt))]
pub unsafe fn svaddlt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svaddlt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlt))]
pub unsafe fn svaddlt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddlt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlt))]
pub unsafe fn svaddlt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svaddlt_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlt))]
pub unsafe fn svaddlt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddlt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddlt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddlt))]
pub unsafe fn svaddlt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svaddlt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(faddp))]
pub unsafe fn svaddp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svaddp_f32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(faddp))]
pub unsafe fn svaddp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svaddp_f32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(faddp))]
pub unsafe fn svaddp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svaddp_f64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(faddp))]
pub unsafe fn svaddp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svaddp_f64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svaddp_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svaddp_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svaddp_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svaddp_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svaddp_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svaddp_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svaddp_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svaddp_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { svaddp_s8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svaddp_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    unsafe { svaddp_s16_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svaddp_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { svaddp_s32_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svaddp_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { svaddp_s64_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(addp))]
pub unsafe fn svaddp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svaddp_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwb))]
pub unsafe fn svaddwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwb))]
pub unsafe fn svaddwb_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svaddwb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwb))]
pub unsafe fn svaddwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwb))]
pub unsafe fn svaddwb_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svaddwb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwb))]
pub unsafe fn svaddwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwb))]
pub unsafe fn svaddwb_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svaddwb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwb))]
pub unsafe fn svaddwb_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwb_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwb))]
pub unsafe fn svaddwb_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svaddwb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwb))]
pub unsafe fn svaddwb_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwb_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwb))]
pub unsafe fn svaddwb_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svaddwb_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwb))]
pub unsafe fn svaddwb_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwb_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwb))]
pub unsafe fn svaddwb_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svaddwb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwt))]
pub unsafe fn svaddwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwt))]
pub unsafe fn svaddwt_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svaddwt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwt))]
pub unsafe fn svaddwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwt))]
pub unsafe fn svaddwt_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svaddwt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwt))]
pub unsafe fn svaddwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(saddwt))]
pub unsafe fn svaddwt_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svaddwt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwt))]
pub unsafe fn svaddwt_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwt_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svaddwt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwt))]
pub unsafe fn svaddwt_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svaddwt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwt))]
pub unsafe fn svaddwt_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwt_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svaddwt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwt))]
pub unsafe fn svaddwt_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svaddwt_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwt))]
pub unsafe fn svaddwt_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaddwt_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svaddwt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uaddwt))]
pub unsafe fn svaddwt_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svaddwt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(aesd))]
pub unsafe fn svaesd_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaesd_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svaesd_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(aese))]
pub unsafe fn svaese_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaese_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svaese_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(aesimc))]
pub unsafe fn svaesimc_u8(op: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaesimc_u8(op: svint8_t) -> svint8_t;
    }
    unsafe { _svaesimc_u8(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(aesmc))]
pub unsafe fn svaesmc_u8(op: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svaesmc_u8(op: svint8_t) -> svint8_t;
    }
    unsafe { _svaesmc_u8(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbcax_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbcax_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbcax_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbcax_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbcax_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbcax_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbcax_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbcax_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbcax_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbcax_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbcax_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbcax_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbcax_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbcax_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbcax_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbcax_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbcax_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbcax_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbcax_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bcax))]
pub unsafe fn svbcax_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbcax_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbdep_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svbdep_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svbdep_u8(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_u16(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbdep_u16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svbdep_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_n_u16(op1: svuint16_t, op2: u16) -> svuint16_t {
    svbdep_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbdep_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svbdep_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svbdep_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbdep_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svbdep_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bdep))]
pub unsafe fn svbdep_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svbdep_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbext_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svbext_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svbext_u8(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_u16(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbext_u16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svbext_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_n_u16(op1: svuint16_t, op2: u16) -> svuint16_t {
    svbext_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbext_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svbext_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svbext_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbext_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svbext_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bext))]
pub unsafe fn svbext_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svbext_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbgrp_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svbgrp_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svbgrp_u8(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_u16(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbgrp_u16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svbgrp_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_n_u16(op1: svuint16_t, op2: u16) -> svuint16_t {
    svbgrp_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbgrp_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svbgrp_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svbgrp_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbgrp_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svbgrp_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-bitperm")]
//#[cfg_attr(test, assert_instr(bgrp))]
pub unsafe fn svbgrp_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svbgrp_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl1n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbsl1n_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbsl1n_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl1n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbsl1n_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbsl1n_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl1n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbsl1n_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbsl1n_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl1n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbsl1n_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbsl1n_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbsl1n_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbsl1n_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbsl1n_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbsl1n_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbsl1n_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbsl1n_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbsl1n_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl1n))]
pub unsafe fn svbsl1n_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbsl1n_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl2n_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbsl2n_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbsl2n_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl2n_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbsl2n_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbsl2n_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl2n_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbsl2n_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbsl2n_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl2n_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbsl2n_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbsl2n_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbsl2n_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbsl2n_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbsl2n_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbsl2n_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbsl2n_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbsl2n_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbsl2n_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl2n))]
pub unsafe fn svbsl2n_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbsl2n_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svbsl_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svbsl_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svbsl_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svbsl_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svbsl_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svbsl_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svbsl_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svbsl_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svbsl_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svbsl_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svbsl_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svbsl_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svbsl_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svbsl_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svbsl_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(bsl))]
pub unsafe fn svbsl_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svbsl_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_s8<const IMM_ROTATION: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcadd_s8(op1: svint8_t, op2: svint8_t, imm_rotation: i32) -> svint8_t;
    }
    unsafe { _svcadd_s8(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_s16<const IMM_ROTATION: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcadd_s16(op1: svint16_t, op2: svint16_t, imm_rotation: i32) -> svint16_t;
    }
    unsafe { _svcadd_s16(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_s32<const IMM_ROTATION: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcadd_s32(op1: svint32_t, op2: svint32_t, imm_rotation: i32) -> svint32_t;
    }
    unsafe { _svcadd_s32(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_s64<const IMM_ROTATION: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcadd_s64(op1: svint64_t, op2: svint64_t, imm_rotation: i32) -> svint64_t;
    }
    unsafe { _svcadd_s64(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_u8<const IMM_ROTATION: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s8::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_u16<const IMM_ROTATION: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s16::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_u32<const IMM_ROTATION: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s32::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cadd, IMM_ROTATION = 90))]
pub unsafe fn svcadd_u64<const IMM_ROTATION: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    unsafe { svcadd_s64::<IMM_ROTATION>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cdot, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svcdot_lane_s32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint8_t,
    op3: svint8_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svcdot_lane_s32(
            op1: svint32_t,
            op2: svint8_t,
            op3: svint8_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcdot_lane_s32(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cdot, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svcdot_lane_s64<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svcdot_lane_s64(
            op1: svint64_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svcdot_lane_s64(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cdot, IMM_ROTATION = 90))]
pub unsafe fn svcdot_s32<const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint8_t,
    op3: svint8_t,
) -> svint32_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcdot_s32(
            op1: svint32_t,
            op2: svint8_t,
            op3: svint8_t,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcdot_s32(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cdot, IMM_ROTATION = 90))]
pub unsafe fn svcdot_s64<const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint64_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcdot_s64(
            op1: svint64_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svcdot_s64(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svcmla_lane_s16<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svcmla_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svcmla_lane_s16(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svcmla_lane_s32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svcmla_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcmla_lane_s32(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svcmla_lane_u16<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    unsafe {
        svcmla_lane_s16::<IMM_INDEX, IMM_ROTATION>(
            op1.as_signed(),
            op2.as_signed(),
            op3.as_signed(),
        )
        .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svcmla_lane_u32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    unsafe {
        svcmla_lane_s32::<IMM_INDEX, IMM_ROTATION>(
            op1.as_signed(),
            op2.as_signed(),
            op3.as_signed(),
        )
        .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_s8<const IMM_ROTATION: i32>(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcmla_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t, imm_rotation: i32) -> svint8_t;
    }
    unsafe { _svcmla_s8(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_s16<const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcmla_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svcmla_s16(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_s32<const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcmla_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svcmla_s32(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_s64<const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcmla_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svcmla_s64(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_u8<const IMM_ROTATION: i32>(
    op1: svuint8_t,
    op2: svuint8_t,
    op3: svuint8_t,
) -> svuint8_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    unsafe {
        svcmla_s8::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_u16<const IMM_ROTATION: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    unsafe {
        svcmla_s16::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_u32<const IMM_ROTATION: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    unsafe {
        svcmla_s32::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(cmla, IMM_ROTATION = 90))]
pub unsafe fn svcmla_u64<const IMM_ROTATION: i32>(
    op1: svuint64_t,
    op2: svuint64_t,
    op3: svuint64_t,
) -> svuint64_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    unsafe {
        svcmla_s64::<IMM_ROTATION>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtlt))]
pub unsafe fn svcvtlt_f64_f32_m(inactive: svfloat64_t, pg: svbool_t, op: svfloat32_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcvtlt_f64_f32_m(inactive: svfloat64_t, pg: svbool2_t, op: svfloat32_t)
            -> svfloat64_t;
    }
    unsafe { _svcvtlt_f64_f32_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtlt))]
pub unsafe fn svcvtlt_f64_f32_x(pg: svbool_t, op: svfloat32_t) -> svfloat64_t {
    unsafe { svcvtlt_f64_f32_m(simd_reinterpret(op), pg, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtnt))]
pub unsafe fn svcvtnt_f32_f64_m(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcvtnt_f32_f64_m(even: svfloat32_t, pg: svbool2_t, op: svfloat64_t) -> svfloat32_t;
    }
    unsafe { _svcvtnt_f32_f64_m(even, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtnt))]
pub unsafe fn svcvtnt_f32_f64_x(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    svcvtnt_f32_f64_m(even, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtx))]
pub unsafe fn svcvtx_f32_f64_m(inactive: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcvtx_f32_f64_m(inactive: svfloat32_t, pg: svbool2_t, op: svfloat64_t) -> svfloat32_t;
    }
    unsafe { _svcvtx_f32_f64_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtx))]
pub unsafe fn svcvtx_f32_f64_x(pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    unsafe { svcvtx_f32_f64_m(simd_reinterpret(op), pg, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtx))]
pub unsafe fn svcvtx_f32_f64_z(pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    svcvtx_f32_f64_m(svdup_n_f32(0.0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtxnt))]
pub unsafe fn svcvtxnt_f32_f64_m(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svcvtxnt_f32_f64_m(even: svfloat32_t, pg: svbool2_t, op: svfloat64_t) -> svfloat32_t;
    }
    unsafe { _svcvtxnt_f32_f64_m(even, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fcvtxnt))]
pub unsafe fn svcvtxnt_f32_f64_x(even: svfloat32_t, pg: svbool_t, op: svfloat64_t) -> svfloat32_t {
    svcvtxnt_f32_f64_m(even, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveor3_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _sveor3_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    sveor3_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveor3_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _sveor3_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    sveor3_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveor3_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _sveor3_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    sveor3_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveor3_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _sveor3_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    sveor3_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { sveor3_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    sveor3_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { sveor3_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    sveor3_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { sveor3_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    sveor3_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { sveor3_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eor3))]
pub unsafe fn sveor3_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    sveor3_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_s8(odd: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveorbt_s8(odd: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _sveorbt_s8(odd, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_s8(odd: svint8_t, op1: svint8_t, op2: i8) -> svint8_t {
    sveorbt_s8(odd, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_s16(odd: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveorbt_s16(odd: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _sveorbt_s16(odd, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_s16(odd: svint16_t, op1: svint16_t, op2: i16) -> svint16_t {
    sveorbt_s16(odd, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_s32(odd: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveorbt_s32(odd: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _sveorbt_s32(odd, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_s32(odd: svint32_t, op1: svint32_t, op2: i32) -> svint32_t {
    sveorbt_s32(odd, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_s64(odd: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveorbt_s64(odd: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _sveorbt_s64(odd, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_s64(odd: svint64_t, op1: svint64_t, op2: i64) -> svint64_t {
    sveorbt_s64(odd, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_u8(odd: svuint8_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { sveorbt_s8(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_u8(odd: svuint8_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    sveorbt_u8(odd, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_u16(odd: svuint16_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    unsafe { sveorbt_s16(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_u16(odd: svuint16_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    sveorbt_u16(odd, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_u32(odd: svuint32_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { sveorbt_s32(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_u32(odd: svuint32_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    sveorbt_u32(odd, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_u64(odd: svuint64_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { sveorbt_s64(odd.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eorbt))]
pub unsafe fn sveorbt_n_u64(odd: svuint64_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    sveorbt_u64(odd, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_s8(even: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveortb_s8(even: svint8_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _sveortb_s8(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_s8(even: svint8_t, op1: svint8_t, op2: i8) -> svint8_t {
    sveortb_s8(even, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_s16(even: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveortb_s16(even: svint16_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _sveortb_s16(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_s16(even: svint16_t, op1: svint16_t, op2: i16) -> svint16_t {
    sveortb_s16(even, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_s32(even: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveortb_s32(even: svint32_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _sveortb_s32(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_s32(even: svint32_t, op1: svint32_t, op2: i32) -> svint32_t {
    sveortb_s32(even, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_s64(even: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _sveortb_s64(even: svint64_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _sveortb_s64(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_s64(even: svint64_t, op1: svint64_t, op2: i64) -> svint64_t {
    sveortb_s64(even, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_u8(even: svuint8_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { sveortb_s8(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_u8(even: svuint8_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    sveortb_u8(even, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_u16(even: svuint16_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    unsafe { sveortb_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_u16(even: svuint16_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    sveortb_u16(even, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_u32(even: svuint32_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { sveortb_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_u32(even: svuint32_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    sveortb_u32(even, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_u64(even: svuint64_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { sveortb_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(eortb))]
pub unsafe fn sveortb_n_u64(even: svuint64_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    sveortb_u64(even, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhadd_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhadd_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhadd_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhadd_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhadd_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhadd_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhadd_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhadd_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhadd_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhadd_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhadd_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhadd_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhadd_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhadd_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhadd_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhadd_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhadd_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhadd_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhadd_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shadd))]
pub unsafe fn svhadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhadd_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhadd_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhadd_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhadd_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhadd_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhadd_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhadd_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhadd_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhadd_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhadd_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhadd_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhadd_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhadd_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhadd_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhadd_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhadd_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhadd_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhadd_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhadd_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhadd_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhadd))]
pub unsafe fn svhadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhadd_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(histcnt))]
pub unsafe fn svhistcnt_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svhistcnt_s32_z(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhistcnt_s32_z(pg.into(), op1, op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(histcnt))]
pub unsafe fn svhistcnt_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svhistcnt_s64_z(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhistcnt_s64_z(pg.into(), op1, op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(histcnt))]
pub unsafe fn svhistcnt_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    unsafe { svhistcnt_s32_z(pg, op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(histcnt))]
pub unsafe fn svhistcnt_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { svhistcnt_s64_z(pg, op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(histseg))]
pub unsafe fn svhistseg_s8(op1: svint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svhistseg_s8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhistseg_s8(op1, op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(histseg))]
pub unsafe fn svhistseg_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    unsafe { svhistseg_s8(op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsub_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsub_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsub_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsub_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsub_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsub_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsub_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsub_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsub_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsub_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsub_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsub_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsub_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsub_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsub_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsub_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsub_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsub_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsub_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsub_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsub_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsub_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsub_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsub_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsub_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsub_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsub_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsub_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsub_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsub_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsub_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsub_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsub_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsub_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsub_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsub_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsub_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsub_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsub_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsub_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsub_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsub_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsub_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsub_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsub_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsub_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsub_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsub_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsub_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsub_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsub_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsubr_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsubr_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsubr_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsubr_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svhsubr_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svhsubr_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsubr_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsubr_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsubr_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsubr_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svhsubr_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svhsubr_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsubr_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsubr_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsubr_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsubr_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svhsubr_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svhsubr_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsubr_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsubr_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsubr_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsubr_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svhsubr_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shsub))]
pub unsafe fn svhsubr_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svhsubr_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svhsubr_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsubr_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsubr_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsubr_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svhsubr_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svhsubr_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svhsubr_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsubr_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsubr_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsubr_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svhsubr_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svhsubr_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svhsubr_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsubr_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsubr_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsubr_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svhsubr_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svhsubr_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svhsubr_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svhsubr_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsubr_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsubr_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsubr_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svhsubr_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uhsub))]
pub unsafe fn svhsubr_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svhsubr_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64index_f64(
    pg: svbool_t,
    base: *const f64,
    indices: svint64_t,
) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_s64index_f64(
            pg: svbool2_t,
            base: *const f64,
            indices: svint64_t,
        ) -> svfloat64_t;
    }
    _svldnt1_gather_s64index_f64(pg.into(), base, indices)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64index_s64(
    pg: svbool_t,
    base: *const i64,
    indices: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_s64index_s64(
            pg: svbool2_t,
            base: *const i64,
            indices: svint64_t,
        ) -> svint64_t;
    }
    _svldnt1_gather_s64index_s64(pg.into(), base, indices)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64index_u64(
    pg: svbool_t,
    base: *const u64,
    indices: svint64_t,
) -> svuint64_t {
    svldnt1_gather_s64index_s64(pg, base.as_signed(), indices).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64index_f64(
    pg: svbool_t,
    base: *const f64,
    indices: svuint64_t,
) -> svfloat64_t {
    svldnt1_gather_s64index_f64(pg, base, indices.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64index_s64(
    pg: svbool_t,
    base: *const i64,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1_gather_s64index_s64(pg, base, indices.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64index_u64(
    pg: svbool_t,
    base: *const u64,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1_gather_s64index_s64(pg, base.as_signed(), indices.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64offset_f64(
    pg: svbool_t,
    base: *const f64,
    offsets: svint64_t,
) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_s64offset_f64(
            pg: svbool2_t,
            base: *const f64,
            offsets: svint64_t,
        ) -> svfloat64_t;
    }
    _svldnt1_gather_s64offset_f64(pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i64,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i64,
            offsets: svint64_t,
        ) -> svint64_t;
    }
    _svldnt1_gather_s64offset_s64(pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u64,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1_gather_s64offset_s64(pg, base.as_signed(), offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32offset_f32(
    pg: svbool_t,
    base: *const f32,
    offsets: svuint32_t,
) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_u32offset_f32(
            pg: svbool4_t,
            base: *const f32,
            offsets: svint32_t,
        ) -> svfloat32_t;
    }
    _svldnt1_gather_u32offset_f32(pg.into(), base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32offset_s32(
    pg: svbool_t,
    base: *const i32,
    offsets: svuint32_t,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_u32offset_s32(
            pg: svbool4_t,
            base: *const i32,
            offsets: svint32_t,
        ) -> svint32_t;
    }
    _svldnt1_gather_u32offset_s32(pg.into(), base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32offset_u32(
    pg: svbool_t,
    base: *const u32,
    offsets: svuint32_t,
) -> svuint32_t {
    svldnt1_gather_u32offset_s32(pg, base.as_signed(), offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64offset_f64(
    pg: svbool_t,
    base: *const f64,
    offsets: svuint64_t,
) -> svfloat64_t {
    svldnt1_gather_s64offset_f64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i64,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u64,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1_gather_s64offset_s64(pg, base.as_signed(), offsets.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_f32(pg: svbool_t, bases: svuint32_t) -> svfloat32_t {
    svldnt1_gather_u32base_offset_f32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1_gather_u32base_offset_s32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1_gather_u32base_offset_u32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_f64(pg: svbool_t, bases: svuint64_t) -> svfloat64_t {
    svldnt1_gather_u64base_offset_f64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_index_f32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svfloat32_t {
    svldnt1_gather_u32base_offset_f32(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svint32_t {
    svldnt1_gather_u32base_offset_s32(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svuint32_t {
    svldnt1_gather_u32base_offset_u32(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_index_f64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svfloat64_t {
    svldnt1_gather_u64base_offset_f64(pg, bases, index.unchecked_shl(3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_offset_f32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_u32base_offset_f32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> svfloat32_t;
    }
    _svldnt1_gather_u32base_offset_f32(pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_u32base_offset_s32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> svint32_t;
    }
    _svldnt1_gather_u32base_offset_s32(pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    svldnt1_gather_u32base_offset_s32(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_offset_f64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_u64base_offset_f64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> svfloat64_t;
    }
    _svldnt1_gather_u64base_offset_f64(pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> svint64_t;
    }
    _svldnt1_gather_u64base_offset_s64(pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1d))]
pub unsafe fn svldnt1_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i8,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sb_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i8,
            offsets: svint64_t,
        ) -> nxv2i8;
    }
    simd_cast(_svldnt1sb_gather_s64offset_s64(pg.into(), base, offsets))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i16,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sh_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i16,
            offsets: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast(_svldnt1sh_gather_s64offset_s64(pg.into(), base, offsets))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64offset_s64(
    pg: svbool_t,
    base: *const i32,
    offsets: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sw_gather_s64offset_s64(
            pg: svbool2_t,
            base: *const i32,
            offsets: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast(_svldnt1sw_gather_s64offset_s64(pg.into(), base, offsets))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_s64offset_u64(
    pg: svbool_t,
    base: *const i8,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1sb_gather_s64offset_s64(pg, base, offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64offset_u64(
    pg: svbool_t,
    base: *const i16,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64offset_s64(pg, base, offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64offset_u64(
    pg: svbool_t,
    base: *const i32,
    offsets: svint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64offset_s64(pg, base, offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32offset_s32(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint32_t,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sb_gather_u32offset_s32(
            pg: svbool4_t,
            base: *const i8,
            offsets: svint32_t,
        ) -> nxv4i8;
    }
    simd_cast(_svldnt1sb_gather_u32offset_s32(
        pg.into(),
        base,
        offsets.as_signed(),
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32offset_s32(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint32_t,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sh_gather_u32offset_s32(
            pg: svbool4_t,
            base: *const i16,
            offsets: svint32_t,
        ) -> nxv4i16;
    }
    simd_cast(_svldnt1sh_gather_u32offset_s32(
        pg.into(),
        base,
        offsets.as_signed(),
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32offset_u32(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint32_t,
) -> svuint32_t {
    svldnt1sb_gather_u32offset_s32(pg, base, offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32offset_u32(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint32_t,
) -> svuint32_t {
    svldnt1sh_gather_u32offset_s32(pg, base, offsets).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1sb_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1sh_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64offset_s64(
    pg: svbool_t,
    base: *const i32,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1sw_gather_s64offset_s64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64offset_u64(
    pg: svbool_t,
    base: *const i8,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1sb_gather_s64offset_s64(pg, base, offsets.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64offset_u64(
    pg: svbool_t,
    base: *const i16,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64offset_s64(pg, base, offsets.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64offset_u64(
    pg: svbool_t,
    base: *const i32,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64offset_s64(pg, base, offsets.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sb_gather_u32base_offset_s32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i8;
    }
    simd_cast(_svldnt1sb_gather_u32base_offset_s32(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sh_gather_u32base_offset_s32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i16;
    }
    simd_cast(_svldnt1sh_gather_u32base_offset_s32(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    svldnt1sb_gather_u32base_offset_s32(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    svldnt1sh_gather_u32base_offset_s32(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sb_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i8;
    }
    simd_cast(_svldnt1sb_gather_u64base_offset_s64(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sh_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i16;
    }
    simd_cast(_svldnt1sh_gather_u64base_offset_s64(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sw_gather_u64base_offset_s64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i32;
    }
    simd_cast(_svldnt1sw_gather_u64base_offset_s64(
        pg.into(),
        bases.as_signed(),
        offset,
    ))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1sb_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1sh_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    svldnt1sw_gather_u64base_offset_s64(pg, bases, offset).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1sb_gather_u32base_offset_s32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1sh_gather_u32base_offset_s32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1sb_gather_u32base_offset_u32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1sh_gather_u32base_offset_u32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1sb_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1sh_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1sw_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sb))]
pub unsafe fn svldnt1sb_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1sb_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1sh_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1sw_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64index_s64(
    pg: svbool_t,
    base: *const i16,
    indices: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sh_gather_s64index_s64(
            pg: svbool2_t,
            base: *const i16,
            indices: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast(_svldnt1sh_gather_s64index_s64(pg.into(), base, indices))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64index_s64(
    pg: svbool_t,
    base: *const i32,
    indices: svint64_t,
) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1sw_gather_s64index_s64(
            pg: svbool2_t,
            base: *const i32,
            indices: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast(_svldnt1sw_gather_s64index_s64(pg.into(), base, indices))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_s64index_u64(
    pg: svbool_t,
    base: *const i16,
    indices: svint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64index_s64(pg, base, indices).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_s64index_u64(
    pg: svbool_t,
    base: *const i32,
    indices: svint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64index_s64(pg, base, indices).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64index_s64(
    pg: svbool_t,
    base: *const i16,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1sh_gather_s64index_s64(pg, base, indices.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64index_s64(
    pg: svbool_t,
    base: *const i32,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1sw_gather_s64index_s64(pg, base, indices.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64index_u64(
    pg: svbool_t,
    base: *const i16,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1sh_gather_s64index_s64(pg, base, indices.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64index_u64(
    pg: svbool_t,
    base: *const i32,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1sw_gather_s64index_s64(pg, base, indices.as_signed()).as_unsigned()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svint32_t {
    svldnt1sh_gather_u32base_offset_s32(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svuint32_t {
    svldnt1sh_gather_u32base_offset_u32(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1sh_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1sw_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sh))]
pub unsafe fn svldnt1sh_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1sh_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1sw))]
pub unsafe fn svldnt1sw_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1sw_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_s64offset_s64(
    pg: svbool_t,
    base: *const u8,
    offsets: svint64_t,
) -> svint64_t {
    svldnt1ub_gather_s64offset_u64(pg, base, offsets).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64offset_s64(
    pg: svbool_t,
    base: *const u16,
    offsets: svint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64offset_u64(pg, base, offsets).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64offset_s64(
    pg: svbool_t,
    base: *const u32,
    offsets: svint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64offset_u64(pg, base, offsets).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u8,
    offsets: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1ub_gather_s64offset_u64(
            pg: svbool2_t,
            base: *const i8,
            offsets: svint64_t,
        ) -> nxv2i8;
    }
    simd_cast::<nxv2u8, _>(
        _svldnt1ub_gather_s64offset_u64(pg.into(), base.as_signed(), offsets).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u16,
    offsets: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uh_gather_s64offset_u64(
            pg: svbool2_t,
            base: *const i16,
            offsets: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast::<nxv2u16, _>(
        _svldnt1uh_gather_s64offset_u64(pg.into(), base.as_signed(), offsets).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64offset_u64(
    pg: svbool_t,
    base: *const u32,
    offsets: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uw_gather_s64offset_u64(
            pg: svbool2_t,
            base: *const i32,
            offsets: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast::<nxv2u32, _>(
        _svldnt1uw_gather_s64offset_u64(pg.into(), base.as_signed(), offsets).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32offset_s32(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint32_t,
) -> svint32_t {
    svldnt1ub_gather_u32offset_u32(pg, base, offsets).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32offset_s32(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint32_t,
) -> svint32_t {
    svldnt1uh_gather_u32offset_u32(pg, base, offsets).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32offset_u32(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint32_t,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1ub_gather_u32offset_u32(
            pg: svbool4_t,
            base: *const i8,
            offsets: svint32_t,
        ) -> nxv4i8;
    }
    simd_cast::<nxv4u8, _>(
        _svldnt1ub_gather_u32offset_u32(pg.into(), base.as_signed(), offsets.as_signed())
            .as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32offset_u32(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint32_t,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uh_gather_u32offset_u32(
            pg: svbool4_t,
            base: *const i16,
            offsets: svint32_t,
        ) -> nxv4i16;
    }
    simd_cast::<nxv4u16, _>(
        _svldnt1uh_gather_u32offset_u32(pg.into(), base.as_signed(), offsets.as_signed())
            .as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64offset_s64(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1ub_gather_s64offset_u64(pg, base, offsets.as_signed()).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64offset_s64(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64offset_u64(pg, base, offsets.as_signed()).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64offset_s64(
    pg: svbool_t,
    base: *const u32,
    offsets: svuint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64offset_u64(pg, base, offsets.as_signed()).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u8,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1ub_gather_s64offset_u64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u16,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1uh_gather_s64offset_u64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64offset_u64(
    pg: svbool_t,
    base: *const u32,
    offsets: svuint64_t,
) -> svuint64_t {
    svldnt1uw_gather_s64offset_u64(pg, base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    svldnt1ub_gather_u32base_offset_u32(pg, bases, offset).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svint32_t {
    svldnt1uh_gather_u32base_offset_u32(pg, bases, offset).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1ub_gather_u32base_offset_u32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i8;
    }
    simd_cast::<nxv4u8, _>(
        _svldnt1ub_gather_u32base_offset_u32(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uh_gather_u32base_offset_u32(
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        ) -> nxv4i16;
    }
    simd_cast::<nxv4u16, _>(
        _svldnt1uh_gather_u32base_offset_u32(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    svldnt1ub_gather_u64base_offset_u64(pg, bases, offset).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    svldnt1uh_gather_u64base_offset_u64(pg, bases, offset).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svint64_t {
    svldnt1uw_gather_u64base_offset_u64(pg, bases, offset).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1ub_gather_u64base_offset_u64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i8;
    }
    simd_cast::<nxv2u8, _>(
        _svldnt1ub_gather_u64base_offset_u64(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uh_gather_u64base_offset_u64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i16;
    }
    simd_cast::<nxv2u16, _>(
        _svldnt1uh_gather_u64base_offset_u64(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uw_gather_u64base_offset_u64(
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        ) -> nxv2i32;
    }
    simd_cast::<nxv2u32, _>(
        _svldnt1uw_gather_u64base_offset_u64(pg.into(), bases.as_signed(), offset).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1ub_gather_u32base_offset_s32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_s32(pg: svbool_t, bases: svuint32_t) -> svint32_t {
    svldnt1uh_gather_u32base_offset_s32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1ub_gather_u32base_offset_u32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_u32(pg: svbool_t, bases: svuint32_t) -> svuint32_t {
    svldnt1uh_gather_u32base_offset_u32(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1ub_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1uh_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_s64(pg: svbool_t, bases: svuint64_t) -> svint64_t {
    svldnt1uw_gather_u64base_offset_s64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1b))]
pub unsafe fn svldnt1ub_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1ub_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1uh_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_u64(pg: svbool_t, bases: svuint64_t) -> svuint64_t {
    svldnt1uw_gather_u64base_offset_u64(pg, bases, 0)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64index_s64(
    pg: svbool_t,
    base: *const u16,
    indices: svint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64index_u64(pg, base, indices).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64index_s64(
    pg: svbool_t,
    base: *const u32,
    indices: svint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64index_u64(pg, base, indices).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_s64index_u64(
    pg: svbool_t,
    base: *const u16,
    indices: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uh_gather_s64index_u64(
            pg: svbool2_t,
            base: *const i16,
            indices: svint64_t,
        ) -> nxv2i16;
    }
    simd_cast::<nxv2u16, _>(
        _svldnt1uh_gather_s64index_u64(pg.into(), base.as_signed(), indices).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_s64index_u64(
    pg: svbool_t,
    base: *const u32,
    indices: svint64_t,
) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svldnt1uw_gather_s64index_u64(
            pg: svbool2_t,
            base: *const i32,
            indices: svint64_t,
        ) -> nxv2i32;
    }
    simd_cast::<nxv2u32, _>(
        _svldnt1uw_gather_s64index_u64(pg.into(), base.as_signed(), indices).as_unsigned(),
    )
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64index_s64(
    pg: svbool_t,
    base: *const u16,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1uh_gather_s64index_u64(pg, base, indices.as_signed()).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64index_s64(
    pg: svbool_t,
    base: *const u32,
    indices: svuint64_t,
) -> svint64_t {
    svldnt1uw_gather_s64index_u64(pg, base, indices.as_signed()).as_signed()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64index_u64(
    pg: svbool_t,
    base: *const u16,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1uh_gather_s64index_u64(pg, base, indices.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64index_u64(
    pg: svbool_t,
    base: *const u32,
    indices: svuint64_t,
) -> svuint64_t {
    svldnt1uw_gather_s64index_u64(pg, base, indices.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svint32_t {
    svldnt1uh_gather_u32base_offset_s32(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
) -> svuint32_t {
    svldnt1uh_gather_u32base_offset_u32(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1uh_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svint64_t {
    svldnt1uw_gather_u64base_offset_s64(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1h))]
pub unsafe fn svldnt1uh_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1uh_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(1))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ldnt1w))]
pub unsafe fn svldnt1uw_gather_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
) -> svuint64_t {
    svldnt1uw_gather_u64base_offset_u64(pg, bases, index.unchecked_shl(2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(flogb))]
pub unsafe fn svlogb_f32_m(inactive: svint32_t, pg: svbool_t, op: svfloat32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svlogb_f32_m(inactive: svint32_t, pg: svbool4_t, op: svfloat32_t) -> svint32_t;
    }
    unsafe { _svlogb_f32_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(flogb))]
pub unsafe fn svlogb_f32_x(pg: svbool_t, op: svfloat32_t) -> svint32_t {
    unsafe { svlogb_f32_m(simd_reinterpret(op), pg, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(flogb))]
pub unsafe fn svlogb_f32_z(pg: svbool_t, op: svfloat32_t) -> svint32_t {
    svlogb_f32_m(svdup_n_s32(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(flogb))]
pub unsafe fn svlogb_f64_m(inactive: svint64_t, pg: svbool_t, op: svfloat64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svlogb_f64_m(inactive: svint64_t, pg: svbool2_t, op: svfloat64_t) -> svint64_t;
    }
    unsafe { _svlogb_f64_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(flogb))]
pub unsafe fn svlogb_f64_x(pg: svbool_t, op: svfloat64_t) -> svint64_t {
    unsafe { svlogb_f64_m(simd_reinterpret(op), pg, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(flogb))]
pub unsafe fn svlogb_f64_z(pg: svbool_t, op: svfloat64_t) -> svint64_t {
    svlogb_f64_m(svdup_n_s64(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(match))]
pub unsafe fn svmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t;
    }
    unsafe { _svmatch_s8(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(match))]
pub unsafe fn svmatch_s16(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmatch_s16(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svbool8_t;
    }
    unsafe { _svmatch_s16(pg.into(), op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(match))]
pub unsafe fn svmatch_u8(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svbool_t {
    unsafe { svmatch_s8(pg, op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(match))]
pub unsafe fn svmatch_u16(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svbool_t {
    unsafe { svmatch_s16(pg, op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxnmp))]
pub unsafe fn svmaxnmp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmaxnmp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svmaxnmp_f32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxnmp))]
pub unsafe fn svmaxnmp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svmaxnmp_f32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxnmp))]
pub unsafe fn svmaxnmp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmaxnmp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svmaxnmp_f64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxnmp))]
pub unsafe fn svmaxnmp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svmaxnmp_f64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxp))]
pub unsafe fn svmaxp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svmaxp_f32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxp))]
pub unsafe fn svmaxp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svmaxp_f32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxp))]
pub unsafe fn svmaxp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svmaxp_f64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmaxp))]
pub unsafe fn svmaxp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svmaxp_f64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svmaxp_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svmaxp_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svmaxp_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svmaxp_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svmaxp_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svmaxp_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svmaxp_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smaxp))]
pub unsafe fn svmaxp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svmaxp_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svmaxp_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svmaxp_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svmaxp_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svmaxp_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svmaxp_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svmaxp_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmaxp_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svmaxp_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umaxp))]
pub unsafe fn svmaxp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svmaxp_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminnmp))]
pub unsafe fn svminnmp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svminnmp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svminnmp_f32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminnmp))]
pub unsafe fn svminnmp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svminnmp_f32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminnmp))]
pub unsafe fn svminnmp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svminnmp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svminnmp_f64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminnmp))]
pub unsafe fn svminnmp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svminnmp_f64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminp))]
pub unsafe fn svminp_f32_m(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_f32_m(pg: svbool4_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t;
    }
    unsafe { _svminp_f32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminp))]
pub unsafe fn svminp_f32_x(pg: svbool_t, op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    svminp_f32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminp))]
pub unsafe fn svminp_f64_m(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_f64_m(pg: svbool2_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t;
    }
    unsafe { _svminp_f64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fminp))]
pub unsafe fn svminp_f64_x(pg: svbool_t, op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    svminp_f64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svminp_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svminp_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svminp_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svminp_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svminp_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svminp_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svminp_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sminp))]
pub unsafe fn svminp_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svminp_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svminp_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svminp_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svminp_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svminp_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svminp_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svminp_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svminp_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svminp_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uminp))]
pub unsafe fn svminp_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svminp_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub unsafe fn svmla_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmla_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svmla_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub unsafe fn svmla_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmla_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmla_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub unsafe fn svmla_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmla_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmla_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub unsafe fn svmla_lane_u16<const IMM_INDEX: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    unsafe {
        svmla_lane_s16::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub unsafe fn svmla_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    unsafe {
        svmla_lane_s32::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mla, IMM_INDEX = 0))]
pub unsafe fn svmla_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint64_t,
    op3: svuint64_t,
) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    unsafe {
        svmla_lane_s64::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb, IMM_INDEX = 0))]
pub unsafe fn svmlalb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlalb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb, IMM_INDEX = 0))]
pub unsafe fn svmlalb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlalb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb, IMM_INDEX = 0))]
pub unsafe fn svmlalb_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalb_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlalb_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb, IMM_INDEX = 0))]
pub unsafe fn svmlalb_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalb_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlalb_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb))]
pub unsafe fn svmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalb_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb))]
pub unsafe fn svmlalb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlalb_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb))]
pub unsafe fn svmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalb_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb))]
pub unsafe fn svmlalb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlalb_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb))]
pub unsafe fn svmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalb_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalb))]
pub unsafe fn svmlalb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlalb_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb))]
pub unsafe fn svmlalb_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalb_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalb_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb))]
pub unsafe fn svmlalb_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlalb_u16(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb))]
pub unsafe fn svmlalb_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalb_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb))]
pub unsafe fn svmlalb_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlalb_u32(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb))]
pub unsafe fn svmlalb_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalb_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalb))]
pub unsafe fn svmlalb_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlalb_u64(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt, IMM_INDEX = 0))]
pub unsafe fn svmlalt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlalt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt, IMM_INDEX = 0))]
pub unsafe fn svmlalt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlalt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt, IMM_INDEX = 0))]
pub unsafe fn svmlalt_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalt_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlalt_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt, IMM_INDEX = 0))]
pub unsafe fn svmlalt_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlalt_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlalt_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt))]
pub unsafe fn svmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt))]
pub unsafe fn svmlalt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlalt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt))]
pub unsafe fn svmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt))]
pub unsafe fn svmlalt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlalt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt))]
pub unsafe fn svmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlalt))]
pub unsafe fn svmlalt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlalt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt))]
pub unsafe fn svmlalt_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalt_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlalt_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt))]
pub unsafe fn svmlalt_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlalt_u16(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt))]
pub unsafe fn svmlalt_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalt_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlalt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt))]
pub unsafe fn svmlalt_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlalt_u32(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt))]
pub unsafe fn svmlalt_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlalt_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlalt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlalt))]
pub unsafe fn svmlalt_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlalt_u64(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub unsafe fn svmls_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmls_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svmls_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub unsafe fn svmls_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmls_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmls_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub unsafe fn svmls_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmls_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmls_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub unsafe fn svmls_lane_u16<const IMM_INDEX: i32>(
    op1: svuint16_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    unsafe {
        svmls_lane_s16::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub unsafe fn svmls_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    unsafe {
        svmls_lane_s32::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mls, IMM_INDEX = 0))]
pub unsafe fn svmls_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint64_t,
    op3: svuint64_t,
) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    unsafe {
        svmls_lane_s64::<IMM_INDEX>(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb, IMM_INDEX = 0))]
pub unsafe fn svmlslb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlslb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb, IMM_INDEX = 0))]
pub unsafe fn svmlslb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlslb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb, IMM_INDEX = 0))]
pub unsafe fn svmlslb_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslb_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlslb_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb, IMM_INDEX = 0))]
pub unsafe fn svmlslb_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslb_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlslb_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb))]
pub unsafe fn svmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslb_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb))]
pub unsafe fn svmlslb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlslb_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb))]
pub unsafe fn svmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslb_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb))]
pub unsafe fn svmlslb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlslb_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb))]
pub unsafe fn svmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslb_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslb))]
pub unsafe fn svmlslb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlslb_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb))]
pub unsafe fn svmlslb_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslb_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslb_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb))]
pub unsafe fn svmlslb_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlslb_u16(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb))]
pub unsafe fn svmlslb_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslb_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb))]
pub unsafe fn svmlslb_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlslb_u32(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb))]
pub unsafe fn svmlslb_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslb_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslb))]
pub unsafe fn svmlslb_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlslb_u64(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt, IMM_INDEX = 0))]
pub unsafe fn svmlslt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svmlslt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt, IMM_INDEX = 0))]
pub unsafe fn svmlslt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svmlslt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt, IMM_INDEX = 0))]
pub unsafe fn svmlslt_lane_u32<const IMM_INDEX: i32>(
    op1: svuint32_t,
    op2: svuint16_t,
    op3: svuint16_t,
) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslt_lane_u32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe {
        _svmlslt_lane_u32(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt, IMM_INDEX = 0))]
pub unsafe fn svmlslt_lane_u64<const IMM_INDEX: i32>(
    op1: svuint64_t,
    op2: svuint32_t,
    op3: svuint32_t,
) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmlslt_lane_u64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe {
        _svmlslt_lane_u64(op1.as_signed(), op2.as_signed(), op3.as_signed(), IMM_INDEX)
            .as_unsigned()
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt))]
pub unsafe fn svmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt))]
pub unsafe fn svmlslt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svmlslt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt))]
pub unsafe fn svmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt))]
pub unsafe fn svmlslt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svmlslt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt))]
pub unsafe fn svmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smlslt))]
pub unsafe fn svmlslt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svmlslt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt))]
pub unsafe fn svmlslt_u16(op1: svuint16_t, op2: svuint8_t, op3: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslt_u16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svmlslt_u16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt))]
pub unsafe fn svmlslt_n_u16(op1: svuint16_t, op2: svuint8_t, op3: u8) -> svuint16_t {
    svmlslt_u16(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt))]
pub unsafe fn svmlslt_u32(op1: svuint32_t, op2: svuint16_t, op3: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslt_u32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svmlslt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt))]
pub unsafe fn svmlslt_n_u32(op1: svuint32_t, op2: svuint16_t, op3: u16) -> svuint32_t {
    svmlslt_u32(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt))]
pub unsafe fn svmlslt_u64(op1: svuint64_t, op2: svuint32_t, op3: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmlslt_u64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svmlslt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umlslt))]
pub unsafe fn svmlslt_n_u64(op1: svuint64_t, op2: svuint32_t, op3: u32) -> svuint64_t {
    svmlslt_u64(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllb))]
pub unsafe fn svmovlb_s16(op: svint8_t) -> svint16_t {
    svshllb_n_s16::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllb))]
pub unsafe fn svmovlb_s32(op: svint16_t) -> svint32_t {
    svshllb_n_s32::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllb))]
pub unsafe fn svmovlb_s64(op: svint32_t) -> svint64_t {
    svshllb_n_s64::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllb))]
pub unsafe fn svmovlb_u16(op: svuint8_t) -> svuint16_t {
    svshllb_n_u16::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllb))]
pub unsafe fn svmovlb_u32(op: svuint16_t) -> svuint32_t {
    svshllb_n_u32::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllb))]
pub unsafe fn svmovlb_u64(op: svuint32_t) -> svuint64_t {
    svshllb_n_u64::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllt))]
pub unsafe fn svmovlt_s16(op: svint8_t) -> svint16_t {
    svshllt_n_s16::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllt))]
pub unsafe fn svmovlt_s32(op: svint16_t) -> svint32_t {
    svshllt_n_s32::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllt))]
pub unsafe fn svmovlt_s64(op: svint32_t) -> svint64_t {
    svshllt_n_s64::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllt))]
pub unsafe fn svmovlt_u16(op: svuint8_t) -> svuint16_t {
    svshllt_n_u16::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllt))]
pub unsafe fn svmovlt_u32(op: svuint16_t) -> svuint32_t {
    svshllt_n_u32::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllt))]
pub unsafe fn svmovlt_u64(op: svuint32_t) -> svuint64_t {
    svshllt_n_u64::<0>(op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_f32<const IMM_INDEX: i32>(op1: svfloat32_t, op2: svfloat32_t) -> svfloat32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmul_lane_f32(op1: svfloat32_t, op2: svfloat32_t, imm_index: i32) -> svfloat32_t;
    }
    unsafe { _svmul_lane_f32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(fmul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_f64<const IMM_INDEX: i32>(op1: svfloat64_t, op2: svfloat64_t) -> svfloat64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmul_lane_f64(op1: svfloat64_t, op2: svfloat64_t, imm_index: i32) -> svfloat64_t;
    }
    unsafe { _svmul_lane_f64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_s16<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmul_lane_s16(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint16_t;
    }
    unsafe { _svmul_lane_s16(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_s32<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmul_lane_s32(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmul_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_s64<const IMM_INDEX: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmul_lane_s64(op1: svint64_t, op2: svint64_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmul_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_u16<const IMM_INDEX: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    unsafe { svmul_lane_s16::<IMM_INDEX>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_u32<const IMM_INDEX: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    unsafe { svmul_lane_s32::<IMM_INDEX>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(mul, IMM_INDEX = 0))]
pub unsafe fn svmul_lane_u64<const IMM_INDEX: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    unsafe { svmul_lane_s64::<IMM_INDEX>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb, IMM_INDEX = 0))]
pub unsafe fn svmullb_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullb_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullb_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb, IMM_INDEX = 0))]
pub unsafe fn svmullb_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullb_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullb_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb, IMM_INDEX = 0))]
pub unsafe fn svmullb_lane_u32<const IMM_INDEX: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullb_lane_u32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullb_lane_u32(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb, IMM_INDEX = 0))]
pub unsafe fn svmullb_lane_u64<const IMM_INDEX: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullb_lane_u64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullb_lane_u64(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb))]
pub unsafe fn svmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb))]
pub unsafe fn svmullb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svmullb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb))]
pub unsafe fn svmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb))]
pub unsafe fn svmullb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svmullb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb))]
pub unsafe fn svmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullb))]
pub unsafe fn svmullb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svmullb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb))]
pub unsafe fn svmullb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb))]
pub unsafe fn svmullb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svmullb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb))]
pub unsafe fn svmullb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb))]
pub unsafe fn svmullb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svmullb_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb))]
pub unsafe fn svmullb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullb))]
pub unsafe fn svmullb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svmullb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt, IMM_INDEX = 0))]
pub unsafe fn svmullt_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullt_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullt_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt, IMM_INDEX = 0))]
pub unsafe fn svmullt_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullt_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullt_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt, IMM_INDEX = 0))]
pub unsafe fn svmullt_lane_u32<const IMM_INDEX: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullt_lane_u32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svmullt_lane_u32(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt, IMM_INDEX = 0))]
pub unsafe fn svmullt_lane_u64<const IMM_INDEX: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svmullt_lane_u64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svmullt_lane_u64(op1.as_signed(), op2.as_signed(), IMM_INDEX).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt))]
pub unsafe fn svmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt))]
pub unsafe fn svmullt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svmullt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt))]
pub unsafe fn svmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt))]
pub unsafe fn svmullt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svmullt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt))]
pub unsafe fn svmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(smullt))]
pub unsafe fn svmullt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svmullt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt))]
pub unsafe fn svmullt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svmullt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt))]
pub unsafe fn svmullt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svmullt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt))]
pub unsafe fn svmullt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svmullt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt))]
pub unsafe fn svmullt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svmullt_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt))]
pub unsafe fn svmullt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svmullt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svmullt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(umullt))]
pub unsafe fn svmullt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svmullt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svnbsl_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svnbsl_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svnbsl_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svnbsl_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svnbsl_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svnbsl_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svnbsl_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svnbsl_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svnbsl_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svnbsl_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svnbsl_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svnbsl_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_u8(op1: svuint8_t, op2: svuint8_t, op3: svuint8_t) -> svuint8_t {
    unsafe { svnbsl_s8(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_u8(op1: svuint8_t, op2: svuint8_t, op3: u8) -> svuint8_t {
    svnbsl_u8(op1, op2, svdup_n_u8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_u16(op1: svuint16_t, op2: svuint16_t, op3: svuint16_t) -> svuint16_t {
    unsafe { svnbsl_s16(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_u16(op1: svuint16_t, op2: svuint16_t, op3: u16) -> svuint16_t {
    svnbsl_u16(op1, op2, svdup_n_u16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    unsafe { svnbsl_s32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svnbsl_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    unsafe { svnbsl_s64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nbsl))]
pub unsafe fn svnbsl_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svnbsl_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nmatch))]
pub unsafe fn svnmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svnmatch_s8(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svbool_t;
    }
    unsafe { _svnmatch_s8(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nmatch))]
pub unsafe fn svnmatch_s16(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svnmatch_s16(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svbool8_t;
    }
    unsafe { _svnmatch_s16(pg.into(), op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nmatch))]
pub unsafe fn svnmatch_u8(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svbool_t {
    unsafe { svnmatch_s8(pg, op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(nmatch))]
pub unsafe fn svnmatch_u16(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svbool_t {
    unsafe { svnmatch_s16(pg, op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(pmul))]
pub unsafe fn svpmul_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svpmul_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svpmul_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(pmul))]
pub unsafe fn svpmul_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svpmul_u8(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_pair_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svpmullb_pair_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svpmullb_pair_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_pair_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svpmullb_pair_u8(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_pair_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svpmullb_pair_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svpmullb_pair_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_pair_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svpmullb_pair_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_pair_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svpmullb_pair_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svpmullb_pair_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_pair_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svpmullb_pair_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    unsafe { simd_reinterpret(svpmullb_pair_u8(op1, op2)) }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svpmullb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    unsafe { simd_reinterpret(svpmullb_pair_u32(op1, op2)) }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullb))]
pub unsafe fn svpmullb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svpmullb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_pair_u8(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svpmullt_pair_u8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svpmullt_pair_u8(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_pair_n_u8(op1: svuint8_t, op2: u8) -> svuint8_t {
    svpmullt_pair_u8(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_pair_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svpmullt_pair_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svpmullt_pair_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_pair_n_u32(op1: svuint32_t, op2: u32) -> svuint32_t {
    svpmullt_pair_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_pair_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svpmullt_pair_u64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svpmullt_pair_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_pair_n_u64(op1: svuint64_t, op2: u64) -> svuint64_t {
    svpmullt_pair_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    unsafe { simd_reinterpret(svpmullt_pair_u8(op1, op2)) }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svpmullt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    unsafe { simd_reinterpret(svpmullt_pair_u32(op1, op2)) }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-aes")]
//#[cfg_attr(test, assert_instr(pmullt))]
pub unsafe fn svpmullt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svpmullt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqabs_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t;
    }
    unsafe { _svqabs_s8_m(inactive, pg, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s8_x(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqabs_s8_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s8_z(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqabs_s8_m(svdup_n_s8(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s16_m(inactive: svint16_t, pg: svbool_t, op: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqabs_s16_m(inactive: svint16_t, pg: svbool8_t, op: svint16_t) -> svint16_t;
    }
    unsafe { _svqabs_s16_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s16_x(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqabs_s16_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s16_z(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqabs_s16_m(svdup_n_s16(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s32_m(inactive: svint32_t, pg: svbool_t, op: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqabs_s32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svqabs_s32_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s32_x(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqabs_s32_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s32_z(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqabs_s32_m(svdup_n_s32(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s64_m(inactive: svint64_t, pg: svbool_t, op: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqabs_s64_m(inactive: svint64_t, pg: svbool2_t, op: svint64_t) -> svint64_t;
    }
    unsafe { _svqabs_s64_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s64_x(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqabs_s64_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqabs))]
pub unsafe fn svqabs_s64_z(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqabs_s64_m(svdup_n_s64(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqadd_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqadd_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqadd_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqadd_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqadd_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqadd_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqadd_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqadd_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqadd_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqadd_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqadd_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqadd_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqadd_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqadd_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqadd_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqadd_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqadd_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqadd_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqadd_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqadd))]
pub unsafe fn svqadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqadd_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqadd_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqadd_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqadd_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqadd_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqadd_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqadd_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqadd_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqadd_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqadd_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqadd_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqadd_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqadd_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqadd_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqadd_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqadd_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqadd_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqadd_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqadd_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqadd_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqadd))]
pub unsafe fn svqadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqadd_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub unsafe fn svqcadd_s8<const IMM_ROTATION: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqcadd_s8(op1: svint8_t, op2: svint8_t, imm_rotation: i32) -> svint8_t;
    }
    unsafe { _svqcadd_s8(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub unsafe fn svqcadd_s16<const IMM_ROTATION: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqcadd_s16(op1: svint16_t, op2: svint16_t, imm_rotation: i32) -> svint16_t;
    }
    unsafe { _svqcadd_s16(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub unsafe fn svqcadd_s32<const IMM_ROTATION: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqcadd_s32(op1: svint32_t, op2: svint32_t, imm_rotation: i32) -> svint32_t;
    }
    unsafe { _svqcadd_s32(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqcadd, IMM_ROTATION = 90))]
pub unsafe fn svqcadd_s64<const IMM_ROTATION: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert!(IMM_ROTATION == 90 || IMM_ROTATION == 270);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqcadd_s64(op1: svint64_t, op2: svint64_t, imm_rotation: i32) -> svint64_t;
    }
    unsafe { _svqcadd_s64(op1, op2, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb, IMM_INDEX = 0))]
pub unsafe fn svqdmlalb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlalb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb, IMM_INDEX = 0))]
pub unsafe fn svqdmlalb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlalb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb))]
pub unsafe fn svqdmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlalb_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb))]
pub unsafe fn svqdmlalb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlalb_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb))]
pub unsafe fn svqdmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlalb_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb))]
pub unsafe fn svqdmlalb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlalb_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb))]
pub unsafe fn svqdmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlalb_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalb))]
pub unsafe fn svqdmlalb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlalb_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub unsafe fn svqdmlalbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlalbt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub unsafe fn svqdmlalbt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlalbt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub unsafe fn svqdmlalbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlalbt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub unsafe fn svqdmlalbt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlalbt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub unsafe fn svqdmlalbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlalbt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalbt))]
pub unsafe fn svqdmlalbt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlalbt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt, IMM_INDEX = 0))]
pub unsafe fn svqdmlalt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlalt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt, IMM_INDEX = 0))]
pub unsafe fn svqdmlalt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlalt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt))]
pub unsafe fn svqdmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlalt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt))]
pub unsafe fn svqdmlalt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlalt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt))]
pub unsafe fn svqdmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlalt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt))]
pub unsafe fn svqdmlalt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlalt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt))]
pub unsafe fn svqdmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlalt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlalt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlalt))]
pub unsafe fn svqdmlalt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlalt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb, IMM_INDEX = 0))]
pub unsafe fn svqdmlslb_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslb_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlslb_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb, IMM_INDEX = 0))]
pub unsafe fn svqdmlslb_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslb_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlslb_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb))]
pub unsafe fn svqdmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslb_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlslb_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb))]
pub unsafe fn svqdmlslb_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlslb_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb))]
pub unsafe fn svqdmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslb_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlslb_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb))]
pub unsafe fn svqdmlslb_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlslb_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb))]
pub unsafe fn svqdmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslb_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlslb_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslb))]
pub unsafe fn svqdmlslb_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlslb_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub unsafe fn svqdmlslbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslbt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlslbt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub unsafe fn svqdmlslbt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlslbt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub unsafe fn svqdmlslbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslbt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlslbt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub unsafe fn svqdmlslbt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlslbt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub unsafe fn svqdmlslbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslbt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlslbt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslbt))]
pub unsafe fn svqdmlslbt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlslbt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt, IMM_INDEX = 0))]
pub unsafe fn svqdmlslt_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslt_lane_s32(
            op1: svint32_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqdmlslt_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt, IMM_INDEX = 0))]
pub unsafe fn svqdmlslt_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslt_lane_s64(
            op1: svint64_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqdmlslt_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt))]
pub unsafe fn svqdmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslt_s16(op1: svint16_t, op2: svint8_t, op3: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmlslt_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt))]
pub unsafe fn svqdmlslt_n_s16(op1: svint16_t, op2: svint8_t, op3: i8) -> svint16_t {
    svqdmlslt_s16(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt))]
pub unsafe fn svqdmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslt_s32(op1: svint32_t, op2: svint16_t, op3: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmlslt_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt))]
pub unsafe fn svqdmlslt_n_s32(op1: svint32_t, op2: svint16_t, op3: i16) -> svint32_t {
    svqdmlslt_s32(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt))]
pub unsafe fn svqdmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmlslt_s64(op1: svint64_t, op2: svint32_t, op3: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmlslt_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmlslt))]
pub unsafe fn svqdmlslt_n_s64(op1: svint64_t, op2: svint32_t, op3: i32) -> svint64_t {
    svqdmlslt_s64(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh, IMM_INDEX = 0))]
pub unsafe fn svqdmulh_lane_s16<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_lane_s16(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint16_t;
    }
    unsafe { _svqdmulh_lane_s16(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh, IMM_INDEX = 0))]
pub unsafe fn svqdmulh_lane_s32<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_lane_s32(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqdmulh_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh, IMM_INDEX = 0))]
pub unsafe fn svqdmulh_lane_s64<const IMM_INDEX: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_lane_s64(op1: svint64_t, op2: svint64_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqdmulh_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqdmulh_s8(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_n_s8(op1: svint8_t, op2: i8) -> svint8_t {
    svqdmulh_s8(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqdmulh_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_n_s16(op1: svint16_t, op2: i16) -> svint16_t {
    svqdmulh_s16(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqdmulh_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_n_s32(op1: svint32_t, op2: i32) -> svint32_t {
    svqdmulh_s32(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqdmulh_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmulh))]
pub unsafe fn svqdmulh_n_s64(op1: svint64_t, op2: i64) -> svint64_t {
    svqdmulh_s64(op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb, IMM_INDEX = 0))]
pub unsafe fn svqdmullb_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullb_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqdmullb_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb, IMM_INDEX = 0))]
pub unsafe fn svqdmullb_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullb_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqdmullb_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb))]
pub unsafe fn svqdmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmullb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb))]
pub unsafe fn svqdmullb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svqdmullb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb))]
pub unsafe fn svqdmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmullb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb))]
pub unsafe fn svqdmullb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svqdmullb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb))]
pub unsafe fn svqdmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmullb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullb))]
pub unsafe fn svqdmullb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svqdmullb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt, IMM_INDEX = 0))]
pub unsafe fn svqdmullt_lane_s32<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullt_lane_s32(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqdmullt_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt, IMM_INDEX = 0))]
pub unsafe fn svqdmullt_lane_s64<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullt_lane_s64(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqdmullt_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt))]
pub unsafe fn svqdmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svqdmullt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt))]
pub unsafe fn svqdmullt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svqdmullt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt))]
pub unsafe fn svqdmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svqdmullt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt))]
pub unsafe fn svqdmullt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svqdmullt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt))]
pub unsafe fn svqdmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqdmullt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svqdmullt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqdmullt))]
pub unsafe fn svqdmullt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svqdmullt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqneg_s8_m(inactive: svint8_t, pg: svbool_t, op: svint8_t) -> svint8_t;
    }
    unsafe { _svqneg_s8_m(inactive, pg, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s8_x(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqneg_s8_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s8_z(pg: svbool_t, op: svint8_t) -> svint8_t {
    svqneg_s8_m(svdup_n_s8(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s16_m(inactive: svint16_t, pg: svbool_t, op: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqneg_s16_m(inactive: svint16_t, pg: svbool8_t, op: svint16_t) -> svint16_t;
    }
    unsafe { _svqneg_s16_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s16_x(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqneg_s16_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s16_z(pg: svbool_t, op: svint16_t) -> svint16_t {
    svqneg_s16_m(svdup_n_s16(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s32_m(inactive: svint32_t, pg: svbool_t, op: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqneg_s32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svqneg_s32_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s32_x(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqneg_s32_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s32_z(pg: svbool_t, op: svint32_t) -> svint32_t {
    svqneg_s32_m(svdup_n_s32(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s64_m(inactive: svint64_t, pg: svbool_t, op: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqneg_s64_m(inactive: svint64_t, pg: svbool2_t, op: svint64_t) -> svint64_t;
    }
    unsafe { _svqneg_s64_m(inactive, pg.into(), op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s64_x(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqneg_s64_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqneg))]
pub unsafe fn svqneg_s64_z(pg: svbool_t, op: svint64_t) -> svint64_t {
    svqneg_s64_m(svdup_n_s64(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svqrdcmlah_lane_s16<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdcmlah_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdcmlah_lane_s16(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_INDEX = 0, IMM_ROTATION = 90))]
pub unsafe fn svqrdcmlah_lane_s32<const IMM_INDEX: i32, const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdcmlah_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_index: i32,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdcmlah_lane_s32(op1, op2, op3, IMM_INDEX, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub unsafe fn svqrdcmlah_s8<const IMM_ROTATION: i32>(
    op1: svint8_t,
    op2: svint8_t,
    op3: svint8_t,
) -> svint8_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdcmlah_s8(
            op1: svint8_t,
            op2: svint8_t,
            op3: svint8_t,
            imm_rotation: i32,
        ) -> svint8_t;
    }
    unsafe { _svqrdcmlah_s8(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub unsafe fn svqrdcmlah_s16<const IMM_ROTATION: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdcmlah_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            imm_rotation: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdcmlah_s16(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub unsafe fn svqrdcmlah_s32<const IMM_ROTATION: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdcmlah_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            imm_rotation: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdcmlah_s32(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdcmlah, IMM_ROTATION = 90))]
pub unsafe fn svqrdcmlah_s64<const IMM_ROTATION: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    /*//static_assert!(
        IMM_ROTATION == 0 || IMM_ROTATION == 90 || IMM_ROTATION == 180 || IMM_ROTATION == 270
    );*/
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdcmlah_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            imm_rotation: i32,
        ) -> svint64_t;
    }
    unsafe { _svqrdcmlah_s64(op1, op2, op3, IMM_ROTATION) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah, IMM_INDEX = 0))]
pub unsafe fn svqrdmlah_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdmlah_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah, IMM_INDEX = 0))]
pub unsafe fn svqrdmlah_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdmlah_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah, IMM_INDEX = 0))]
pub unsafe fn svqrdmlah_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqrdmlah_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svqrdmlah_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svqrdmlah_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svqrdmlah_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svqrdmlah_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svqrdmlah_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svqrdmlah_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlah_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svqrdmlah_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlah))]
pub unsafe fn svqrdmlah_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svqrdmlah_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh, IMM_INDEX = 0))]
pub unsafe fn svqrdmlsh_lane_s16<const IMM_INDEX: i32>(
    op1: svint16_t,
    op2: svint16_t,
    op3: svint16_t,
) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_lane_s16(
            op1: svint16_t,
            op2: svint16_t,
            op3: svint16_t,
            IMM_INDEX: i32,
        ) -> svint16_t;
    }
    unsafe { _svqrdmlsh_lane_s16(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh, IMM_INDEX = 0))]
pub unsafe fn svqrdmlsh_lane_s32<const IMM_INDEX: i32>(
    op1: svint32_t,
    op2: svint32_t,
    op3: svint32_t,
) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_lane_s32(
            op1: svint32_t,
            op2: svint32_t,
            op3: svint32_t,
            IMM_INDEX: i32,
        ) -> svint32_t;
    }
    unsafe { _svqrdmlsh_lane_s32(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh, IMM_INDEX = 0))]
pub unsafe fn svqrdmlsh_lane_s64<const IMM_INDEX: i32>(
    op1: svint64_t,
    op2: svint64_t,
    op3: svint64_t,
) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_lane_s64(
            op1: svint64_t,
            op2: svint64_t,
            op3: svint64_t,
            IMM_INDEX: i32,
        ) -> svint64_t;
    }
    unsafe { _svqrdmlsh_lane_s64(op1, op2, op3, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_s8(op1: svint8_t, op2: svint8_t, op3: svint8_t) -> svint8_t;
    }
    unsafe { _svqrdmlsh_s8(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_n_s8(op1: svint8_t, op2: svint8_t, op3: i8) -> svint8_t {
    svqrdmlsh_s8(op1, op2, svdup_n_s8(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_s16(op1: svint16_t, op2: svint16_t, op3: svint16_t) -> svint16_t;
    }
    unsafe { _svqrdmlsh_s16(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_n_s16(op1: svint16_t, op2: svint16_t, op3: i16) -> svint16_t {
    svqrdmlsh_s16(op1, op2, svdup_n_s16(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_s32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svqrdmlsh_s32(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_n_s32(op1: svint32_t, op2: svint32_t, op3: i32) -> svint32_t {
    svqrdmlsh_s32(op1, op2, svdup_n_s32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmlsh_s64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svqrdmlsh_s64(op1, op2, op3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmlsh))]
pub unsafe fn svqrdmlsh_n_s64(op1: svint64_t, op2: svint64_t, op3: i64) -> svint64_t {
    svqrdmlsh_s64(op1, op2, svdup_n_s64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh, IMM_INDEX = 0))]
pub unsafe fn svqrdmulh_lane_s16<const IMM_INDEX: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM_INDEX, 0, 7);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_lane_s16(op1: svint16_t, op2: svint16_t, imm_index: i32) -> svint16_t;
    }
    unsafe { _svqrdmulh_lane_s16(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh, IMM_INDEX = 0))]
pub unsafe fn svqrdmulh_lane_s32<const IMM_INDEX: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM_INDEX, 0, 3);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_lane_s32(op1: svint32_t, op2: svint32_t, imm_index: i32) -> svint32_t;
    }
    unsafe { _svqrdmulh_lane_s32(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh, IMM_INDEX = 0))]
pub unsafe fn svqrdmulh_lane_s64<const IMM_INDEX: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM_INDEX, 0, 1);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_lane_s64(op1: svint64_t, op2: svint64_t, imm_index: i32) -> svint64_t;
    }
    unsafe { _svqrdmulh_lane_s64(op1, op2, IMM_INDEX) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_s8(op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqrdmulh_s8(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_n_s8(op1: svint8_t, op2: i8) -> svint8_t {
    svqrdmulh_s8(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_s16(op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqrdmulh_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_n_s16(op1: svint16_t, op2: i16) -> svint16_t {
    svqrdmulh_s16(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_s32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqrdmulh_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_n_s32(op1: svint32_t, op2: i32) -> svint32_t {
    svqrdmulh_s32(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrdmulh_s64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqrdmulh_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrdmulh))]
pub unsafe fn svqrdmulh_n_s64(op1: svint64_t, op2: i64) -> svint64_t {
    svqrdmulh_s64(op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqrshl_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqrshl_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqrshl_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqrshl_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqrshl_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqrshl_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqrshl_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqrshl_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqrshl_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqrshl_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqrshl_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqrshl_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqrshl_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqrshl_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqrshl_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqrshl_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqrshl_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqrshl_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqrshl_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqrshl_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqrshl_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqrshl_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqrshl_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshl))]
pub unsafe fn svqrshl_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqrshl_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqrshl_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqrshl_u8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqrshl_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqrshl_u8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqrshl_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqrshl_u8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqrshl_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqrshl_u16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqrshl_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqrshl_u16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqrshl_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqrshl_u16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqrshl_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqrshl_u32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqrshl_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqrshl_u32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqrshl_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqrshl_u32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqrshl_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqrshl_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqrshl_u64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqrshl_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqrshl_u64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqrshl_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshl))]
pub unsafe fn svqrshl_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqrshl_u64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrnb, IMM2 = 1))]
pub unsafe fn svqrshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnb_n_s16(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrnb, IMM2 = 1))]
pub unsafe fn svqrshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnb_n_s32(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrnb, IMM2 = 1))]
pub unsafe fn svqrshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnb_n_s64(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshrnb, IMM2 = 1))]
pub unsafe fn svqrshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnb_n_u16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnb_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshrnb, IMM2 = 1))]
pub unsafe fn svqrshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnb_n_u32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnb_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshrnb, IMM2 = 1))]
pub unsafe fn svqrshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnb_n_u64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnb_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrnt, IMM2 = 1))]
pub unsafe fn svqrshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnt_n_s16(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrnt, IMM2 = 1))]
pub unsafe fn svqrshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnt_n_s32(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrnt, IMM2 = 1))]
pub unsafe fn svqrshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnt_n_s64(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshrnt, IMM2 = 1))]
pub unsafe fn svqrshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnt_n_u16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrnt_n_u16(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshrnt, IMM2 = 1))]
pub unsafe fn svqrshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnt_n_u32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrnt_n_u32(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqrshrnt, IMM2 = 1))]
pub unsafe fn svqrshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrnt_n_u64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrnt_n_u64(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrunb, IMM2 = 1))]
pub unsafe fn svqrshrunb_n_s16<const IMM2: i32>(op1: svint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrunb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrunb_n_s16(op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrunb, IMM2 = 1))]
pub unsafe fn svqrshrunb_n_s32<const IMM2: i32>(op1: svint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrunb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrunb_n_s32(op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrunb, IMM2 = 1))]
pub unsafe fn svqrshrunb_n_s64<const IMM2: i32>(op1: svint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrunb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrunb_n_s64(op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrunt, IMM2 = 1))]
pub unsafe fn svqrshrunt_n_s16<const IMM2: i32>(even: svuint8_t, op1: svint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrunt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqrshrunt_n_s16(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrunt, IMM2 = 1))]
pub unsafe fn svqrshrunt_n_s32<const IMM2: i32>(even: svuint16_t, op1: svint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrunt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqrshrunt_n_s32(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqrshrunt, IMM2 = 1))]
pub unsafe fn svqrshrunt_n_s64<const IMM2: i32>(even: svuint32_t, op1: svint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqrshrunt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqrshrunt_n_s64(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqshl_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqshl_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqshl_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqshl_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqshl_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqshl_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqshl_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqshl_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqshl_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqshl_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqshl_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqshl_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqshl_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqshl_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqshl_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqshl_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqshl_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqshl_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqshl_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqshl_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqshl_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqshl_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqshl_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshl))]
pub unsafe fn svqshl_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqshl_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqshl_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqshl_u8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqshl_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqshl_u8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svqshl_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svqshl_u8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqshl_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqshl_u16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqshl_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqshl_u16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svqshl_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svqshl_u16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqshl_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqshl_u32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqshl_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqshl_u32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svqshl_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svqshl_u32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshl_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqshl_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqshl_u64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqshl_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqshl_u64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svqshl_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshl))]
pub unsafe fn svqshl_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svqshl_u64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s8_m<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svuint8_t {
    //static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshlu_n_s8_m(pg: svbool_t, op1: svint8_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshlu_n_s8_m(pg, op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s8_x<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svuint8_t {
    svqshlu_n_s8_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s8_z<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svuint8_t {
    svqshlu_n_s8_m::<IMM2>(pg, svsel_s8(pg, op1, svdup_n_s8(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s16_m<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svuint16_t {
    //static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshlu_n_s16_m(pg: svbool8_t, op1: svint16_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshlu_n_s16_m(pg.into(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s16_x<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svuint16_t {
    svqshlu_n_s16_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s16_z<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svuint16_t {
    svqshlu_n_s16_m::<IMM2>(pg, svsel_s16(pg, op1, svdup_n_s16(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s32_m<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svuint32_t {
    //static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshlu_n_s32_m(pg: svbool4_t, op1: svint32_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshlu_n_s32_m(pg.into(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s32_x<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svuint32_t {
    svqshlu_n_s32_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s32_z<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svuint32_t {
    svqshlu_n_s32_m::<IMM2>(pg, svsel_s32(pg, op1, svdup_n_s32(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s64_m<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svuint64_t {
    //static_assert_range!(IMM2, 0, 63);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqshlu_n_s64_m(pg: svbool2_t, op1: svint64_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svqshlu_n_s64_m(pg.into(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s64_x<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svuint64_t {
    svqshlu_n_s64_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshlu, IMM2 = 0))]
pub unsafe fn svqshlu_n_s64_z<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svuint64_t {
    svqshlu_n_s64_m::<IMM2>(pg, svsel_s64(pg, op1, svdup_n_s64(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrnb, IMM2 = 1))]
pub unsafe fn svqshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnb_n_s16(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrnb, IMM2 = 1))]
pub unsafe fn svqshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnb_n_s32(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrnb, IMM2 = 1))]
pub unsafe fn svqshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnb_n_s64(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshrnb, IMM2 = 1))]
pub unsafe fn svqshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnb_n_u16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnb_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshrnb, IMM2 = 1))]
pub unsafe fn svqshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnb_n_u32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnb_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshrnb, IMM2 = 1))]
pub unsafe fn svqshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnb_n_u64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnb_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrnt, IMM2 = 1))]
pub unsafe fn svqshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnt_n_s16(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrnt, IMM2 = 1))]
pub unsafe fn svqshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnt_n_s32(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrnt, IMM2 = 1))]
pub unsafe fn svqshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnt_n_s64(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshrnt, IMM2 = 1))]
pub unsafe fn svqshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnt_n_u16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrnt_n_u16(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshrnt, IMM2 = 1))]
pub unsafe fn svqshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnt_n_u32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrnt_n_u32(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqshrnt, IMM2 = 1))]
pub unsafe fn svqshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrnt_n_u64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrnt_n_u64(even.as_signed(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrunb, IMM2 = 1))]
pub unsafe fn svqshrunb_n_s16<const IMM2: i32>(op1: svint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrunb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrunb_n_s16(op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrunb, IMM2 = 1))]
pub unsafe fn svqshrunb_n_s32<const IMM2: i32>(op1: svint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrunb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrunb_n_s32(op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrunb, IMM2 = 1))]
pub unsafe fn svqshrunb_n_s64<const IMM2: i32>(op1: svint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrunb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrunb_n_s64(op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrunt, IMM2 = 1))]
pub unsafe fn svqshrunt_n_s16<const IMM2: i32>(even: svuint8_t, op1: svint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrunt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svqshrunt_n_s16(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrunt, IMM2 = 1))]
pub unsafe fn svqshrunt_n_s32<const IMM2: i32>(even: svuint16_t, op1: svint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrunt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svqshrunt_n_s32(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqshrunt, IMM2 = 1))]
pub unsafe fn svqshrunt_n_s64<const IMM2: i32>(even: svuint32_t, op1: svint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqshrunt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svqshrunt_n_s64(even.as_signed(), op1, IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsub_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsub_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsub_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsub_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsub_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsub_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsub_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsub_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsub_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsub_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsub_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsub_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsub_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsub_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsub_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsub_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsub_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsub_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsub_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsub_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsub_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsub_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsub_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsub))]
pub unsafe fn svqsub_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsub_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsub_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsub_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsub_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsub_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsub_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsub_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsub_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsub_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsub_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsub_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsub_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsub_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsub_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsub_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsub_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsub_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsub_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsub_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsub_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsub_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsub_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsub_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsub_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsub_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsub))]
pub unsafe fn svqsub_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsub_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsubr_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsubr_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsubr_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsubr_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svqsubr_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svqsubr_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsubr_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsubr_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsubr_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsubr_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svqsubr_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svqsubr_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsubr_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsubr_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsubr_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsubr_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svqsubr_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svqsubr_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsubr_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsubr_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsubr_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsubr_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svqsubr_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqsubr))]
pub unsafe fn svqsubr_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svqsubr_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svqsubr_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsubr_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsubr_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsubr_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svqsubr_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svqsubr_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svqsubr_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsubr_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsubr_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsubr_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svqsubr_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svqsubr_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svqsubr_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsubr_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsubr_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsubr_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svqsubr_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svqsubr_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqsubr_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svqsubr_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsubr_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsubr_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsubr_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svqsubr_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqsubr))]
pub unsafe fn svqsubr_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svqsubr_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtnb))]
pub unsafe fn svqxtnb_s16(op: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnb_s16(op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnb_s16(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtnb))]
pub unsafe fn svqxtnb_s32(op: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnb_s32(op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnb_s32(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtnb))]
pub unsafe fn svqxtnb_s64(op: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnb_s64(op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnb_s64(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqxtnb))]
pub unsafe fn svqxtnb_u16(op: svuint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnb_u16(op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnb_u16(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqxtnb))]
pub unsafe fn svqxtnb_u32(op: svuint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnb_u32(op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnb_u32(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqxtnb))]
pub unsafe fn svqxtnb_u64(op: svuint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnb_u64(op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnb_u64(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtnt))]
pub unsafe fn svqxtnt_s16(even: svint8_t, op: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnt_s16(even: svint8_t, op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnt_s16(even, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtnt))]
pub unsafe fn svqxtnt_s32(even: svint16_t, op: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnt_s32(even: svint16_t, op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnt_s32(even, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtnt))]
pub unsafe fn svqxtnt_s64(even: svint32_t, op: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnt_s64(even: svint32_t, op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnt_s64(even, op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqxtnt))]
pub unsafe fn svqxtnt_u16(even: svuint8_t, op: svuint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnt_u16(even: svint8_t, op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtnt_u16(even.as_signed(), op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqxtnt))]
pub unsafe fn svqxtnt_u32(even: svuint16_t, op: svuint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnt_u32(even: svint16_t, op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtnt_u32(even.as_signed(), op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uqxtnt))]
pub unsafe fn svqxtnt_u64(even: svuint32_t, op: svuint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svqxtnt_u64(even: svint32_t, op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtnt_u64(even.as_signed(), op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtunb))]
pub unsafe fn svqxtunb_s16(op: svint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqxtunb_s16(op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtunb_s16(op).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtunb))]
pub unsafe fn svqxtunb_s32(op: svint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqxtunb_s32(op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtunb_s32(op).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtunb))]
pub unsafe fn svqxtunb_s64(op: svint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqxtunb_s64(op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtunb_s64(op).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtunt))]
pub unsafe fn svqxtunt_s16(even: svuint8_t, op: svint16_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqxtunt_s16(even: svint8_t, op: svint16_t) -> svint8_t;
    }
    unsafe { _svqxtunt_s16(even.as_signed(), op).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtunt))]
pub unsafe fn svqxtunt_s32(even: svuint16_t, op: svint32_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqxtunt_s32(even: svint16_t, op: svint32_t) -> svint16_t;
    }
    unsafe { _svqxtunt_s32(even.as_signed(), op).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sqxtunt))]
pub unsafe fn svqxtunt_s64(even: svuint32_t, op: svint64_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svqxtunt_s64(even: svint32_t, op: svint64_t) -> svint32_t;
    }
    unsafe { _svqxtunt_s64(even.as_signed(), op).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svraddhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svraddhnb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svraddhnb_s16(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svraddhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svraddhnb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svraddhnb_s32(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svraddhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svraddhnb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svraddhnb_s64(op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svraddhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svraddhnb_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svraddhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svraddhnb_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svraddhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnb))]
pub unsafe fn svraddhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svraddhnb_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svraddhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svraddhnt_s16(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svraddhnt_s16(even, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svraddhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svraddhnt_s32(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svraddhnt_s32(even, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svraddhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svraddhnt_s64(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svraddhnt_s64(even, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svraddhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svraddhnt_u16(even, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svraddhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svraddhnt_u32(even, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svraddhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(raddhnt))]
pub unsafe fn svraddhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svraddhnt_u64(even, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sha3")]
//#[cfg_attr(test, assert_instr(rax1))]
pub unsafe fn svrax1_s64(op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrax1_s64(op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrax1_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sha3")]
//#[cfg_attr(test, assert_instr(rax1))]
pub unsafe fn svrax1_u64(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    unsafe { svrax1_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urecpe))]
pub unsafe fn svrecpe_u32_m(inactive: svuint32_t, pg: svbool_t, op: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrecpe_u32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svrecpe_u32_m(inactive.as_signed(), pg.into(), op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urecpe))]
pub unsafe fn svrecpe_u32_x(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrecpe_u32_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urecpe))]
pub unsafe fn svrecpe_u32_z(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrecpe_u32_m(svdup_n_u32(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrhadd_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrhadd_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrhadd_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrhadd_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrhadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrhadd_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrhadd_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrhadd_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrhadd_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrhadd_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrhadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrhadd_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrhadd_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrhadd_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrhadd_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrhadd_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrhadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrhadd_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrhadd_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrhadd_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrhadd_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrhadd_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrhadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srhadd))]
pub unsafe fn svrhadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrhadd_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrhadd_u8_m(pg, op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svrhadd_u8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svrhadd_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svrhadd_u8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    svrhadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: u8) -> svuint8_t {
    svrhadd_u8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrhadd_u16_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svrhadd_u16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svrhadd_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svrhadd_u16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    svrhadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: u16) -> svuint16_t {
    svrhadd_u16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrhadd_u32_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svrhadd_u32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svrhadd_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svrhadd_u32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    svrhadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: u32) -> svuint32_t {
    svrhadd_u32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrhadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrhadd_u64_m(pg.into(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svrhadd_u64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svrhadd_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svrhadd_u64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    svrhadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urhadd))]
pub unsafe fn svrhadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: u64) -> svuint64_t {
    svrhadd_u64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrshl_s8_m(pg, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s8_m(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrshl_s8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s8_x(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrshl_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s8_x(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrshl_s8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s8_z(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t {
    svrshl_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s8_z(pg: svbool_t, op1: svint8_t, op2: i8) -> svint8_t {
    svrshl_s8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s16_m(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrshl_s16_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s16_m(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrshl_s16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s16_x(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrshl_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s16_x(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrshl_s16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s16_z(pg: svbool_t, op1: svint16_t, op2: svint16_t) -> svint16_t {
    svrshl_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s16_z(pg: svbool_t, op1: svint16_t, op2: i16) -> svint16_t {
    svrshl_s16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s32_m(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrshl_s32_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s32_m(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrshl_s32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s32_x(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrshl_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s32_x(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrshl_s32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s32_z(pg: svbool_t, op1: svint32_t, op2: svint32_t) -> svint32_t {
    svrshl_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s32_z(pg: svbool_t, op1: svint32_t, op2: i32) -> svint32_t {
    svrshl_s32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s64_m(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrshl_s64_m(pg.into(), op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s64_m(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrshl_s64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s64_x(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrshl_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s64_x(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrshl_s64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_s64_z(pg: svbool_t, op1: svint64_t, op2: svint64_t) -> svint64_t {
    svrshl_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshl))]
pub unsafe fn svrshl_n_s64_z(pg: svbool_t, op1: svint64_t, op2: i64) -> svint64_t {
    svrshl_s64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svrshl_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svrshl_u8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svrshl_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svrshl_u8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svrshl_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svrshl_u8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svrshl_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svrshl_u16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svrshl_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svrshl_u16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svrshl_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svrshl_u16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svrshl_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svrshl_u32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svrshl_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svrshl_u32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svrshl_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svrshl_u32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshl_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svrshl_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svrshl_u64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svrshl_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svrshl_u64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svrshl_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshl))]
pub unsafe fn svrshl_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svrshl_u64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s8_m<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_s8_m(pg: svbool_t, op1: svint8_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshr_n_s8_m(pg, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s8_x<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svint8_t {
    svrshr_n_s8_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s8_z<const IMM2: i32>(pg: svbool_t, op1: svint8_t) -> svint8_t {
    svrshr_n_s8_m::<IMM2>(pg, svsel_s8(pg, op1, svdup_n_s8(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s16_m<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_s16_m(pg: svbool8_t, op1: svint16_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshr_n_s16_m(pg.into(), op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s16_x<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svint16_t {
    svrshr_n_s16_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s16_z<const IMM2: i32>(pg: svbool_t, op1: svint16_t) -> svint16_t {
    svrshr_n_s16_m::<IMM2>(pg, svsel_s16(pg, op1, svdup_n_s16(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s32_m<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_s32_m(pg: svbool4_t, op1: svint32_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshr_n_s32_m(pg.into(), op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s32_x<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svint32_t {
    svrshr_n_s32_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s32_z<const IMM2: i32>(pg: svbool_t, op1: svint32_t) -> svint32_t {
    svrshr_n_s32_m::<IMM2>(pg, svsel_s32(pg, op1, svdup_n_s32(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s64_m<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svint64_t {
    //static_assert_range!(IMM2, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_s64_m(pg: svbool2_t, op1: svint64_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svrshr_n_s64_m(pg.into(), op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s64_x<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svint64_t {
    svrshr_n_s64_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srshr, IMM2 = 1))]
pub unsafe fn svrshr_n_s64_z<const IMM2: i32>(pg: svbool_t, op1: svint64_t) -> svint64_t {
    svrshr_n_s64_m::<IMM2>(pg, svsel_s64(pg, op1, svdup_n_s64(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u8_m<const IMM2: i32>(pg: svbool_t, op1: svuint8_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_u8_m(pg: svbool_t, op1: svint8_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshr_n_u8_m(pg, op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u8_x<const IMM2: i32>(pg: svbool_t, op1: svuint8_t) -> svuint8_t {
    svrshr_n_u8_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u8_z<const IMM2: i32>(pg: svbool_t, op1: svuint8_t) -> svuint8_t {
    svrshr_n_u8_m::<IMM2>(pg, svsel_u8(pg, op1, svdup_n_u8(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u16_m<const IMM2: i32>(pg: svbool_t, op1: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_u16_m(pg: svbool8_t, op1: svint16_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshr_n_u16_m(pg.into(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u16_x<const IMM2: i32>(pg: svbool_t, op1: svuint16_t) -> svuint16_t {
    svrshr_n_u16_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u16_z<const IMM2: i32>(pg: svbool_t, op1: svuint16_t) -> svuint16_t {
    svrshr_n_u16_m::<IMM2>(pg, svsel_u16(pg, op1, svdup_n_u16(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u32_m<const IMM2: i32>(pg: svbool_t, op1: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_u32_m(pg: svbool4_t, op1: svint32_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshr_n_u32_m(pg.into(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u32_x<const IMM2: i32>(pg: svbool_t, op1: svuint32_t) -> svuint32_t {
    svrshr_n_u32_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u32_z<const IMM2: i32>(pg: svbool_t, op1: svuint32_t) -> svuint32_t {
    svrshr_n_u32_m::<IMM2>(pg, svsel_u32(pg, op1, svdup_n_u32(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u64_m<const IMM2: i32>(pg: svbool_t, op1: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM2, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshr_n_u64_m(pg: svbool2_t, op1: svint64_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svrshr_n_u64_m(pg.into(), op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u64_x<const IMM2: i32>(pg: svbool_t, op1: svuint64_t) -> svuint64_t {
    svrshr_n_u64_m::<IMM2>(pg, op1)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(urshr, IMM2 = 1))]
pub unsafe fn svrshr_n_u64_z<const IMM2: i32>(pg: svbool_t, op1: svuint64_t) -> svuint64_t {
    svrshr_n_u64_m::<IMM2>(pg, svsel_u64(pg, op1, svdup_n_u64(0)))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub unsafe fn svrshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshrnb_n_s16(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub unsafe fn svrshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshrnb_n_s32(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub unsafe fn svrshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshrnb_n_s64(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub unsafe fn svrshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    unsafe { svrshrnb_n_s16::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub unsafe fn svrshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    unsafe { svrshrnb_n_s32::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnb, IMM2 = 1))]
pub unsafe fn svrshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    unsafe { svrshrnb_n_s64::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub unsafe fn svrshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svrshrnt_n_s16(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub unsafe fn svrshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svrshrnt_n_s32(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub unsafe fn svrshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svrshrnt_n_s64(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub unsafe fn svrshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    unsafe { svrshrnt_n_s16::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub unsafe fn svrshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    unsafe { svrshrnt_n_s32::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rshrnt, IMM2 = 1))]
pub unsafe fn svrshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    unsafe { svrshrnt_n_s64::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursqrte))]
pub unsafe fn svrsqrte_u32_m(inactive: svuint32_t, pg: svbool_t, op: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsqrte_u32_m(inactive: svint32_t, pg: svbool4_t, op: svint32_t) -> svint32_t;
    }
    unsafe { _svrsqrte_u32_m(inactive.as_signed(), pg.into(), op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursqrte))]
pub unsafe fn svrsqrte_u32_x(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrsqrte_u32_m(op, pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursqrte))]
pub unsafe fn svrsqrte_u32_z(pg: svbool_t, op: svuint32_t) -> svuint32_t {
    svrsqrte_u32_m(svdup_n_u32(0), pg, op)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub unsafe fn svrsra_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svrsra_n_s8(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub unsafe fn svrsra_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svrsra_n_s16(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub unsafe fn svrsra_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svrsra_n_s32(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(srsra, IMM3 = 1))]
pub unsafe fn svrsra_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svrsra_n_s64(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub unsafe fn svrsra_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    //static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_u8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svrsra_n_u8(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub unsafe fn svrsra_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_u16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svrsra_n_u16(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub unsafe fn svrsra_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_u32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svrsra_n_u32(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ursra, IMM3 = 1))]
pub unsafe fn svrsra_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svrsra_n_u64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svrsra_n_u64(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svrsubhnb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svrsubhnb_s16(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svrsubhnb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svrsubhnb_s32(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svrsubhnb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svrsubhnb_s64(op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svrsubhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svrsubhnb_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svrsubhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svrsubhnb_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svrsubhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnb))]
pub unsafe fn svrsubhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svrsubhnb_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svrsubhnt_s16(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svrsubhnt_s16(even, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svrsubhnt_s32(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svrsubhnt_s32(even, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svrsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svrsubhnt_s64(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svrsubhnt_s64(even, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svrsubhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svrsubhnt_u16(even, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svrsubhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svrsubhnt_u32(even, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svrsubhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(rsubhnt))]
pub unsafe fn svrsubhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svrsubhnt_u64(even, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclb))]
pub unsafe fn svsbclb_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsbclb_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svsbclb_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclb))]
pub unsafe fn svsbclb_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svsbclb_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclb))]
pub unsafe fn svsbclb_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsbclb_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svsbclb_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclb))]
pub unsafe fn svsbclb_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svsbclb_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclt))]
pub unsafe fn svsbclt_u32(op1: svuint32_t, op2: svuint32_t, op3: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsbclt_u32(op1: svint32_t, op2: svint32_t, op3: svint32_t) -> svint32_t;
    }
    unsafe { _svsbclt_u32(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclt))]
pub unsafe fn svsbclt_n_u32(op1: svuint32_t, op2: svuint32_t, op3: u32) -> svuint32_t {
    svsbclt_u32(op1, op2, svdup_n_u32(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclt))]
pub unsafe fn svsbclt_u64(op1: svuint64_t, op2: svuint64_t, op3: svuint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsbclt_u64(op1: svint64_t, op2: svint64_t, op3: svint64_t) -> svint64_t;
    }
    unsafe { _svsbclt_u64(op1.as_signed(), op2.as_signed(), op3.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sbclt))]
pub unsafe fn svsbclt_n_u64(op1: svuint64_t, op2: svuint64_t, op3: u64) -> svuint64_t {
    svsbclt_u64(op1, op2, svdup_n_u64(op3))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllb, IMM2 = 0))]
pub unsafe fn svshllb_n_s16<const IMM2: i32>(op1: svint8_t) -> svint16_t {
    //static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllb_n_s16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllb_n_s16(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllb, IMM2 = 0))]
pub unsafe fn svshllb_n_s32<const IMM2: i32>(op1: svint16_t) -> svint32_t {
    //static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllb_n_s32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllb_n_s32(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllb, IMM2 = 0))]
pub unsafe fn svshllb_n_s64<const IMM2: i32>(op1: svint32_t) -> svint64_t {
    //static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllb_n_s64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllb_n_s64(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllb, IMM2 = 0))]
pub unsafe fn svshllb_n_u16<const IMM2: i32>(op1: svuint8_t) -> svuint16_t {
    //static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllb_n_u16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllb_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllb, IMM2 = 0))]
pub unsafe fn svshllb_n_u32<const IMM2: i32>(op1: svuint16_t) -> svuint32_t {
    //static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllb_n_u32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllb_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllb, IMM2 = 0))]
pub unsafe fn svshllb_n_u64<const IMM2: i32>(op1: svuint32_t) -> svuint64_t {
    //static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllb_n_u64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllb_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllt, IMM2 = 0))]
pub unsafe fn svshllt_n_s16<const IMM2: i32>(op1: svint8_t) -> svint16_t {
    //static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllt_n_s16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllt_n_s16(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllt, IMM2 = 0))]
pub unsafe fn svshllt_n_s32<const IMM2: i32>(op1: svint16_t) -> svint32_t {
    //static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllt_n_s32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllt_n_s32(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sshllt, IMM2 = 0))]
pub unsafe fn svshllt_n_s64<const IMM2: i32>(op1: svint32_t) -> svint64_t {
    //static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllt_n_s64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllt_n_s64(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllt, IMM2 = 0))]
pub unsafe fn svshllt_n_u16<const IMM2: i32>(op1: svuint8_t) -> svuint16_t {
    //static_assert_range!(IMM2, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllt_n_u16(op1: svint8_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshllt_n_u16(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllt, IMM2 = 0))]
pub unsafe fn svshllt_n_u32<const IMM2: i32>(op1: svuint16_t) -> svuint32_t {
    //static_assert_range!(IMM2, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllt_n_u32(op1: svint16_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshllt_n_u32(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ushllt, IMM2 = 0))]
pub unsafe fn svshllt_n_u64<const IMM2: i32>(op1: svuint32_t) -> svuint64_t {
    //static_assert_range!(IMM2, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshllt_n_u64(op1: svint32_t, imm2: i32) -> svint64_t;
    }
    unsafe { _svshllt_n_u64(op1.as_signed(), IMM2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub unsafe fn svshrnb_n_s16<const IMM2: i32>(op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshrnb_n_s16(op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svshrnb_n_s16(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub unsafe fn svshrnb_n_s32<const IMM2: i32>(op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshrnb_n_s32(op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshrnb_n_s32(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub unsafe fn svshrnb_n_s64<const IMM2: i32>(op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshrnb_n_s64(op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshrnb_n_s64(op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub unsafe fn svshrnb_n_u16<const IMM2: i32>(op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    unsafe { svshrnb_n_s16::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub unsafe fn svshrnb_n_u32<const IMM2: i32>(op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    unsafe { svshrnb_n_s32::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnb, IMM2 = 1))]
pub unsafe fn svshrnb_n_u64<const IMM2: i32>(op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    unsafe { svshrnb_n_s64::<IMM2>(op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub unsafe fn svshrnt_n_s16<const IMM2: i32>(even: svint8_t, op1: svint16_t) -> svint8_t {
    //static_assert_range!(IMM2, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshrnt_n_s16(even: svint8_t, op1: svint16_t, imm2: i32) -> svint8_t;
    }
    unsafe { _svshrnt_n_s16(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub unsafe fn svshrnt_n_s32<const IMM2: i32>(even: svint16_t, op1: svint32_t) -> svint16_t {
    //static_assert_range!(IMM2, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshrnt_n_s32(even: svint16_t, op1: svint32_t, imm2: i32) -> svint16_t;
    }
    unsafe { _svshrnt_n_s32(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub unsafe fn svshrnt_n_s64<const IMM2: i32>(even: svint32_t, op1: svint64_t) -> svint32_t {
    //static_assert_range!(IMM2, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svshrnt_n_s64(even: svint32_t, op1: svint64_t, imm2: i32) -> svint32_t;
    }
    unsafe { _svshrnt_n_s64(even, op1, IMM2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub unsafe fn svshrnt_n_u16<const IMM2: i32>(even: svuint8_t, op1: svuint16_t) -> svuint8_t {
    //static_assert_range!(IMM2, 1, 8);
    unsafe { svshrnt_n_s16::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub unsafe fn svshrnt_n_u32<const IMM2: i32>(even: svuint16_t, op1: svuint32_t) -> svuint16_t {
    //static_assert_range!(IMM2, 1, 16);
    unsafe { svshrnt_n_s32::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(shrnt, IMM2 = 1))]
pub unsafe fn svshrnt_n_u64<const IMM2: i32>(even: svuint32_t, op1: svuint64_t) -> svuint32_t {
    //static_assert_range!(IMM2, 1, 32);
    unsafe { svshrnt_n_s64::<IMM2>(even.as_signed(), op1.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert_range!(IMM3, 0, 7);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsli_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsli_n_s8(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM3, 0, 15);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsli_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsli_n_s16(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM3, 0, 31);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsli_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsli_n_s32(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM3, 0, 63);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsli_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsli_n_s64(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    //static_assert_range!(IMM3, 0, 7);
    unsafe { svsli_n_s8::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM3, 0, 15);
    unsafe { svsli_n_s16::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM3, 0, 31);
    unsafe { svsli_n_s32::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sli, IMM3 = 0))]
pub unsafe fn svsli_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM3, 0, 63);
    unsafe { svsli_n_s64::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sm4")]
//#[cfg_attr(test, assert_instr(sm4e))]
pub unsafe fn svsm4e_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsm4e_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svsm4e_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2,sve2-sm4")]
//#[cfg_attr(test, assert_instr(sm4ekey))]
pub unsafe fn svsm4ekey_u32(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsm4ekey_u32(op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svsm4ekey_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u8_m(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsqadd_u8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svsqadd_u8_m(pg, op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u8_m(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svsqadd_u8_m(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u8_x(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svsqadd_u8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u8_x(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svsqadd_u8_x(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u8_z(pg: svbool_t, op1: svuint8_t, op2: svint8_t) -> svuint8_t {
    svsqadd_u8_m(pg, svsel_u8(pg, op1, svdup_n_u8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u8_z(pg: svbool_t, op1: svuint8_t, op2: i8) -> svuint8_t {
    svsqadd_u8_z(pg, op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u16_m(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsqadd_u16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svsqadd_u16_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u16_m(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svsqadd_u16_m(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u16_x(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svsqadd_u16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u16_x(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svsqadd_u16_x(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u16_z(pg: svbool_t, op1: svuint16_t, op2: svint16_t) -> svuint16_t {
    svsqadd_u16_m(pg, svsel_u16(pg, op1, svdup_n_u16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u16_z(pg: svbool_t, op1: svuint16_t, op2: i16) -> svuint16_t {
    svsqadd_u16_z(pg, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u32_m(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsqadd_u32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svsqadd_u32_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u32_m(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svsqadd_u32_m(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u32_x(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svsqadd_u32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u32_x(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svsqadd_u32_x(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u32_z(pg: svbool_t, op1: svuint32_t, op2: svint32_t) -> svuint32_t {
    svsqadd_u32_m(pg, svsel_u32(pg, op1, svdup_n_u32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u32_z(pg: svbool_t, op1: svuint32_t, op2: i32) -> svuint32_t {
    svsqadd_u32_z(pg, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u64_m(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsqadd_u64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svsqadd_u64_m(pg.into(), op1.as_signed(), op2).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u64_m(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svsqadd_u64_m(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u64_x(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svsqadd_u64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u64_x(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svsqadd_u64_x(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_u64_z(pg: svbool_t, op1: svuint64_t, op2: svint64_t) -> svuint64_t {
    svsqadd_u64_m(pg, svsel_u64(pg, op1, svdup_n_u64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usqadd))]
pub unsafe fn svsqadd_n_u64_z(pg: svbool_t, op1: svuint64_t, op2: i64) -> svuint64_t {
    svsqadd_u64_z(pg, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub unsafe fn svsra_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsra_n_s8(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub unsafe fn svsra_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsra_n_s16(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub unsafe fn svsra_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsra_n_s32(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssra, IMM3 = 1))]
pub unsafe fn svsra_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsra_n_s64(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub unsafe fn svsra_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    //static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_u8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsra_n_u8(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub unsafe fn svsra_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_u16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsra_n_u16(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub unsafe fn svsra_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_u32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsra_n_u32(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usra, IMM3 = 1))]
pub unsafe fn svsra_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsra_n_u64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsra_n_u64(op1.as_signed(), op2.as_signed(), IMM3).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsri_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svsri_n_s8(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsri_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svsri_n_s16(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsri_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svsri_n_s32(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsri_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svsri_n_s64(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    //static_assert_range!(IMM3, 1, 8);
    unsafe { svsri_n_s8::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM3, 1, 16);
    unsafe { svsri_n_s16::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM3, 1, 32);
    unsafe { svsri_n_s32::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sri, IMM3 = 1))]
pub unsafe fn svsri_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM3, 1, 64);
    unsafe { svsri_n_s64::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64index_f64(
    pg: svbool_t,
    base: *mut f64,
    indices: svint64_t,
    data: svfloat64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_s64index_f64(
            data: svfloat64_t,
            pg: svbool2_t,
            base: *mut f64,
            indices: svint64_t,
        );
    }
    _svstnt1_scatter_s64index_f64(data, pg.into(), base, indices)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64index_s64(
    pg: svbool_t,
    base: *mut i64,
    indices: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_s64index_s64(
            data: svint64_t,
            pg: svbool2_t,
            base: *mut i64,
            indices: svint64_t,
        );
    }
    _svstnt1_scatter_s64index_s64(data, pg.into(), base, indices)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64index_u64(
    pg: svbool_t,
    base: *mut u64,
    indices: svint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64index_s64(pg, base as *mut i64, indices, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64index_f64(
    pg: svbool_t,
    base: *mut f64,
    indices: svuint64_t,
    data: svfloat64_t,
) {
    svstnt1_scatter_s64index_f64(pg, base, indices.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64index_s64(
    pg: svbool_t,
    base: *mut i64,
    indices: svuint64_t,
    data: svint64_t,
) {
    svstnt1_scatter_s64index_s64(pg, base, indices.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64index_u64(
    pg: svbool_t,
    base: *mut u64,
    indices: svuint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64index_s64(pg, base as *mut i64, indices.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64offset_f64(
    pg: svbool_t,
    base: *mut f64,
    offsets: svint64_t,
    data: svfloat64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_s64offset_f64(
            data: svfloat64_t,
            pg: svbool2_t,
            base: *mut f64,
            offsets: svint64_t,
        );
    }
    _svstnt1_scatter_s64offset_f64(data, pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i64,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_s64offset_s64(
            data: svint64_t,
            pg: svbool2_t,
            base: *mut i64,
            offsets: svint64_t,
        );
    }
    _svstnt1_scatter_s64offset_s64(data, pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u64,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64offset_s64(pg, base as *mut i64, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32offset_f32(
    pg: svbool_t,
    base: *mut f32,
    offsets: svuint32_t,
    data: svfloat32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_u32offset_f32(
            data: svfloat32_t,
            pg: svbool4_t,
            base: *mut f32,
            offsets: svint32_t,
        );
    }
    _svstnt1_scatter_u32offset_f32(data, pg.into(), base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32offset_s32(
    pg: svbool_t,
    base: *mut i32,
    offsets: svuint32_t,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_u32offset_s32(
            data: svint32_t,
            pg: svbool4_t,
            base: *mut i32,
            offsets: svint32_t,
        );
    }
    _svstnt1_scatter_u32offset_s32(data, pg.into(), base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32offset_u32(
    pg: svbool_t,
    base: *mut u32,
    offsets: svuint32_t,
    data: svuint32_t,
) {
    svstnt1_scatter_u32offset_s32(pg, base as *mut i32, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64offset_f64(
    pg: svbool_t,
    base: *mut f64,
    offsets: svuint64_t,
    data: svfloat64_t,
) {
    svstnt1_scatter_s64offset_f64(pg, base, offsets.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i64,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u64,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1_scatter_s64offset_s64(pg, base as *mut i64, offsets.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_f32(pg: svbool_t, bases: svuint32_t, data: svfloat32_t) {
    svstnt1_scatter_u32base_offset_f32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_s32(pg: svbool_t, bases: svuint32_t, data: svint32_t) {
    svstnt1_scatter_u32base_offset_s32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_u32(pg: svbool_t, bases: svuint32_t, data: svuint32_t) {
    svstnt1_scatter_u32base_offset_u32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_f64(pg: svbool_t, bases: svuint64_t, data: svfloat64_t) {
    svstnt1_scatter_u64base_offset_f64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_index_f32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svfloat32_t,
) {
    svstnt1_scatter_u32base_offset_f32(pg, bases, index.unchecked_shl(2), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svint32_t,
) {
    svstnt1_scatter_u32base_offset_s32(pg, bases, index.unchecked_shl(2), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svuint32_t,
) {
    svstnt1_scatter_u32base_offset_u32(pg, bases, index.unchecked_shl(2), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_index_f64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svfloat64_t,
) {
    svstnt1_scatter_u64base_offset_f64(pg, bases, index.unchecked_shl(3), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svint64_t,
) {
    svstnt1_scatter_u64base_offset_s64(pg, bases, index.unchecked_shl(3), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svuint64_t,
) {
    svstnt1_scatter_u64base_offset_u64(pg, bases, index.unchecked_shl(3), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_offset_f32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svfloat32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_u32base_offset_f32(
            data: svfloat32_t,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u32base_offset_f32(data, pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_u32base_offset_s32(
            data: svint32_t,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u32base_offset_s32(data, pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1_scatter_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svuint32_t,
) {
    svstnt1_scatter_u32base_offset_s32(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_offset_f64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svfloat64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_u64base_offset_f64(
            data: svfloat64_t,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u64base_offset_f64(data, pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1_scatter_u64base_offset_s64(
            data: svint64_t,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1_scatter_u64base_offset_s64(data, pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1d))]
pub unsafe fn svstnt1_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i8,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1b_scatter_s64offset_s64(
            data: nxv2i8,
            pg: svbool2_t,
            base: *mut i8,
            offsets: svint64_t,
        );
    }
    _svstnt1b_scatter_s64offset_s64(simd_cast(data), pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i16,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1h_scatter_s64offset_s64(
            data: nxv2i16,
            pg: svbool2_t,
            base: *mut i16,
            offsets: svint64_t,
        );
    }
    _svstnt1h_scatter_s64offset_s64(simd_cast(data), pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64offset_s64(
    pg: svbool_t,
    base: *mut i32,
    offsets: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1w_scatter_s64offset_s64(
            data: nxv2i32,
            pg: svbool2_t,
            base: *mut i32,
            offsets: svint64_t,
        );
    }
    _svstnt1w_scatter_s64offset_s64(simd_cast(data), pg.into(), base, offsets)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u8,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1b_scatter_s64offset_s64(pg, base as *mut i8, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u16,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64offset_s64(pg, base as *mut i16, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64offset_u64(
    pg: svbool_t,
    base: *mut u32,
    offsets: svint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64offset_s64(pg, base as *mut i32, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32offset_s32(
    pg: svbool_t,
    base: *mut i8,
    offsets: svuint32_t,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1b_scatter_u32offset_s32(
            data: nxv4i8,
            pg: svbool4_t,
            base: *mut i8,
            offsets: svint32_t,
        );
    }
    _svstnt1b_scatter_u32offset_s32(simd_cast(data), pg.into(), base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32offset_s32(
    pg: svbool_t,
    base: *mut i16,
    offsets: svuint32_t,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1h_scatter_u32offset_s32(
            data: nxv4i16,
            pg: svbool4_t,
            base: *mut i16,
            offsets: svint32_t,
        );
    }
    _svstnt1h_scatter_u32offset_s32(simd_cast(data), pg.into(), base, offsets.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32offset_u32(
    pg: svbool_t,
    base: *mut u8,
    offsets: svuint32_t,
    data: svuint32_t,
) {
    svstnt1b_scatter_u32offset_s32(pg, base as *mut i8, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32offset_u32(
    pg: svbool_t,
    base: *mut u16,
    offsets: svuint32_t,
    data: svuint32_t,
) {
    svstnt1h_scatter_u32offset_s32(pg, base as *mut i16, offsets, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i8,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1b_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i16,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1h_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64offset_s64(
    pg: svbool_t,
    base: *mut i32,
    offsets: svuint64_t,
    data: svint64_t,
) {
    svstnt1w_scatter_s64offset_s64(pg, base, offsets.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u8,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1b_scatter_s64offset_s64(pg, base as *mut i8, offsets.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u16,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64offset_s64(pg, base as *mut i16, offsets.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64offset_u64(
    pg: svbool_t,
    base: *mut u32,
    offsets: svuint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64offset_s64(pg, base as *mut i32, offsets.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1b_scatter_u32base_offset_s32(
            data: nxv4i8,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1b_scatter_u32base_offset_s32(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_offset_s32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svint32_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1h_scatter_u32base_offset_s32(
            data: nxv4i16,
            pg: svbool4_t,
            bases: svint32_t,
            offset: i64,
        );
    }
    _svstnt1h_scatter_u32base_offset_s32(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svuint32_t,
) {
    svstnt1b_scatter_u32base_offset_s32(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_offset_u32(
    pg: svbool_t,
    bases: svuint32_t,
    offset: i64,
    data: svuint32_t,
) {
    svstnt1h_scatter_u32base_offset_s32(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1b_scatter_u64base_offset_s64(
            data: nxv2i8,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1b_scatter_u64base_offset_s64(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1h_scatter_u64base_offset_s64(
            data: nxv2i16,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1h_scatter_u64base_offset_s64(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_offset_s64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1w_scatter_u64base_offset_s64(
            data: nxv2i32,
            pg: svbool2_t,
            bases: svint64_t,
            offset: i64,
        );
    }
    _svstnt1w_scatter_u64base_offset_s64(simd_cast(data), pg.into(), bases.as_signed(), offset)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1b_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1h_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_offset_u64(
    pg: svbool_t,
    bases: svuint64_t,
    offset: i64,
    data: svuint64_t,
) {
    svstnt1w_scatter_u64base_offset_s64(pg, bases, offset, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_s32(pg: svbool_t, bases: svuint32_t, data: svint32_t) {
    svstnt1b_scatter_u32base_offset_s32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_s32(pg: svbool_t, bases: svuint32_t, data: svint32_t) {
    svstnt1h_scatter_u32base_offset_s32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u32base_u32(pg: svbool_t, bases: svuint32_t, data: svuint32_t) {
    svstnt1b_scatter_u32base_offset_u32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_u32(pg: svbool_t, bases: svuint32_t, data: svuint32_t) {
    svstnt1h_scatter_u32base_offset_u32(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1b_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1h_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_s64(pg: svbool_t, bases: svuint64_t, data: svint64_t) {
    svstnt1w_scatter_u64base_offset_s64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1b))]
pub unsafe fn svstnt1b_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1b_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1h_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_u64(pg: svbool_t, bases: svuint64_t, data: svuint64_t) {
    svstnt1w_scatter_u64base_offset_u64(pg, bases, 0, data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64index_s64(
    pg: svbool_t,
    base: *mut i16,
    indices: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1h_scatter_s64index_s64(
            data: nxv2i16,
            pg: svbool2_t,
            base: *mut i16,
            indices: svint64_t,
        );
    }
    _svstnt1h_scatter_s64index_s64(simd_cast(data), pg.into(), base, indices)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64index_s64(
    pg: svbool_t,
    base: *mut i32,
    indices: svint64_t,
    data: svint64_t,
) {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svstnt1w_scatter_s64index_s64(
            data: nxv2i32,
            pg: svbool2_t,
            base: *mut i32,
            indices: svint64_t,
        );
    }
    _svstnt1w_scatter_s64index_s64(simd_cast(data), pg.into(), base, indices)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_s64index_u64(
    pg: svbool_t,
    base: *mut u16,
    indices: svint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64index_s64(pg, base as *mut i16, indices, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_s64index_u64(
    pg: svbool_t,
    base: *mut u32,
    indices: svint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64index_s64(pg, base as *mut i32, indices, data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64index_s64(
    pg: svbool_t,
    base: *mut i16,
    indices: svuint64_t,
    data: svint64_t,
) {
    svstnt1h_scatter_s64index_s64(pg, base, indices.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64index_s64(
    pg: svbool_t,
    base: *mut i32,
    indices: svuint64_t,
    data: svint64_t,
) {
    svstnt1w_scatter_s64index_s64(pg, base, indices.as_signed(), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64index_u64(
    pg: svbool_t,
    base: *mut u16,
    indices: svuint64_t,
    data: svuint64_t,
) {
    svstnt1h_scatter_s64index_s64(pg, base as *mut i16, indices.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64index_u64(
    pg: svbool_t,
    base: *mut u32,
    indices: svuint64_t,
    data: svuint64_t,
) {
    svstnt1w_scatter_s64index_s64(pg, base as *mut i32, indices.as_signed(), data.as_signed())
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_index_s32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svint32_t,
) {
    svstnt1h_scatter_u32base_offset_s32(pg, bases, index.unchecked_shl(1), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u32base_index_u32(
    pg: svbool_t,
    bases: svuint32_t,
    index: i64,
    data: svuint32_t,
) {
    svstnt1h_scatter_u32base_offset_u32(pg, bases, index.unchecked_shl(1), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svint64_t,
) {
    svstnt1h_scatter_u64base_offset_s64(pg, bases, index.unchecked_shl(1), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_index_s64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svint64_t,
) {
    svstnt1w_scatter_u64base_offset_s64(pg, bases, index.unchecked_shl(2), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1h))]
pub unsafe fn svstnt1h_scatter_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svuint64_t,
) {
    svstnt1h_scatter_u64base_offset_u64(pg, bases, index.unchecked_shl(1), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(stnt1w))]
pub unsafe fn svstnt1w_scatter_u64base_index_u64(
    pg: svbool_t,
    bases: svuint64_t,
    index: i64,
    data: svuint64_t,
) {
    svstnt1w_scatter_u64base_offset_u64(pg, bases, index.unchecked_shl(2), data)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubhnb_s16(op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svsubhnb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_n_s16(op1: svint16_t, op2: i16) -> svint8_t {
    svsubhnb_s16(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubhnb_s32(op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svsubhnb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_n_s32(op1: svint32_t, op2: i32) -> svint16_t {
    svsubhnb_s32(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubhnb_s64(op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svsubhnb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_n_s64(op1: svint64_t, op2: i64) -> svint32_t {
    svsubhnb_s64(op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_u16(op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svsubhnb_s16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_n_u16(op1: svuint16_t, op2: u16) -> svuint8_t {
    svsubhnb_u16(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_u32(op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svsubhnb_s32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_n_u32(op1: svuint32_t, op2: u32) -> svuint16_t {
    svsubhnb_u32(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_u64(op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svsubhnb_s64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnb))]
pub unsafe fn svsubhnb_n_u64(op1: svuint64_t, op2: u64) -> svuint32_t {
    svsubhnb_u64(op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubhnt_s16(even: svint8_t, op1: svint16_t, op2: svint16_t) -> svint8_t;
    }
    unsafe { _svsubhnt_s16(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_n_s16(even: svint8_t, op1: svint16_t, op2: i16) -> svint8_t {
    svsubhnt_s16(even, op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubhnt_s32(even: svint16_t, op1: svint32_t, op2: svint32_t) -> svint16_t;
    }
    unsafe { _svsubhnt_s32(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_n_s32(even: svint16_t, op1: svint32_t, op2: i32) -> svint16_t {
    svsubhnt_s32(even, op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubhnt_s64(even: svint32_t, op1: svint64_t, op2: svint64_t) -> svint32_t;
    }
    unsafe { _svsubhnt_s64(even, op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_n_s64(even: svint32_t, op1: svint64_t, op2: i64) -> svint32_t {
    svsubhnt_s64(even, op1, svdup_n_s64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_u16(even: svuint8_t, op1: svuint16_t, op2: svuint16_t) -> svuint8_t {
    unsafe { svsubhnt_s16(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_n_u16(even: svuint8_t, op1: svuint16_t, op2: u16) -> svuint8_t {
    svsubhnt_u16(even, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_u32(even: svuint16_t, op1: svuint32_t, op2: svuint32_t) -> svuint16_t {
    unsafe { svsubhnt_s32(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_n_u32(even: svuint16_t, op1: svuint32_t, op2: u32) -> svuint16_t {
    svsubhnt_u32(even, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_u64(even: svuint32_t, op1: svuint64_t, op2: svuint64_t) -> svuint32_t {
    unsafe { svsubhnt_s64(even.as_signed(), op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(subhnt))]
pub unsafe fn svsubhnt_n_u64(even: svuint32_t, op1: svuint64_t, op2: u64) -> svuint32_t {
    svsubhnt_u64(even, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublb))]
pub unsafe fn svsublb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublb))]
pub unsafe fn svsublb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsublb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublb))]
pub unsafe fn svsublb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublb))]
pub unsafe fn svsublb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsublb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublb))]
pub unsafe fn svsublb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublb))]
pub unsafe fn svsublb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsublb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublb))]
pub unsafe fn svsublb_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublb_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublb))]
pub unsafe fn svsublb_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svsublb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublb))]
pub unsafe fn svsublb_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublb_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublb))]
pub unsafe fn svsublb_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svsublb_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublb))]
pub unsafe fn svsublb_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublb_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublb))]
pub unsafe fn svsublb_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svsublb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublbt))]
pub unsafe fn svsublbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svsublbt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublbt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublbt))]
pub unsafe fn svsublbt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsublbt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublbt))]
pub unsafe fn svsublbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svsublbt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublbt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublbt))]
pub unsafe fn svsublbt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsublbt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublbt))]
pub unsafe fn svsublbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svsublbt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublbt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublbt))]
pub unsafe fn svsublbt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsublbt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublt))]
pub unsafe fn svsublt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublt_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublt))]
pub unsafe fn svsublt_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsublt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublt))]
pub unsafe fn svsublt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublt_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublt))]
pub unsafe fn svsublt_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsublt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublt))]
pub unsafe fn svsublt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublt_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssublt))]
pub unsafe fn svsublt_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsublt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublt))]
pub unsafe fn svsublt_u16(op1: svuint8_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublt_u16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsublt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublt))]
pub unsafe fn svsublt_n_u16(op1: svuint8_t, op2: u8) -> svuint16_t {
    svsublt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublt))]
pub unsafe fn svsublt_u32(op1: svuint16_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublt_u32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsublt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublt))]
pub unsafe fn svsublt_n_u32(op1: svuint16_t, op2: u16) -> svuint32_t {
    svsublt_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublt))]
pub unsafe fn svsublt_u64(op1: svuint32_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsublt_u64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsublt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usublt))]
pub unsafe fn svsublt_n_u64(op1: svuint32_t, op2: u32) -> svuint64_t {
    svsublt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubltb))]
pub unsafe fn svsubltb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svsubltb_s16(op1: svint8_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubltb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubltb))]
pub unsafe fn svsubltb_n_s16(op1: svint8_t, op2: i8) -> svint16_t {
    svsubltb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubltb))]
pub unsafe fn svsubltb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svsubltb_s32(op1: svint16_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubltb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubltb))]
pub unsafe fn svsubltb_n_s32(op1: svint16_t, op2: i16) -> svint32_t {
    svsubltb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubltb))]
pub unsafe fn svsubltb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svsubltb_s64(op1: svint32_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubltb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubltb))]
pub unsafe fn svsubltb_n_s64(op1: svint32_t, op2: i32) -> svint64_t {
    svsubltb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwb))]
pub unsafe fn svsubwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwb_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwb_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwb))]
pub unsafe fn svsubwb_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svsubwb_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwb))]
pub unsafe fn svsubwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwb_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwb_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwb))]
pub unsafe fn svsubwb_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svsubwb_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwb))]
pub unsafe fn svsubwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwb_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwb_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwb))]
pub unsafe fn svsubwb_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svsubwb_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwb))]
pub unsafe fn svsubwb_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwb_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwb_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwb))]
pub unsafe fn svsubwb_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svsubwb_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwb))]
pub unsafe fn svsubwb_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwb_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwb_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwb))]
pub unsafe fn svsubwb_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svsubwb_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwb))]
pub unsafe fn svsubwb_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwb_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwb_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwb))]
pub unsafe fn svsubwb_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svsubwb_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwt))]
pub unsafe fn svsubwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwt_s16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwt_s16(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwt))]
pub unsafe fn svsubwt_n_s16(op1: svint16_t, op2: i8) -> svint16_t {
    svsubwt_s16(op1, svdup_n_s8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwt))]
pub unsafe fn svsubwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwt_s32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwt_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwt))]
pub unsafe fn svsubwt_n_s32(op1: svint32_t, op2: i16) -> svint32_t {
    svsubwt_s32(op1, svdup_n_s16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwt))]
pub unsafe fn svsubwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwt_s64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwt_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(ssubwt))]
pub unsafe fn svsubwt_n_s64(op1: svint64_t, op2: i32) -> svint64_t {
    svsubwt_s64(op1, svdup_n_s32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwt))]
pub unsafe fn svsubwt_u16(op1: svuint16_t, op2: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwt_u16(op1: svint16_t, op2: svint8_t) -> svint16_t;
    }
    unsafe { _svsubwt_u16(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwt))]
pub unsafe fn svsubwt_n_u16(op1: svuint16_t, op2: u8) -> svuint16_t {
    svsubwt_u16(op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwt))]
pub unsafe fn svsubwt_u32(op1: svuint32_t, op2: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwt_u32(op1: svint32_t, op2: svint16_t) -> svint32_t;
    }
    unsafe { _svsubwt_u32(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwt))]
pub unsafe fn svsubwt_n_u32(op1: svuint32_t, op2: u16) -> svuint32_t {
    svsubwt_u32(op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwt))]
pub unsafe fn svsubwt_u64(op1: svuint64_t, op2: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svsubwt_u64(op1: svint64_t, op2: svint32_t) -> svint64_t;
    }
    unsafe { _svsubwt_u64(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(usubwt))]
pub unsafe fn svsubwt_n_u64(op1: svuint64_t, op2: u32) -> svuint64_t {
    svsubwt_u64(op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_f32(data: svfloat32x2_t, indices: svuint32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbl2_f32(data0: svfloat32_t, data1: svfloat32_t, indices: svint32_t) -> svfloat32_t;
    }
    unsafe {
        _svtbl2_f32(
            svget2_f32::<0>(data),
            svget2_f32::<1>(data),
            indices.as_signed(),
        )
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_f64(data: svfloat64x2_t, indices: svuint64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbl2_f64(data0: svfloat64_t, data1: svfloat64_t, indices: svint64_t) -> svfloat64_t;
    }
    unsafe {
        _svtbl2_f64(
            svget2_f64::<0>(data),
            svget2_f64::<1>(data),
            indices.as_signed(),
        )
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_s8(data: svint8x2_t, indices: svuint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbl2_s8(data0: svint8_t, data1: svint8_t, indices: svint8_t) -> svint8_t;
    }
    unsafe {
        _svtbl2_s8(
            svget2_s8::<0>(data),
            svget2_s8::<1>(data),
            indices.as_signed(),
        )
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_s16(data: svint16x2_t, indices: svuint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbl2_s16(data0: svint16_t, data1: svint16_t, indices: svint16_t) -> svint16_t;
    }
    unsafe {
        _svtbl2_s16(
            svget2_s16::<0>(data),
            svget2_s16::<1>(data),
            indices.as_signed(),
        )
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_s32(data: svint32x2_t, indices: svuint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbl2_s32(data0: svint32_t, data1: svint32_t, indices: svint32_t) -> svint32_t;
    }
    unsafe {
        _svtbl2_s32(
            svget2_s32::<0>(data),
            svget2_s32::<1>(data),
            indices.as_signed(),
        )
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_s64(data: svint64x2_t, indices: svuint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbl2_s64(data0: svint64_t, data1: svint64_t, indices: svint64_t) -> svint64_t;
    }
    unsafe {
        _svtbl2_s64(
            svget2_s64::<0>(data),
            svget2_s64::<1>(data),
            indices.as_signed(),
        )
    }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_u8(data: svuint8x2_t, indices: svuint8_t) -> svuint8_t {
    unsafe { svtbl2_s8(data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_u16(data: svuint16x2_t, indices: svuint16_t) -> svuint16_t {
    unsafe { svtbl2_s16(data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_u32(data: svuint32x2_t, indices: svuint32_t) -> svuint32_t {
    unsafe { svtbl2_s32(data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbl))]
pub unsafe fn svtbl2_u64(data: svuint64x2_t, indices: svuint64_t) -> svuint64_t {
    unsafe { svtbl2_s64(data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_f32(fallback: svfloat32_t, data: svfloat32_t, indices: svuint32_t) -> svfloat32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbx_f32(fallback: svfloat32_t, data: svfloat32_t, indices: svint32_t) -> svfloat32_t;
    }
    unsafe { _svtbx_f32(fallback, data, indices.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_f64(fallback: svfloat64_t, data: svfloat64_t, indices: svuint64_t) -> svfloat64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbx_f64(fallback: svfloat64_t, data: svfloat64_t, indices: svint64_t) -> svfloat64_t;
    }
    unsafe { _svtbx_f64(fallback, data, indices.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_s8(fallback: svint8_t, data: svint8_t, indices: svuint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbx_s8(fallback: svint8_t, data: svint8_t, indices: svint8_t) -> svint8_t;
    }
    unsafe { _svtbx_s8(fallback, data, indices.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_s16(fallback: svint16_t, data: svint16_t, indices: svuint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbx_s16(fallback: svint16_t, data: svint16_t, indices: svint16_t) -> svint16_t;
    }
    unsafe { _svtbx_s16(fallback, data, indices.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_s32(fallback: svint32_t, data: svint32_t, indices: svuint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbx_s32(fallback: svint32_t, data: svint32_t, indices: svint32_t) -> svint32_t;
    }
    unsafe { _svtbx_s32(fallback, data, indices.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_s64(fallback: svint64_t, data: svint64_t, indices: svuint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svtbx_s64(fallback: svint64_t, data: svint64_t, indices: svint64_t) -> svint64_t;
    }
    unsafe { _svtbx_s64(fallback, data, indices.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_u8(fallback: svuint8_t, data: svuint8_t, indices: svuint8_t) -> svuint8_t {
    unsafe { svtbx_s8(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_u16(fallback: svuint16_t, data: svuint16_t, indices: svuint16_t) -> svuint16_t {
    unsafe { svtbx_s16(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_u32(fallback: svuint32_t, data: svuint32_t, indices: svuint32_t) -> svuint32_t {
    unsafe { svtbx_s32(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(tbx))]
pub unsafe fn svtbx_u64(fallback: svuint64_t, data: svuint64_t, indices: svuint64_t) -> svuint64_t {
    unsafe { svtbx_s64(fallback.as_signed(), data.as_signed(), indices).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(punpkhi))]
pub unsafe fn svunpkhi_b(op: svbool_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_b(op: svbool_t) -> svbool8_t;
    }
    unsafe { _svunpkhi_b(op).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sunpkhi))]
pub unsafe fn svunpkhi_s16(op: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_s16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpkhi_s16(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sunpkhi))]
pub unsafe fn svunpkhi_s32(op: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_s32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpkhi_s32(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sunpkhi))]
pub unsafe fn svunpkhi_s64(op: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_s64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpkhi_s64(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uunpkhi))]
pub unsafe fn svunpkhi_u16(op: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_u16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpkhi_u16(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uunpkhi))]
pub unsafe fn svunpkhi_u32(op: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_u32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpkhi_u32(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uunpkhi))]
pub unsafe fn svunpkhi_u64(op: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpkhi_u64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpkhi_u64(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(punpklo))]
pub unsafe fn svunpklo_b(op: svbool_t) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_b(op: svbool_t) -> svbool8_t;
    }
    unsafe { _svunpklo_b(op).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sunpklo))]
pub unsafe fn svunpklo_s16(op: svint8_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_s16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpklo_s16(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sunpklo))]
pub unsafe fn svunpklo_s32(op: svint16_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_s32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpklo_s32(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(sunpklo))]
pub unsafe fn svunpklo_s64(op: svint32_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_s64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpklo_s64(op) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uunpklo))]
pub unsafe fn svunpklo_u16(op: svuint8_t) -> svuint16_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_u16(op: svint8_t) -> svint16_t;
    }
    unsafe { _svunpklo_u16(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uunpklo))]
pub unsafe fn svunpklo_u32(op: svuint16_t) -> svuint32_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_u32(op: svint16_t) -> svint32_t;
    }
    unsafe { _svunpklo_u32(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(uunpklo))]
pub unsafe fn svunpklo_u64(op: svuint32_t) -> svuint64_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svunpklo_u64(op: svint32_t) -> svint64_t;
    }
    unsafe { _svunpklo_u64(op.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svuint8_t) -> svint8_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svuqadd_s8_m(pg: svbool_t, op1: svint8_t, op2: svint8_t) -> svint8_t;
    }
    unsafe { _svuqadd_s8_m(pg, op1, op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s8_m(pg: svbool_t, op1: svint8_t, op2: u8) -> svint8_t {
    svuqadd_s8_m(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s8_x(pg: svbool_t, op1: svint8_t, op2: svuint8_t) -> svint8_t {
    svuqadd_s8_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s8_x(pg: svbool_t, op1: svint8_t, op2: u8) -> svint8_t {
    svuqadd_s8_x(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s8_z(pg: svbool_t, op1: svint8_t, op2: svuint8_t) -> svint8_t {
    svuqadd_s8_m(pg, svsel_s8(pg, op1, svdup_n_s8(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s8_z(pg: svbool_t, op1: svint8_t, op2: u8) -> svint8_t {
    svuqadd_s8_z(pg, op1, svdup_n_u8(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s16_m(pg: svbool_t, op1: svint16_t, op2: svuint16_t) -> svint16_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svuqadd_s16_m(pg: svbool8_t, op1: svint16_t, op2: svint16_t) -> svint16_t;
    }
    unsafe { _svuqadd_s16_m(pg.into(), op1, op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s16_m(pg: svbool_t, op1: svint16_t, op2: u16) -> svint16_t {
    svuqadd_s16_m(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s16_x(pg: svbool_t, op1: svint16_t, op2: svuint16_t) -> svint16_t {
    svuqadd_s16_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s16_x(pg: svbool_t, op1: svint16_t, op2: u16) -> svint16_t {
    svuqadd_s16_x(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s16_z(pg: svbool_t, op1: svint16_t, op2: svuint16_t) -> svint16_t {
    svuqadd_s16_m(pg, svsel_s16(pg, op1, svdup_n_s16(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s16_z(pg: svbool_t, op1: svint16_t, op2: u16) -> svint16_t {
    svuqadd_s16_z(pg, op1, svdup_n_u16(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s32_m(pg: svbool_t, op1: svint32_t, op2: svuint32_t) -> svint32_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svuqadd_s32_m(pg: svbool4_t, op1: svint32_t, op2: svint32_t) -> svint32_t;
    }
    unsafe { _svuqadd_s32_m(pg.into(), op1, op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s32_m(pg: svbool_t, op1: svint32_t, op2: u32) -> svint32_t {
    svuqadd_s32_m(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s32_x(pg: svbool_t, op1: svint32_t, op2: svuint32_t) -> svint32_t {
    svuqadd_s32_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s32_x(pg: svbool_t, op1: svint32_t, op2: u32) -> svint32_t {
    svuqadd_s32_x(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s32_z(pg: svbool_t, op1: svint32_t, op2: svuint32_t) -> svint32_t {
    svuqadd_s32_m(pg, svsel_s32(pg, op1, svdup_n_s32(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s32_z(pg: svbool_t, op1: svint32_t, op2: u32) -> svint32_t {
    svuqadd_s32_z(pg, op1, svdup_n_u32(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s64_m(pg: svbool_t, op1: svint64_t, op2: svuint64_t) -> svint64_t {
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svuqadd_s64_m(pg: svbool2_t, op1: svint64_t, op2: svint64_t) -> svint64_t;
    }
    unsafe { _svuqadd_s64_m(pg.into(), op1, op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s64_m(pg: svbool_t, op1: svint64_t, op2: u64) -> svint64_t {
    svuqadd_s64_m(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s64_x(pg: svbool_t, op1: svint64_t, op2: svuint64_t) -> svint64_t {
    svuqadd_s64_m(pg, op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s64_x(pg: svbool_t, op1: svint64_t, op2: u64) -> svint64_t {
    svuqadd_s64_x(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_s64_z(pg: svbool_t, op1: svint64_t, op2: svuint64_t) -> svint64_t {
    svuqadd_s64_m(pg, svsel_s64(pg, op1, svdup_n_s64(0)), op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(suqadd))]
pub unsafe fn svuqadd_n_s64_z(pg: svbool_t, op1: svint64_t, op2: u64) -> svint64_t {
    svuqadd_s64_z(pg, op1, svdup_n_u64(op2))
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b8_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b8_s32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilege_b8_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b16_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b16_s32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_s32(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b32_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b32_s32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_s32(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b64_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b64_s32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_s32(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b8_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b8_s64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilege_b8_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b16_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b16_s64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_s64(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b32_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b32_s64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_s64(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilege))]
pub unsafe fn svwhilege_b64_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b64_s64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_s64(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b8_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b8_u32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilege_b8_u32(op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b16_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b16_u32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b32_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b32_u32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b64_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b64_u32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b8_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b8_u64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilege_b8_u64(op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b16_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b16_u64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilege_b16_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b32_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b32_u64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilege_b32_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehs))]
pub unsafe fn svwhilege_b64_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilege_b64_u64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilege_b64_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b8_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b8_s32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_s32(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b16_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b16_s32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_s32(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b32_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b32_s32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_s32(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b64_s32(op1: i32, op2: i32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b64_s32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_s32(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b8_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b8_s64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_s64(op1, op2) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b16_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b16_s64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_s64(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b32_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b32_s64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_s64(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilegt))]
pub unsafe fn svwhilegt_b64_s64(op1: i64, op2: i64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b64_s64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_s64(op1, op2).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b8_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b8_u32(op1: i32, op2: i32) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_u32(op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b16_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b16_u32(op1: i32, op2: i32) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b32_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b32_u32(op1: i32, op2: i32) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b64_u32(op1: u32, op2: u32) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b64_u32(op1: i32, op2: i32) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_u32(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b8_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b8_u64(op1: i64, op2: i64) -> svbool_t;
    }
    unsafe { _svwhilegt_b8_u64(op1.as_signed(), op2.as_signed()) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b16_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b16_u64(op1: i64, op2: i64) -> svbool8_t;
    }
    unsafe { _svwhilegt_b16_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b32_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b32_u64(op1: i64, op2: i64) -> svbool4_t;
    }
    unsafe { _svwhilegt_b32_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilehi))]
pub unsafe fn svwhilegt_b64_u64(op1: u64, op2: u64) -> svbool_t {
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilegt_b64_u64(op1: i64, op2: i64) -> svbool2_t;
    }
    unsafe { _svwhilegt_b64_u64(op1.as_signed(), op2.as_signed()).into() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_8ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilerw_8ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool_t;
    }
    _svwhilerw_8ptr(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_16ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilerw_16ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool8_t;
    }
    _svwhilerw_16ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_32ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilerw_32ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool4_t;
    }
    _svwhilerw_32ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilerw_64ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilerw_64ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool2_t;
    }
    _svwhilerw_64ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_f32(op1: *const f32, op2: *const f32) -> svbool_t {
    svwhilerw_32ptr::<f32>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_f64(op1: *const f64, op2: *const f64) -> svbool_t {
    svwhilerw_64ptr::<f64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s8(op1: *const i8, op2: *const i8) -> svbool_t {
    svwhilerw_8ptr::<i8>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s16(op1: *const i16, op2: *const i16) -> svbool_t {
    svwhilerw_16ptr::<i16>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s32(op1: *const i32, op2: *const i32) -> svbool_t {
    svwhilerw_32ptr::<i32>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_s64(op1: *const i64, op2: *const i64) -> svbool_t {
    svwhilerw_64ptr::<i64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u8(op1: *const u8, op2: *const u8) -> svbool_t {
    svwhilerw_8ptr::<u8>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u16(op1: *const u16, op2: *const u16) -> svbool_t {
    svwhilerw_16ptr::<u16>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u32(op1: *const u32, op2: *const u32) -> svbool_t {
    svwhilerw_32ptr::<u32>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilerw))]
pub unsafe fn svwhilerw_u64(op1: *const u64, op2: *const u64) -> svbool_t {
    svwhilerw_64ptr::<u64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_8ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilewr_8ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool_t;
    }
    _svwhilewr_8ptr(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_16ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilewr_16ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool8_t;
    }
    _svwhilewr_16ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_32ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilewr_32ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool4_t;
    }
    _svwhilewr_32ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
unsafe fn svwhilewr_64ptr<T>(op1: *const T, op2: *const T) -> svbool_t {
    let op1 = op1 as *const crate::ffi::c_void;
    let op2 = op2 as *const crate::ffi::c_void;
    extern "C" {
        #[cfg_attr(
            target_arch = "aarch64",
            feature(link_llvm_intrinsics)
        )]
        fn _svwhilewr_64ptr(
            op1: *const crate::ffi::c_void,
            op2: *const crate::ffi::c_void,
        ) -> svbool2_t;
    }
    _svwhilewr_64ptr(op1, op2).into()
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_f32(op1: *const f32, op2: *const f32) -> svbool_t {
    svwhilewr_32ptr::<f32>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_f64(op1: *const f64, op2: *const f64) -> svbool_t {
    svwhilewr_64ptr::<f64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s8(op1: *const i8, op2: *const i8) -> svbool_t {
    svwhilewr_8ptr::<i8>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s16(op1: *const i16, op2: *const i16) -> svbool_t {
    svwhilewr_16ptr::<i16>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s32(op1: *const i32, op2: *const i32) -> svbool_t {
    svwhilewr_32ptr::<i32>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_s64(op1: *const i64, op2: *const i64) -> svbool_t {
    svwhilewr_64ptr::<i64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u8(op1: *const u8, op2: *const u8) -> svbool_t {
    svwhilewr_8ptr::<u8>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u16(op1: *const u16, op2: *const u16) -> svbool_t {
    svwhilewr_16ptr::<u16>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u32(op1: *const u32, op2: *const u32) -> svbool_t {
    svwhilewr_32ptr::<u32>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(whilewr))]
pub unsafe fn svwhilewr_u64(op1: *const u64, op2: *const u64) -> svbool_t {
    svwhilewr_64ptr::<u64>(op1, op2)
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_s8<const IMM3: i32>(op1: svint8_t, op2: svint8_t) -> svint8_t {
    //static_assert_range!(IMM3, 1, 8);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svxar_n_s8(op1: svint8_t, op2: svint8_t, imm3: i32) -> svint8_t;
    }
    unsafe { _svxar_n_s8(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_s16<const IMM3: i32>(op1: svint16_t, op2: svint16_t) -> svint16_t {
    //static_assert_range!(IMM3, 1, 16);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svxar_n_s16(op1: svint16_t, op2: svint16_t, imm3: i32) -> svint16_t;
    }
    unsafe { _svxar_n_s16(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_s32<const IMM3: i32>(op1: svint32_t, op2: svint32_t) -> svint32_t {
    //static_assert_range!(IMM3, 1, 32);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svxar_n_s32(op1: svint32_t, op2: svint32_t, imm3: i32) -> svint32_t;
    }
    unsafe { _svxar_n_s32(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_s64<const IMM3: i32>(op1: svint64_t, op2: svint64_t) -> svint64_t {
    //static_assert_range!(IMM3, 1, 64);
    extern "C" {
        #[cfg_attr(target_arch = "aarch64", feature(link_llvm_intrinsics))]
        fn _svxar_n_s64(op1: svint64_t, op2: svint64_t, imm3: i32) -> svint64_t;
    }
    unsafe { _svxar_n_s64(op1, op2, IMM3) }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_u8<const IMM3: i32>(op1: svuint8_t, op2: svuint8_t) -> svuint8_t {
    //static_assert_range!(IMM3, 1, 8);
    unsafe { svxar_n_s8::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_u16<const IMM3: i32>(op1: svuint16_t, op2: svuint16_t) -> svuint16_t {
    //static_assert_range!(IMM3, 1, 16);
    unsafe { svxar_n_s16::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_u32<const IMM3: i32>(op1: svuint32_t, op2: svuint32_t) -> svuint32_t {
    //static_assert_range!(IMM3, 1, 32);
    unsafe { svxar_n_s32::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
#[inline]
#[target_feature(enable = "sve,sve2")]
//#[cfg_attr(test, assert_instr(xar, IMM3 = 1))]
pub unsafe fn svxar_n_u64<const IMM3: i32>(op1: svuint64_t, op2: svuint64_t) -> svuint64_t {
    //static_assert_range!(IMM3, 1, 64);
    unsafe { svxar_n_s64::<IMM3>(op1.as_signed(), op2.as_signed()).as_unsigned() }
}
