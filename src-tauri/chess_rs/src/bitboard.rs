use std::{
    fmt,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Not, Shr,
        ShrAssign,
    },
};

use crate::core::{Square, FILES, RANKS};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard::new()
    }
}

impl BitBoard {
    #[inline(always)]
    pub fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub fn get_bit(&self, s: Square) -> bool {
        self.0 & (1 << s as u8) > 0
    }

    #[inline(always)]
    pub fn set_bit(&mut self, s: Square) {
        self.0 |= 1 << s as u8
    }

    #[inline(always)]
    pub fn unset_bit(&mut self, s: Square) {
        self.0 &= !(1 << s as u8)
    }

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            let sq = self.get_lsb();
            self.0 &= self.0 - 1;
            sq
        }
    }

    #[inline(always)]
    pub fn get_lsb(&self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { std::mem::transmute::<u8, Square>(self.0.trailing_zeros() as u8) })
        }
    }

    #[inline(always)]
    pub fn get_msb(&self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { std::mem::transmute::<u8, Square>(63 - self.0.leading_zeros() as u8) })
        }
    }

    #[inline(always)]
    pub fn pop_count(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    pub fn wrapping_mul(&self, n: u64) -> BitBoard {
        BitBoard(self.0.wrapping_mul(n))
    }
}

macro_rules! impl_indv_bit_ops {
    ($t:ty, $b:ty, $tname:ident, $fname:ident, $ta_name:ident, $fa_name:ident) => {
        impl $tname for $t {
            type Output = $t;

            #[inline(always)]
            fn $fname(self, rhs: $t) -> $t {
                Self(self.0.$fname(rhs.0))
            }
        }

        impl $ta_name for $t {
            #[inline(always)]
            fn $fa_name(&mut self, rhs: $t) {
                self.0.$fa_name(rhs.0);
            }
        }

        impl $tname<$b> for $t {
            type Output = $t;

            #[inline(always)]
            fn $fname(self, rhs: $b) -> $t {
                Self(self.0.$fname(rhs))
            }
        }

        impl $ta_name<$b> for $t {
            #[inline(always)]
            fn $fa_name(&mut self, rhs: $b) {
                self.0.$fa_name(rhs);
            }
        }
    };
}

macro_rules! impl_bit_ops {
    ($t:ty, $b:ty) => {
        impl_indv_bit_ops!($t, $b, BitAnd, bitand, BitAndAssign, bitand_assign);
        impl_indv_bit_ops!($t, $b, BitOr, bitor, BitOrAssign, bitor_assign);
        impl_indv_bit_ops!($t, $b, BitXor, bitxor, BitXorAssign, bitxor_assign);
        impl_indv_bit_ops!($t, $b, Mul, mul, MulAssign, mul_assign);
        impl_indv_bit_ops!($t, $b, Shr, shr, ShrAssign, shr_assign);
    };
}

impl_bit_ops!(BitBoard, u64);

// FIXME: Can I add this to the macros?
impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push('\n');
        for rank in RANKS.iter().rev() {
            s.push_str(&format!("{}    ", rank));
            for file in FILES {
                let square = Square::from((file, *rank));
                let bit = if self.get_bit(square) { "1" } else { "0" };
                s.push_str(&format!(" {} ", bit));
            }
            s.push('\n');
        }
        s.push('\n');
        s.push_str("      a  b  c  d  e  f  g  h");
        s.push('\n');

        f.pad(&s)
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            self.pop_lsb()
        }
    }
}
