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
    en_passant: Square,
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
                en_passant: Square::A1,
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

        b.state.en_passant =
            Square::try_from(caps.get(4).ok_or("expected en passant sqaure")?.as_str())?;

        b.state.half_moves = caps
            .get(5)
            .ok_or("expected half move count")?
            .as_str()
            .parse()
            .map_err(|_| "unable to parse half move count")?;

        b.state.full_moves = caps
            .get(6)
            .ok_or("expected full move count")?
            .as_str()
            .parse()
            .map_err(|_| "unable to parse full move count")?;

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

        let moved_piece = self.moved_piece(m).unwrap();
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

    fn moved_piece(&self, m: Move) -> Option<Piece> {
        for i in 0..12 {
            if self.state.piece_bbs[i].get_bit(m.0) {
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
    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    // r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
    // r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
    // 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -
    // rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8
    // r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10

    #[test]
    fn from_fen_starting_position() {
        let l = LookupTables::new();

        let b = Board::from_fen(
            "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            &l,
        )
        .unwrap();

        let expected_state = BoardState {
            turn: Player::White,
            piece_bbs: [bitboard::BitBoard::new(); 12],
            occ_bbs: [bitboard::BitBoard::new(); 3],
            castling: Castling::ALL,
            en_passant: Square::D6,
            half_moves: 0,
            full_moves: 3,
        };

        assert_eq!(b.state, expected_state);
    }
}
