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
            // FIXME: I could probably implement a from_u8 fn but since I'm going to do this a lot
            // I'm doing this under the assumption that this will be much faster.
            Some(unsafe { std::mem::transmute::<u8, Square>(self.0.trailing_zeros() as u8) })
        }
    }

    #[inline(always)]
    pub fn get_msb(&self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            // FIXME: I could probably implement a from_u8 fn but since I'm going to do this a lot
            // I'm doing this under the assumption that this will be much faster.
            Some(unsafe { std::mem::transmute::<u8, Square>(63 - self.0.leading_zeros() as u8) })
        }
    }

    pub fn print(&self) {
        println!();
        for rank in RANKS.iter().rev() {
            print!("{}    ", rank);
            for file in FILES {
                let square = Square::from((file, *rank));
                let bit = if self.get_bit(square) { "1" } else { "0" };
                print!(" {} ", bit);
            }
            println!();
        }
        println!();
        println!("      a  b  c  d  e  f  g  h");
        println!();
        println!("{:#066b}", self.0);
        println!();
    }
}
