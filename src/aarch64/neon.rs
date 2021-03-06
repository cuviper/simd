use super::super::*;
use {simd_cast, f32x2};

pub use sixty_four::{f64x2, i64x2, u64x2, bool64ix2, bool64fx2};

extern "platform-intrinsic" {
    fn aarch64_vsqrtq_f32(x: f32x4) -> f32x4;

    fn aarch64_vrsqrteq_f32(x: f32x4) -> f32x4;
    fn aarch64_vrecpeq_f32(x: f32x4) -> f32x4;

    fn aarch64_vminq_f32(x: f32x4, y: f32x4) -> f32x4;
    fn aarch64_vmaxq_f32(x: f32x4, y: f32x4) -> f32x4;

    fn aarch64_vmaxvq_u8(x: u8x16) -> u8;
    fn aarch64_vmaxvq_u16(x: u16x8) -> u16;
    fn aarch64_vmaxvq_u32(x: u32x4) -> u32;
    fn aarch64_vminvq_u8(x: u8x16) -> u8;
    fn aarch64_vminvq_u16(x: u16x8) -> u16;
    fn aarch64_vminvq_u32(x: u32x4) -> u32;

    fn aarch64_vqtbl1q_u8(x: u8x16, y: u8x16) -> u8x16;
    fn aarch64_vqtbl1q_s8(x: i8x16, y: i8x16) -> i8x16;
}

pub trait Aarch64F32x4 {
    fn to_f64(self) -> f64x2;
}
impl Aarch64F32x4 for f32x4 {
    #[inline]
    fn to_f64(self) -> f64x2 {
        unsafe {
            simd_cast(f32x2(self.0, self.1))
        }
    }
}

pub trait Aarch64U8x16 {
    fn table_lookup_1(self, t0: u8x16) -> u8x16;
}
impl Aarch64U8x16 for u8x16 {
    #[inline]
    fn table_lookup_1(self, t0: u8x16) -> u8x16 {
        unsafe {aarch64_vqtbl1q_u8(t0, self)}
    }
}
pub trait Aarch64I8x16 {
    fn table_lookup_1(self, t0: i8x16) -> i8x16;
}
impl Aarch64I8x16 for i8x16 {
    #[inline]
    fn table_lookup_1(self, t0: i8x16) -> i8x16 {
        unsafe {aarch64_vqtbl1q_s8(t0, self)}
    }
}

#[doc(hidden)]
pub mod common {
    use super::super::super::*;
    use std::mem;

    #[inline]
    pub fn f32x4_sqrt(x: f32x4) -> f32x4 {
        unsafe {super::aarch64_vsqrtq_f32(x)}
    }
    #[inline]
    pub fn f32x4_approx_rsqrt(x: f32x4) -> f32x4 {
        unsafe {super::aarch64_vrsqrteq_f32(x)}
    }
    #[inline]
    pub fn f32x4_approx_reciprocal(x: f32x4) -> f32x4 {
        unsafe {super::aarch64_vrecpeq_f32(x)}
    }
    #[inline]
    pub fn f32x4_max(x: f32x4, y: f32x4) -> f32x4 {
        unsafe {super::aarch64_vmaxq_f32(x, y)}
    }
    #[inline]
    pub fn f32x4_min(x: f32x4, y: f32x4) -> f32x4 {
        unsafe {super::aarch64_vminq_f32(x, y)}
    }

    macro_rules! bools {
        ($($ty: ty, $all: ident ($min: ident), $any: ident ($max: ident);)*) => {
            $(
                #[inline]
                pub fn $all(x: $ty) -> bool {
                    unsafe {
                        super::$min(mem::transmute(x)) != 0
                    }
                }
                #[inline]
                pub fn $any(x: $ty) -> bool {
                    unsafe {
                        super::$max(mem::transmute(x)) != 0
                    }
                }
                )*
        }
    }

    bools! {
        bool32fx4, bool32fx4_all(aarch64_vminvq_u32), bool32fx4_any(aarch64_vmaxvq_u32);
        bool8ix16, bool8ix16_all(aarch64_vminvq_u8), bool8ix16_any(aarch64_vmaxvq_u8);
        bool16ix8, bool16ix8_all(aarch64_vminvq_u16), bool16ix8_any(aarch64_vmaxvq_u16);
        bool32ix4, bool32ix4_all(aarch64_vminvq_u32), bool32ix4_any(aarch64_vmaxvq_u32);
    }
}
