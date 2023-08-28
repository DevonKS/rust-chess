use crate::bitboard;
use crate::lookup_tables;

const MAX_MOVES: usize = 250;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Piece {
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    WhitePawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    BlackPawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PieceKind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl From<Piece> for PieceKind {
    fn from(p: Piece) -> Self {
        match p {
            Piece::WhiteRook => PieceKind::Rook,
            Piece::WhiteKnight => PieceKind::Knight,
            Piece::WhiteBishop => PieceKind::Bishop,
            Piece::WhiteQueen => PieceKind::Queen,
            Piece::WhiteKing => PieceKind::King,
            Piece::WhitePawn => PieceKind::Pawn,
            Piece::BlackRook => PieceKind::Rook,
            Piece::BlackKnight => PieceKind::Knight,
            Piece::BlackBishop => PieceKind::Bishop,
            Piece::BlackQueen => PieceKind::Queen,
            Piece::BlackKing => PieceKind::King,
            Piece::BlackPawn => PieceKind::Pawn,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Player {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move(pub bitboard::Square, pub bitboard::Square);

#[derive(Copy, Clone, Debug)]
struct BoardState {
    turn: Player,
    piece_bbs: [bitboard::BitBoard; 12],
    occ_bbs: [bitboard::BitBoard; 3],
}

#[derive(Debug)]
pub struct Board {
    state: BoardState,
    previous_states: Vec<BoardState>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: BoardState {
                turn: Player::White,
                piece_bbs: [bitboard::BitBoard::new(); 12],
                occ_bbs: [bitboard::BitBoard::new(); 3],
            },
            previous_states: Vec::new(),
        }
    }

    // FIXME: I should use from_fen to set this up.
    pub fn start_pos() -> Self {
        let mut b = Board::new();
        b.state.piece_bbs[Piece::WhiteRook as usize].set_bit(bitboard::Square::A1);
        b.state.piece_bbs[Piece::WhiteRook as usize].set_bit(bitboard::Square::H1);
        b.state.piece_bbs[Piece::WhiteKnight as usize].set_bit(bitboard::Square::B1);
        b.state.piece_bbs[Piece::WhiteKnight as usize].set_bit(bitboard::Square::G1);
        b.state.piece_bbs[Piece::WhiteBishop as usize].set_bit(bitboard::Square::C1);
        b.state.piece_bbs[Piece::WhiteBishop as usize].set_bit(bitboard::Square::F1);
        b.state.piece_bbs[Piece::WhiteQueen as usize].set_bit(bitboard::Square::D1);
        b.state.piece_bbs[Piece::WhiteKing as usize].set_bit(bitboard::Square::E1);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::A2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::B2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::C2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::D2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::E2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::F2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::G2);
        b.state.piece_bbs[Piece::WhitePawn as usize].set_bit(bitboard::Square::H2);

        b.state.piece_bbs[Piece::BlackRook as usize].set_bit(bitboard::Square::A8);
        b.state.piece_bbs[Piece::BlackRook as usize].set_bit(bitboard::Square::H8);
        b.state.piece_bbs[Piece::BlackKnight as usize].set_bit(bitboard::Square::B8);
        b.state.piece_bbs[Piece::BlackKnight as usize].set_bit(bitboard::Square::G8);
        b.state.piece_bbs[Piece::BlackBishop as usize].set_bit(bitboard::Square::C8);
        b.state.piece_bbs[Piece::BlackBishop as usize].set_bit(bitboard::Square::F8);
        b.state.piece_bbs[Piece::BlackQueen as usize].set_bit(bitboard::Square::D8);
        b.state.piece_bbs[Piece::BlackKing as usize].set_bit(bitboard::Square::E8);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::A7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::B7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::C7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::D7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::E7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::F7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::G7);
        b.state.piece_bbs[Piece::BlackPawn as usize].set_bit(bitboard::Square::H7);

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
            PieceKind::Knight => self.generate_knight_moves(p, moves),
            PieceKind::Pawn => self.generate_pawn_moves(p, moves),
            PieceKind::King => self.generate_king_moves(p, moves),
            _ => (), // noop
        }
    }

    fn generate_knight_moves(&self, p: Piece, moves: &mut Vec<Move>) {
        let mut piece_bb = self.state.piece_bbs[p as usize];
        while let Some(from) = piece_bb.pop_lsb() {
            let move_bb = lookup_tables::knight_moves(from);
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
            let move_bb = lookup_tables::pawn_moves(from, is_white);
            let all_occ = self.state.occ_bbs[2];
            let mut valid_moves_bb = bitboard::BitBoard(move_bb.0 & (!all_occ.0));
            while let Some(to) = valid_moves_bb.pop_lsb() {
                moves.push(Move(from, to));
            }

            let capture_move_bb = lookup_tables::pawn_capture_moves(from, is_white);
            let enemy_occ = self.state.occ_bbs[if is_white { 1 } else { 0 }];
            let mut valid_capture_moves_bb = bitboard::BitBoard(capture_move_bb.0 & enemy_occ.0);
            while let Some(to) = valid_capture_moves_bb.pop_lsb() {
                moves.push(Move(from, to));
            }
        }
    }

    fn generate_king_moves(&self, p: Piece, moves: &mut Vec<Move>) {
        let mut piece_bb = self.state.piece_bbs[p as usize];
        while let Some(from) = piece_bb.pop_lsb() {
            let move_bb = lookup_tables::king_moves(from);
            let occ = self.state.occ_bbs[self.state.turn as usize];
            let mut valid_moves_bb = bitboard::BitBoard(move_bb.0 & (!occ.0));
            while let Some(to) = valid_moves_bb.pop_lsb() {
                moves.push(Move(from, to));
            }
        }
    }
}