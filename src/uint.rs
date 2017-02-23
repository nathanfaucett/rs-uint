use collections::string::ToString;
use core::ops::*;
use core::mem;

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
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
    fn is_multiple_of(self, other: Self) -> bool;
    fn is_even(self) -> bool;
    fn is_odd(self) -> bool;
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
            #[inline]
            fn gcd(self, other: Self) -> Self {
                let mut m = self;
                let mut n = other;

                if m == 0 || n == 0 {
                    (m | n).abs()
                } else {
                    let shift = (m | n).trailing_zeros();

                    if m == Self::min_value() || n == Self::min_value() {
                        return (1 << shift).abs()
                    }

                    m = m.abs();
                    n = n.abs();
                    n >>= n.trailing_zeros();

                    while m != 0 {
                        m >>= m.trailing_zeros();
                        if n > m {
                            mem::swap(&mut n, &mut m)
                        }
                        m -= n;
                    }

                    n << shift
                }
            }
            #[inline]
            fn lcm(self, other: Self) -> Self {
                (self * (other / self.gcd(other))).abs()
            }
            #[inline]
            fn is_multiple_of(self, other: Self) -> bool {
                self % other == 0
            }
            #[inline]
            fn is_even(self) -> bool {
                (self) & 1 == 0
            }
            #[inline]
            fn is_odd(self) -> bool {
                !self.is_even()
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
