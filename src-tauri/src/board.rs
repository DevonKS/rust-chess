use crate::bitboard;
use crate::core::{Move, Piece, PieceKind, Player, Square, MAX_MOVES};
use crate::lookup_tables;

use bitflags::bitflags;
use regex::Regex;

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Castling: u8 {
        const NONE = 0b00000000;
        const WHITE_K = 0b00000001;
        const WHITE_Q = 0b00000010;
        const BLACK_K = 0b00000100;
        const BLACK_Q = 0b00001000;
        const WHITE_ALL = 0b00000011;
        const BLACK_ALL = 0b00001100;
        const ALL = 0b00001111;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BoardState {
    turn: Player,
    piece_bbs: [bitboard::BitBoard; 12],
    occ_bbs: [bitboard::BitBoard; 3],
    castling: Castling,
    en_passant: Option<Square>,
    half_moves: u32,
    full_moves: u32,
}

#[derive(Debug)]
pub struct Board<'a> {
    state: BoardState,
    previous_states: Vec<BoardState>,
    lookup_tables: &'a lookup_tables::LookupTables,
}

impl<'a> Board<'a> {
    pub fn new(l: &'a lookup_tables::LookupTables) -> Self {
        Self {
            state: BoardState {
                turn: Player::White,
                piece_bbs: [bitboard::BitBoard::new(); 12],
                occ_bbs: [bitboard::BitBoard::new(); 3],
                castling: Castling::ALL,
                en_passant: None,
                half_moves: 0,
                full_moves: 0,
            },
            previous_states: Vec::new(),
            lookup_tables: l,
        }
    }

    pub fn from_fen(fen: &str, l: &'a lookup_tables::LookupTables) -> Result<Self, String> {
        let mut b = Board::new(l);

        let re = Regex::new(
            r"^([rnbqkpRNBQKP/1-8]+)\s+(w|b)\s+([KQkq]+|-)\s+(-|[a-h][1-8])\s*(\d*)\s*(\d*)\s*$",
        )
        .unwrap();

        let caps = re.captures(fen).ok_or("invalid fen")?;

        let mut current_index: u8 = 56;
        for row in caps
            .get(1)
            .ok_or("expected piece placement")?
            .as_str()
            .split("/")
        {
            for c in row.chars() {
                match c {
                    'R' | 'N' | 'B' | 'Q' | 'K' | 'P' | 'r' | 'n' | 'b' | 'q' | 'k' | 'p' => {
                        let (bb_index, occ_index) = match c {
                            'R' => (Piece::WhiteRook as usize, Player::White as usize),
                            'N' => (Piece::WhiteKnight as usize, Player::White as usize),
                            'B' => (Piece::WhiteBishop as usize, Player::White as usize),
                            'Q' => (Piece::WhiteQueen as usize, Player::White as usize),
                            'K' => (Piece::WhiteKing as usize, Player::White as usize),
                            'P' => (Piece::WhitePawn as usize, Player::White as usize),
                            'r' => (Piece::BlackRook as usize, Player::Black as usize),
                            'n' => (Piece::BlackKnight as usize, Player::Black as usize),
                            'b' => (Piece::BlackBishop as usize, Player::Black as usize),
                            'q' => (Piece::BlackQueen as usize, Player::Black as usize),
                            'k' => (Piece::BlackKing as usize, Player::Black as usize),
                            'p' => (Piece::BlackPawn as usize, Player::Black as usize),
                            x => panic!("unexpected piece placement character: {}", x),
                        };
                        let current_square = Square::try_from(current_index).unwrap();
                        b.state.piece_bbs[bb_index].set_bit(current_square);
                        b.state.occ_bbs[occ_index].set_bit(current_square);
                        b.state.occ_bbs[2].set_bit(current_square);
                        current_index += 1;
                    }
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        current_index += c.to_digit(10).unwrap() as u8;
                    }
                    x => panic!("unexpected piece placement character: {}", x),
                };
            }
            if current_index >= 16 {
                current_index -= 16;
            }
        }

        b.state.turn = match caps.get(2).ok_or("expected active color")?.as_str() {
            "w" => Player::White,
            "b" => Player::Black,
            active_color => panic!("unknown active color: {}", active_color),
        };

        b.state.castling = caps
            .get(3)
            .ok_or("expected castling rights")?
            .as_str()
            .chars()
            .fold(Castling::NONE, |mut rights, c| {
                match c {
                    'K' => rights |= Castling::WHITE_K,
                    'Q' => rights |= Castling::WHITE_Q,
                    'k' => rights |= Castling::BLACK_K,
                    'q' => rights |= Castling::BLACK_Q,
                    '-' => (), // noop
                    r => panic!("unknown castling right: {}", r),
                };
                rights
            });

        b.state.en_passant = match caps.get(4).ok_or("expected en passant sqaure")?.as_str() {
            "-" => None,
            s => Some(Square::try_from(s)?),
        };

        b.state.half_moves = caps
            .get(5)
            .ok_or("expected half move count")?
            .as_str()
            .parse()
            .unwrap_or(0);

        b.state.full_moves = caps
            .get(6)
            .ok_or("expected full move count")?
            .as_str()
            .parse()
            .unwrap_or(0);

        Ok(b)
    }

    // FIXME: I should use from_fen to set this up.
    pub fn start_pos(l: &'a lookup_tables::LookupTables) -> Self {
        let mut b = Board::new(l);
        b.state.piece_bbs[Piece::WhiteRook as usize].set_bit(Square::A1);
        b.state.piece_bbs[Piece::WhiteRook as usize].set_bit(Square::H1);
        b.state.piece_bbs[Piece::WhiteKnight as usize].set_bit(Square::B1);
        b.state.piece_bbs[Piece::WhiteKnight as usize].set_bit(Square::G1);
        b.state.piece_bbs[Piece::WhiteBishop as usize].set_bit(Square::C1);
        b.state.piece_bbs[Piece::WhiteBishop as usize].set_bit(Square::F1);
        b.state.piece_bbs[Piece::WhiteQueen as usize].set_bit(Square::D1);
        b.state.piece_bbs[Piece::WhiteKing as usize].set_bit(Square::E1);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::A2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::B2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::C2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::D2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::E2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::F2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::G2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(Square::H2);

        b.state.piece_bbs[Piece::BlackRook as usize].set_bit(Square::A8);
        b.state.piece_bbs[Piece::BlackRook as usize].set_bit(Square::H8);
        b.state.piece_bbs[Piece::BlackKnight as usize].set_bit(Square::B8);
        b.state.piece_bbs[Piece::BlackKnight as usize].set_bit(Square::G8);
        b.state.piece_bbs[Piece::BlackBishop as usize].set_bit(Square::C8);
        b.state.piece_bbs[Piece::BlackBishop as usize].set_bit(Square::F8);
        b.state.piece_bbs[Piece::BlackQueen as usize].set_bit(Square::D8);
        b.state.piece_bbs[Piece::BlackKing as usize].set_bit(Square::E8);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::A7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::B7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::C7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::D7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::E7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::F7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::G7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(Square::H7);

        for i in 0..6 {
            b.state.occ_bbs[Player::White as usize].0 |= b.state.piece_bbs[i].0;
        }

        for i in 6..12 {
            b.state.occ_bbs[Player::Black as usize].0 |= b.state.piece_bbs[i].0;
        }

        b.state.occ_bbs[2].0 = b.state.occ_bbs[0].0 | b.state.occ_bbs[1].0;

        b
    }

    pub fn shallow_clone(&self) -> Board {
        Board {
            state: self.state,
            previous_states: Vec::new(),
            lookup_tables: self.lookup_tables,
        }
    }

    pub fn print_bbs(&self) {
        for pbb in self.state.piece_bbs {
            pbb.print();
        }
        for pbb in self.state.occ_bbs {
            pbb.print();
        }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let white_pieces = [
            Piece::WhiteRook,
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteQueen,
            Piece::WhiteKing,
            Piece::WhitePawn,
        ];
        let black_pieces = [
            Piece::BlackRook,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackQueen,
            Piece::BlackKing,
            Piece::BlackPawn,
        ];
        let pieces = if self.state.turn == Player::White {
            white_pieces
        } else {
            black_pieces
        };

        let mut moves = Vec::with_capacity(MAX_MOVES);
        for p in pieces {
            self.generate_piece_moves(p, &mut moves);
        }

        moves
    }

    pub fn apply_move(&mut self, m: Move) {
        self.previous_states.push(self.state);

        let moved_piece = self.get_piece(m.0).unwrap();
        self.state.piece_bbs[moved_piece as usize].unset_bit(m.0);
        self.state.piece_bbs[moved_piece as usize].set_bit(m.1);

        self.state.occ_bbs[self.state.turn as usize].unset_bit(m.0);
        self.state.occ_bbs[self.state.turn as usize].set_bit(m.1);

        self.state.occ_bbs[2].unset_bit(m.0);
        self.state.occ_bbs[2].set_bit(m.1);

        self.state.turn = match self.state.turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };
    }

    pub fn undo_move(&mut self) {
        self.state = self.previous_states.pop().unwrap();
    }

    fn get_piece(&self, s: Square) -> Option<Piece> {
        for i in 0..12 {
            if self.state.piece_bbs[i].get_bit(s) {
                // FIXME: I should probably implement a try_from for this
                return unsafe { Some(std::mem::transmute::<u8, Piece>(i as u8)) };
            }
        }
        return None;
    }

    fn generate_piece_moves(&self, p: Piece, moves: &mut Vec<Move>) {
        let pk = PieceKind::from(p);
        match pk {
            PieceKind::Knight
            | PieceKind::King
            | PieceKind::Rook
            | PieceKind::Bishop
            | PieceKind::Queen => self.generate_piece_moves_inner(p, moves),
            PieceKind::Pawn => self.generate_pawn_moves(p, moves),
        }
    }

    // FIXME: better name
    fn generate_piece_moves_inner(&self, p: Piece, moves: &mut Vec<Move>) {
        let mut piece_bb = self.state.piece_bbs[p as usize];
        while let Some(from) = piece_bb.pop_lsb() {
            let move_bb = self
                .lookup_tables
                .lookup_moves(p, from, self.state.occ_bbs[2].0);
            let occ = self.state.occ_bbs[self.state.turn as usize];
            let mut valid_moves_bb = bitboard::BitBoard(move_bb.0 & (!occ.0));
            while let Some(to) = valid_moves_bb.pop_lsb() {
                moves.push(Move(from, to));
            }
        }
    }

    fn generate_pawn_moves(&self, p: Piece, moves: &mut Vec<Move>) {
        let mut piece_bb = self.state.piece_bbs[p as usize];
        while let Some(from) = piece_bb.pop_lsb() {
            let is_white = self.state.turn == Player::White;
            let move_bb = self
                .lookup_tables
                .lookup_moves(p, from, self.state.occ_bbs[2].0);
            let all_occ = self.state.occ_bbs[2];
            let mut valid_moves_bb = bitboard::BitBoard(move_bb.0 & (!all_occ.0));
            while let Some(to) = valid_moves_bb.pop_lsb() {
                moves.push(Move(from, to));
            }

            let square_index = from as u8;
            let player = Player::from(p);
            match player {
                Player::White => {
                    if square_index >= 8 && square_index <= 15 {
                        let check_index = square_index + 8;
                        let move_index = square_index + 16;
                        let check_bb = 1 << check_index;
                        if all_occ.0 & check_bb == 0 {
                            moves.push(Move(from, Square::try_from(move_index as u8).unwrap()))
                        }
                    }
                }
                Player::Black => {
                    if square_index >= 48 && square_index <= 55 {
                        let check_index = square_index - 8;
                        let move_index = square_index - 16;
                        let check_bb = 1 << check_index;
                        if all_occ.0 & check_bb == 0 {
                            moves.push(Move(from, Square::try_from(move_index as u8).unwrap()))
                        }
                    }
                }
            }

            let capture_move_bb = self.lookup_tables.lookup_capture_moves(p, from);
            let enemy_occ = self.state.occ_bbs[if is_white { 1 } else { 0 }];
            let mut valid_capture_moves_bb = bitboard::BitBoard(capture_move_bb.0 & enemy_occ.0);
            while let Some(to) = valid_capture_moves_bb.pop_lsb() {
                moves.push(Move(from, to));
            }
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
                let piece = self.get_piece(square);
                let piece_repr = match piece {
                    Some(p) => match p {
                        Piece::WhiteRook => "R",
                        Piece::WhiteKnight => "N",
                        Piece::WhiteBishop => "B",
                        Piece::WhiteQueen => "Q",
                        Piece::WhiteKing => "K",
                        Piece::WhitePawn => "P",
                        Piece::BlackRook => "r",
                        Piece::BlackKnight => "n",
                        Piece::BlackBishop => "b",
                        Piece::BlackQueen => "q",
                        Piece::BlackKing => "k",
                        Piece::BlackPawn => "p",
                    },
                    None => ".",
                };
                print!(" {} ", piece_repr);
            }
            println!();
        }
        println!();
        println!("      a  b  c  d  e  f  g  h");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard,
        board::{Board, BoardState, Castling},
        core::{Player, Square},
        lookup_tables::LookupTables,
    };

    // FIXME: Test these fens:
    // r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10

    #[test]
    fn from_fen_starting_position() {
        struct TestCase {
            name: &'static str,
            fen: &'static str,
            expected_state: BoardState,
        }
        let l = LookupTables::new();

        let test_cases = vec![
            TestCase {
                name: "en passant square",
                fen: "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
                expected_state: BoardState {
                    turn: Player::White,
                    piece_bbs: [
                        bitboard::BitBoard(129),
                        bitboard::BitBoard(66),
                        bitboard::BitBoard(36),
                        bitboard::BitBoard(8),
                        bitboard::BitBoard(16),
                        bitboard::BitBoard(68719537920),
                        bitboard::BitBoard(9295429630892703744),
                        bitboard::BitBoard(4755801206503243776),
                        bitboard::BitBoard(2594073385365405696),
                        bitboard::BitBoard(576460752303423488),
                        bitboard::BitBoard(1152921504606846976),
                        bitboard::BitBoard(69243978142187520),
                    ],
                    occ_bbs: [
                        bitboard::BitBoard(68719538175),
                        bitboard::BitBoard(18443930457813811200),
                        bitboard::BitBoard(18443930526533349375),
                    ],
                    castling: Castling::ALL,
                    en_passant: Some(Square::D6),
                    half_moves: 0,
                    full_moves: 3,
                },
            },
            TestCase {
                name: "starting position",
                fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                expected_state: BoardState {
                    turn: Player::White,
                    piece_bbs: [
                        bitboard::BitBoard(129),
                        bitboard::BitBoard(66),
                        bitboard::BitBoard(36),
                        bitboard::BitBoard(8),
                        bitboard::BitBoard(16),
                        bitboard::BitBoard(65280),
                        bitboard::BitBoard(9295429630892703744),
                        bitboard::BitBoard(4755801206503243776),
                        bitboard::BitBoard(2594073385365405696),
                        bitboard::BitBoard(576460752303423488),
                        bitboard::BitBoard(1152921504606846976),
                        bitboard::BitBoard(71776119061217280),
                    ],
                    occ_bbs: [
                        bitboard::BitBoard(65535),
                        bitboard::BitBoard(18446462598732840960),
                        bitboard::BitBoard(18446462598732906495),
                    ],
                    castling: Castling::ALL,
                    en_passant: None,
                    half_moves: 0,
                    full_moves: 1,
                },
            },
            TestCase {
                name: "kiwipete",
                fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
                expected_state: BoardState {
                    turn: Player::White,
                    piece_bbs: [
                        bitboard::BitBoard(129),
                        bitboard::BitBoard(68719738880),
                        bitboard::BitBoard(6144),
                        bitboard::BitBoard(2097152),
                        bitboard::BitBoard(16),
                        bitboard::BitBoard(34628232960),
                        bitboard::BitBoard(9295429630892703744),
                        bitboard::BitBoard(37383395344384),
                        bitboard::BitBoard(18015498021109760),
                        bitboard::BitBoard(4503599627370496),
                        bitboard::BitBoard(1152921504606846976),
                        bitboard::BitBoard(12754334924144640),
                    ],
                    occ_bbs: [
                        bitboard::BitBoard(103350075281),
                        bitboard::BitBoard(10483661951467520000),
                        bitboard::BitBoard(10483662054817595281),
                    ],
                    castling: Castling::ALL,
                    en_passant: None,
                    half_moves: 0,
                    full_moves: 0,
                },
            },
            TestCase {
                name: "position 3",
                fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -",
                expected_state: BoardState {
                    turn: Player::White,
                    piece_bbs: [
                        bitboard::BitBoard(33554432),
                        bitboard::BitBoard(0),
                        bitboard::BitBoard(0),
                        bitboard::BitBoard(0),
                        bitboard::BitBoard(4294967296),
                        bitboard::BitBoard(8589955072),
                        bitboard::BitBoard(549755813888),
                        bitboard::BitBoard(0),
                        bitboard::BitBoard(0),
                        bitboard::BitBoard(0),
                        bitboard::BitBoard(2147483648),
                        bitboard::BitBoard(1134696536735744),
                    ],
                    occ_bbs: [
                        bitboard::BitBoard(12918476800),
                        bitboard::BitBoard(1135248440033280),
                        bitboard::BitBoard(1135261358510080),
                    ],
                    castling: Castling::NONE,
                    en_passant: None,
                    half_moves: 0,
                    full_moves: 0,
                },
            },
            TestCase {
                name: "position 5",
                fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                expected_state: BoardState {
                    turn: Player::White,
                    piece_bbs: [
                        bitboard::BitBoard(129),
                        bitboard::BitBoard(4098),
                        bitboard::BitBoard(67108868),
                        bitboard::BitBoard(8),
                        bitboard::BitBoard(16),
                        bitboard::BitBoard(2251799813736192),
                        bitboard::BitBoard(9295429630892703744),
                        bitboard::BitBoard(144115188075864064),
                        bitboard::BitBoard(292733975779082240),
                        bitboard::BitBoard(576460752303423488),
                        bitboard::BitBoard(2305843009213693952),
                        bitboard::BitBoard(63899217759830016),
                    ],
                    occ_bbs: [
                        bitboard::BitBoard(2251799880849311),
                        bitboard::BitBoard(12678481774024597504),
                        bitboard::BitBoard(12680733573905446815),
                    ],
                    castling: Castling::WHITE_K | Castling::WHITE_Q,
                    en_passant: None,
                    half_moves: 1,
                    full_moves: 8,
                },
            },
            TestCase {
                name: "position 6",
                fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                expected_state: BoardState {
                    turn: Player::White,
                    piece_bbs: [
                        bitboard::BitBoard(33),
                        bitboard::BitBoard(2359296),
                        bitboard::BitBoard(274945015808),
                        bitboard::BitBoard(4096),
                        bitboard::BitBoard(64),
                        bitboard::BitBoard(269084160),
                        bitboard::BitBoard(2377900603251621888),
                        bitboard::BitBoard(39582418599936),
                        bitboard::BitBoard(18253611008),
                        bitboard::BitBoard(4503599627370496),
                        bitboard::BitBoard(4611686018427387904),
                        bitboard::BitBoard(64749208967577600),
                    ],
                    occ_bbs: [
                        bitboard::BitBoard(275216463457),
                        bitboard::BitBoard(7058879030946168832),
                        bitboard::BitBoard(7058879306162632289),
                    ],
                    castling: Castling::NONE,
                    en_passant: None,
                    half_moves: 0,
                    full_moves: 10,
                },
            },
        ];

        for test_case in test_cases {
            let b = Board::from_fen(test_case.fen, &l).unwrap();

            assert_eq!(
                b.state, test_case.expected_state,
                "{} failed",
                test_case.name
            );
        }
    }
}
