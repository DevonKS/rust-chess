#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

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
        self.0 = self.0 | (1 << s as u8)
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
        for rank in (0..8).rev() {
            print!("{}    ", rank + 1);
            for file in 0..8 {
                let square_index = rank * 8 + file;
                let square = Square::try_from(square_index)
                    .unwrap_or_else(|_| panic!("Invalid square index: {}", square_index));
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl TryFrom<u8> for Square {
    type Error = &'static str;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Square::A1),
            1 => Ok(Square::B1),
            2 => Ok(Square::C1),
            3 => Ok(Square::D1),
            4 => Ok(Square::E1),
            5 => Ok(Square::F1),
            6 => Ok(Square::G1),
            7 => Ok(Square::H1),
            8 => Ok(Square::A2),
            9 => Ok(Square::B2),
            10 => Ok(Square::C2),
            11 => Ok(Square::D2),
            12 => Ok(Square::E2),
            13 => Ok(Square::F2),
            14 => Ok(Square::G2),
            15 => Ok(Square::H2),
            16 => Ok(Square::A3),
            17 => Ok(Square::B3),
            18 => Ok(Square::C3),
            19 => Ok(Square::D3),
            20 => Ok(Square::E3),
            21 => Ok(Square::F3),
            22 => Ok(Square::G3),
            23 => Ok(Square::H3),
            24 => Ok(Square::A4),
            25 => Ok(Square::B4),
            26 => Ok(Square::C4),
            27 => Ok(Square::D4),
            28 => Ok(Square::E4),
            29 => Ok(Square::F4),
            30 => Ok(Square::G4),
            31 => Ok(Square::H4),
            32 => Ok(Square::A5),
            33 => Ok(Square::B5),
            34 => Ok(Square::C5),
            35 => Ok(Square::D5),
            36 => Ok(Square::E5),
            37 => Ok(Square::F5),
            38 => Ok(Square::G5),
            39 => Ok(Square::H5),
            40 => Ok(Square::A6),
            41 => Ok(Square::B6),
            42 => Ok(Square::C6),
            43 => Ok(Square::D6),
            44 => Ok(Square::E6),
            45 => Ok(Square::F6),
            46 => Ok(Square::G6),
            47 => Ok(Square::H6),
            48 => Ok(Square::A7),
            49 => Ok(Square::B7),
            50 => Ok(Square::C7),
            51 => Ok(Square::D7),
            52 => Ok(Square::E7),
            53 => Ok(Square::F7),
            54 => Ok(Square::G7),
            55 => Ok(Square::H7),
            56 => Ok(Square::A8),
            57 => Ok(Square::B8),
            58 => Ok(Square::C8),
            59 => Ok(Square::D8),
            60 => Ok(Square::E8),
            61 => Ok(Square::F8),
            62 => Ok(Square::G8),
            63 => Ok(Square::H8),
            _ => Err("Square only accepts values less than 63"),
        }
    }
}
