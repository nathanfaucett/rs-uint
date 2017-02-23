use collections::string::ToString;
use core::ops::*;

use abs::Abs;
use bounded::Bounded;
use clamp::Clamp;
use to_primitive::ToPrimitive;
use from_primitive::FromPrimitive;
use max::Max;
use min::Min;
use one::One;
use signum::Signum;
use zero::Zero;


pub trait UInt:
    Copy + One + Zero
    + Min + Max + Signum
    + Abs
    + Bounded
    + Clamp
    + ToPrimitive
    + FromPrimitive
    + PartialEq
    + PartialOrd
    + ToString

    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>

    + AddAssign<Self>
    + MulAssign<Self>
    + SubAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>

    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;
    fn signed_shl(self, n: u32) -> Self;
    fn signed_shr(self, n: u32) -> Self;
    fn unsigned_shl(self, n: u32) -> Self;
    fn unsigned_shr(self, n: u32) -> Self;
    fn swap_bytes(self) -> Self;
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
    fn pow(self, exp: u32) -> Self;
}


macro_rules! impl_uint {
    ($T:ty, $S:ty, $U:ty) => (
        impl UInt for $T {
            #[inline]
            fn count_ones(self) -> u32 {
                <$T>::count_ones(self)
            }
            #[inline]
            fn count_zeros(self) -> u32 {
                <$T>::count_zeros(self)
            }
            #[inline]
            fn leading_zeros(self) -> u32 {
                <$T>::leading_zeros(self)
            }
            #[inline]
            fn trailing_zeros(self) -> u32 {
                <$T>::trailing_zeros(self)
            }
            #[inline]
            fn rotate_left(self, n: u32) -> Self {
                <$T>::rotate_left(self, n)
            }
            #[inline]
            fn rotate_right(self, n: u32) -> Self {
                <$T>::rotate_right(self, n)
            }
            #[inline]
            fn signed_shl(self, n: u32) -> Self {
                ((self as $S) << n) as $T
            }
            #[inline]
            fn signed_shr(self, n: u32) -> Self {
                ((self as $S) >> n) as $T
            }
            #[inline]
            fn unsigned_shl(self, n: u32) -> Self {
                ((self as $U) << n) as $T
            }
            #[inline]
            fn unsigned_shr(self, n: u32) -> Self {
                ((self as $U) >> n) as $T
            }
            #[inline]
            fn swap_bytes(self) -> Self {
                <$T>::swap_bytes(self)
            }
            #[inline]
            fn from_be(x: Self) -> Self {
                <$T>::from_be(x)
            }
            #[inline]
            fn from_le(x: Self) -> Self {
                <$T>::from_le(x)
            }
            #[inline]
            fn to_be(self) -> Self {
                <$T>::to_be(self)
            }
            #[inline]
            fn to_le(self) -> Self {
                <$T>::to_le(self)
            }
            #[inline]
            fn pow(self, exp: u32) -> Self {
                <$T>::pow(self, exp)
            }
        }
    )
}

impl_uint!(u8,    i8,    u8);
impl_uint!(u16,   i16,   u16);
impl_uint!(u32,   i32,   u32);
impl_uint!(u64,   i64,   u64);
impl_uint!(usize, isize, usize);

impl_uint!(i8,    i8,    u8);
impl_uint!(i16,   i16,   u16);
impl_uint!(i32,   i32,   u32);
impl_uint!(i64,   i64,   u64);
impl_uint!(isize, isize, usize);
