use std::{cmp, fmt};

use crate::bitboard::BitBoard;
use crate::core::{
    File, Move, Piece, PieceKind, Player, Rank, Square, BLACK_PIECES, FILES, MAX_MOVES, PIECES,
    RANKS, RANK_1, RANK_8, WHITE_PIECES,
};
use crate::lookup_tables;

use bitflags::bitflags;
use regex::Regex;

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Castling: u8 {
        const WHITE_K = 0b00000001;
        const WHITE_Q = 0b00000010;
        const BLACK_K = 0b00000100;
        const BLACK_Q = 0b00001000;
    }
}

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "-")
        } else {
            let mut s = String::new();

            if self.contains(Castling::WHITE_K) {
                s.push('K');
            }

            if self.contains(Castling::WHITE_Q) {
                s.push('Q');
            }

            if self.contains(Castling::BLACK_K) {
                s.push('k');
            }

            if self.contains(Castling::BLACK_Q) {
                s.push('q');
            }

            write!(f, "{}", s)
        }
    }
}

impl TryFrom<char> for Castling {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'K' => Ok(Castling::WHITE_K),
            'Q' => Ok(Castling::WHITE_Q),
            'k' => Ok(Castling::BLACK_K),
            'q' => Ok(Castling::BLACK_Q),
            '-' => Ok(Castling::empty()),
            _ => Err(format!("unknown castling right: {}", c)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Legality {
    Legal,
    PseudoLegal,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BoardState {
    turn: Player,
    piece_bbs: [BitBoard; 12],
    occ_bbs: [BitBoard; 3],
    castling: Castling,
    en_passant: Option<Square>,
    half_moves: u32,
    full_moves: u32,

    // State to help with move generation
    checkers: BitBoard,
    attacked_squares: BitBoard,
    pinned_pieces: BitBoard,
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
                piece_bbs: [BitBoard::new(); 12],
                occ_bbs: [BitBoard::new(); 3],
                castling: Castling::all(),
                en_passant: None,
                half_moves: 0,
                full_moves: 0,
                checkers: BitBoard::new(),
                attacked_squares: BitBoard::new(),
                pinned_pieces: BitBoard::new(),
            },
            previous_states: Vec::new(),
            lookup_tables: l,
        }
    }

    pub fn from_fen(fen: &str, l: &'a lookup_tables::LookupTables) -> Result<Self, String> {
        let mut b = Board::new(l);

        let re = Regex::new(
            r"^([rnbqkpRNBQKP/1-8]+)\s+(w|b)\s+([KQkq]+|-)\s+(-|[a-h][36])\s*(\d*)\s*(\d*)\s*$",
        )
        .unwrap();

        let caps = re.captures(fen).ok_or("invalid fen")?;

        let mut current_index: u8 = 0;
        for (i, row) in caps
            .get(1)
            .ok_or("expected piece placement")?
            .as_str()
            .split('/')
            .rev()
            .enumerate()
        {
            if i > 7 {
                return Err("too many ranks".to_string());
            }
            for c in row.chars() {
                match c {
                    'R' | 'N' | 'B' | 'Q' | 'K' | 'P' | 'r' | 'n' | 'b' | 'q' | 'k' | 'p' => {
                        let piece = Piece::try_from(c).unwrap();
                        let bb_index = piece as usize;
                        let player = Player::from(piece);
                        let occ_index = player as usize;
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
        }

        b.state.turn =
            Player::try_from(caps.get(2).ok_or("expected active color")?.as_str()).unwrap();

        b.state.castling = caps
            .get(3)
            .ok_or("expected castling rights")?
            .as_str()
            .chars()
            .fold(Castling::empty(), |rights, c| {
                rights | Castling::try_from(c).unwrap()
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

        let (checkers, attacked_squares, pinned_pieces) = b.get_check_info();
        b.state.checkers = checkers;
        b.state.attacked_squares = attacked_squares;
        b.state.pinned_pieces = pinned_pieces;

        if let Some(errors) = b.is_valid() {
            return Err(errors.join("\n"));
        }

        Ok(b)
    }

    // FIXME: proper error handling
    pub fn fen(&self) -> String {
        let mut fen_string = String::new();

        for rank in RANKS.iter().rev() {
            let mut current_offset = 0;
            for file in FILES {
                let square = Square::from((file, *rank));
                let piece = self.get_piece(square);
                match piece {
                    Some(p) => {
                        if current_offset != 0 {
                            fen_string.push(char::from_digit(current_offset, 10).unwrap());
                            current_offset = 0;
                        }
                        fen_string.push_str(&p.to_string());
                    }
                    None => current_offset += 1,
                }
            }

            if current_offset != 0 {
                fen_string.push(char::from_digit(current_offset, 10).unwrap());
            }

            if *rank != Rank::R1 {
                fen_string.push('/')
            }
        }

        fen_string.push(' ');
        fen_string.push_str(&self.state.turn.to_string());

        fen_string.push(' ');
        fen_string.push_str(&self.state.castling.to_string());

        match self.state.en_passant {
            Some(s) => {
                fen_string.push(' ');
                fen_string.push_str(&s.to_string());
            }
            None => fen_string.push_str(" -"),
        }

        fen_string.push(' ');
        fen_string.push_str(self.state.half_moves.to_string().as_str());

        fen_string.push(' ');
        fen_string.push_str(self.state.full_moves.to_string().as_str());

        fen_string
    }

    pub fn start_pos(l: &'a lookup_tables::LookupTables) -> Self {
        Board::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            l,
        )
        .unwrap()
    }

    pub fn shallow_clone(&self) -> Board {
        Board {
            state: self.state,
            previous_states: Vec::new(),
            lookup_tables: self.lookup_tables,
        }
    }

    pub fn generate_moves(&self, legality: Legality) -> Vec<Move> {
        if self.state.checkers.count() > 0 {
            self.generate_evasions(legality)
        } else {
            self.generate_non_evasions(legality)
        }
    }

    pub fn pieces(&self) -> Vec<(Piece, Square)> {
        PIECES.iter().fold(Vec::new(), |mut acc, p| {
            for s in self.state.piece_bbs[*p as usize] {
                acc.push((*p, s));
            }

            acc
        })
    }

    fn generate_evasions(&self, legality: Legality) -> Vec<Move> {
        let mut moves = Vec::with_capacity(MAX_MOVES);

        let king_piece = match self.state.turn {
            Player::White => Piece::WhiteKing,
            Player::Black => Piece::BlackKing,
        };

        // Generate King moves
        self.generate_king_moves(
            king_piece,
            legality,
            &mut moves,
            !self.state.occ_bbs[self.state.turn as usize],
        );

        if self.state.checkers.count() == 1 {
            let pieces = match self.state.turn {
                Player::White => [
                    Piece::WhiteRook,
                    Piece::WhiteKnight,
                    Piece::WhiteBishop,
                    Piece::WhiteQueen,
                    Piece::WhitePawn,
                ],
                Player::Black => [
                    Piece::BlackRook,
                    Piece::BlackKnight,
                    Piece::BlackBishop,
                    Piece::BlackQueen,
                    Piece::BlackPawn,
                ],
            };

            let checking_piece_kind = PieceKind::from(
                self.get_piece(self.state.checkers.get_lsb().unwrap())
                    .unwrap(),
            );
            let move_mask = if checking_piece_kind == PieceKind::Queen
                || checking_piece_kind == PieceKind::Rook
                || checking_piece_kind == PieceKind::Bishop
            {
                let mut move_mask = self.lookup_tables.lookup_between_squares(
                    self.state.checkers.get_lsb().unwrap(),
                    self.state.piece_bbs[king_piece as usize].get_lsb().unwrap(),
                );
                move_mask |= self.state.checkers;
                move_mask
            } else {
                self.state.checkers
            };

            self.generate_pieces_moves(&pieces, legality, &mut moves, move_mask);
        }

        moves
    }

    fn generate_non_evasions(&self, legality: Legality) -> Vec<Move> {
        let mut moves = Vec::with_capacity(MAX_MOVES);

        let pieces = match self.state.turn {
            Player::White => WHITE_PIECES,
            Player::Black => BLACK_PIECES,
        };

        let move_mask = !self.state.occ_bbs[self.state.turn as usize];
        self.generate_pieces_moves(&pieces, legality, &mut moves, move_mask);

        moves
    }

    fn generate_pieces_moves(
        &self,
        pieces: &[Piece],
        legality: Legality,
        moves: &mut Vec<Move>,
        move_mask: BitBoard,
    ) {
        let enemy_player = match self.state.turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        let king_piece = match self.state.turn {
            Player::White => Piece::WhiteKing,
            Player::Black => Piece::BlackKing,
        };
        let king_sq = self.state.piece_bbs[king_piece as usize].get_lsb().unwrap();

        for p in pieces {
            let piece_kind = PieceKind::from(*p);

            match piece_kind {
                PieceKind::Queen | PieceKind::Rook | PieceKind::Bishop | PieceKind::Knight => {
                    for from in self.state.piece_bbs[*p as usize] {
                        let mut inner_move_mask = move_mask;
                        if self.state.pinned_pieces.get_bit(from) && legality == Legality::Legal {
                            inner_move_mask &= self.lookup_tables.lookup_line(from, king_sq);
                        }

                        let mut moves_bb =
                            self.lookup_tables
                                .lookup_moves(*p, from, self.state.occ_bbs[2]);

                        moves_bb &= inner_move_mask;

                        for to in moves_bb {
                            moves.push(Move(from, to, None));
                        }
                    }
                }
                PieceKind::King => {
                    self.generate_king_moves(*p, legality, moves, move_mask);
                }
                PieceKind::Pawn => {
                    let mut capture_move_mask = self.state.occ_bbs[enemy_player as usize];
                    capture_move_mask &= move_mask;
                    if let Some(sq) = self.state.en_passant {
                        capture_move_mask.set_bit(sq);
                    }

                    for from in self.state.piece_bbs[*p as usize] {
                        let mut inner_move_mask = move_mask & !self.state.occ_bbs[2];
                        let mut inner_capture_move_mask = capture_move_mask;
                        if self.state.pinned_pieces.get_bit(from) && legality == Legality::Legal {
                            inner_move_mask &= self.lookup_tables.lookup_line(from, king_sq);
                            inner_capture_move_mask &=
                                self.lookup_tables.lookup_line(from, king_sq);
                        }

                        let mut moves_bb =
                            self.lookup_tables
                                .lookup_moves(*p, from, self.state.occ_bbs[2]);
                        moves_bb &= inner_move_mask;

                        moves_bb |= self.lookup_tables.lookup_capture_moves(*p, from)
                            & inner_capture_move_mask;

                        for to in moves_bb {
                            let move_rank = Rank::from(to);
                            if Some(to) == self.state.en_passant && legality == Legality::Legal {
                                let mut new_all_occ = self.state.occ_bbs[2];
                                new_all_occ.unset_bit(from);
                                if move_rank == Rank::R3 {
                                    new_all_occ.unset_bit(Square::from((File::from(to), Rank::R4)));
                                } else {
                                    new_all_occ.unset_bit(Square::from((File::from(to), Rank::R5)));
                                }
                                new_all_occ.set_bit(to);
                                let mut attacks = BitBoard::new();
                                let pieces = match self.state.turn {
                                    Player::White => {
                                        [Piece::BlackRook, Piece::BlackBishop, Piece::BlackQueen]
                                    }
                                    Player::Black => {
                                        [Piece::WhiteRook, Piece::WhiteBishop, Piece::WhiteQueen]
                                    }
                                };
                                for p in pieces {
                                    for s in self.state.piece_bbs[p as usize] {
                                        attacks |=
                                            self.lookup_tables.lookup_moves(p, s, new_all_occ);
                                    }
                                }

                                if (attacks & self.state.piece_bbs[king_piece as usize]).is_empty()
                                {
                                    moves.push(Move(from, to, None));
                                }
                            } else if move_rank == Rank::R1 || move_rank == Rank::R8 {
                                for pk in [
                                    PieceKind::Queen,
                                    PieceKind::Rook,
                                    PieceKind::Bishop,
                                    PieceKind::Knight,
                                ] {
                                    moves.push(Move(from, to, Some(pk)));
                                }
                            } else {
                                moves.push(Move(from, to, None));
                            }
                        }
                    }
                }
            };
        }
    }

    fn generate_king_moves(
        &self,
        p: Piece,
        legality: Legality,
        moves: &mut Vec<Move>,
        move_mask: BitBoard,
    ) {
        let mut move_mask = move_mask;
        if legality == Legality::Legal {
            move_mask &= !self.state.attacked_squares;
        }

        let from = self.state.piece_bbs[p as usize].get_lsb().unwrap();

        let mut moves_bb = self
            .lookup_tables
            .lookup_moves(p, from, self.state.occ_bbs[2]);
        moves_bb &= move_mask;

        // FIXME: Can I avoid this match or clean up the code?
        let castling_infos = match self.state.turn {
            Player::White => [
                (
                    Castling::WHITE_Q,
                    (Square::A1, Square::E1),
                    (Square::B1, Square::F1),
                    Square::C1,
                ),
                (
                    Castling::WHITE_K,
                    (Square::H1, Square::E1),
                    (Square::H1, Square::D1),
                    Square::G1,
                ),
            ],
            Player::Black => [
                (
                    Castling::BLACK_Q,
                    (Square::A8, Square::E8),
                    (Square::B8, Square::F8),
                    Square::C8,
                ),
                (
                    Castling::BLACK_K,
                    (Square::H8, Square::E8),
                    (Square::H8, Square::D8),
                    Square::G8,
                ),
            ],
        };

        for info in castling_infos {
            if self.state.castling.contains(info.0) {
                let between_bb = self
                    .lookup_tables
                    .lookup_between_squares(info.1 .0, info.1 .1);
                let check_between_bb = self
                    .lookup_tables
                    .lookup_between_squares(info.2 .0, info.2 .1);
                if (between_bb.0 & self.state.occ_bbs[2].0) == 0
                    && (check_between_bb.0 & self.state.attacked_squares.0) == 0
                {
                    moves_bb.set_bit(info.3);
                }
            }
        }

        for to in moves_bb {
            moves.push(Move(from, to, None));
        }
    }

    pub fn apply_move(&mut self, m: Move) {
        self.previous_states.push(self.state);

        let moved_piece = self.get_piece(m.0).unwrap();
        let captured_piece = self.get_piece(m.1);
        if let Some(p) = captured_piece {
            self.state.piece_bbs[p as usize].unset_bit(m.1);
            let captured_color = match self.state.turn {
                Player::White => Player::Black,
                Player::Black => Player::White,
            };
            self.state.occ_bbs[captured_color as usize].unset_bit(m.1);
            self.state.occ_bbs[2].unset_bit(m.1);

            if !self.state.castling.is_empty() {
                if p == Piece::WhiteRook {
                    match m.1 {
                        Square::A1 => self.state.castling.remove(Castling::WHITE_Q),
                        Square::H1 => self.state.castling.remove(Castling::WHITE_K),
                        _ => (),
                    };
                } else if p == Piece::BlackRook {
                    match m.1 {
                        Square::A8 => self.state.castling.remove(Castling::BLACK_Q),
                        Square::H8 => self.state.castling.remove(Castling::BLACK_K),
                        _ => (),
                    };
                }
            }
        }

        if Some(m.1) == self.state.en_passant && PieceKind::from(moved_piece) == PieceKind::Pawn {
            let en_passant_square = self.state.en_passant.unwrap();
            let en_passant_rank = Rank::from(en_passant_square);
            if en_passant_rank == Rank::R3 {
                let capture_square = Square::from((File::from(en_passant_square), Rank::R4));
                self.state.piece_bbs[Piece::WhitePawn as usize].unset_bit(capture_square);
                self.state.occ_bbs[Player::White as usize].unset_bit(capture_square);
                self.state.occ_bbs[2].unset_bit(capture_square);
            } else if en_passant_rank == Rank::R6 {
                let capture_square = Square::from((File::from(en_passant_square), Rank::R5));
                self.state.piece_bbs[Piece::BlackPawn as usize].unset_bit(capture_square);
                self.state.occ_bbs[Player::Black as usize].unset_bit(capture_square);
                self.state.occ_bbs[2].unset_bit(capture_square);
            }
        }

        self.state.piece_bbs[moved_piece as usize].unset_bit(m.0);
        if let Some(promotion_piece_kind) = m.2 {
            let promotion_piece = match self.state.turn {
                Player::White => match promotion_piece_kind {
                    PieceKind::Rook => Piece::WhiteRook,
                    PieceKind::Knight => Piece::WhiteKnight,
                    PieceKind::Bishop => Piece::WhiteBishop,
                    PieceKind::Queen => Piece::WhiteQueen,
                    PieceKind::King => Piece::WhiteKing,
                    PieceKind::Pawn => Piece::WhitePawn,
                },
                Player::Black => match promotion_piece_kind {
                    PieceKind::Rook => Piece::BlackRook,
                    PieceKind::Knight => Piece::BlackKnight,
                    PieceKind::Bishop => Piece::BlackBishop,
                    PieceKind::Queen => Piece::BlackQueen,
                    PieceKind::King => Piece::BlackKing,
                    PieceKind::Pawn => Piece::BlackPawn,
                },
            };

            self.state.piece_bbs[promotion_piece as usize].set_bit(m.1);
        } else {
            self.state.piece_bbs[moved_piece as usize].set_bit(m.1);
        }
        self.state.occ_bbs[self.state.turn as usize].unset_bit(m.0);
        self.state.occ_bbs[self.state.turn as usize].set_bit(m.1);
        self.state.occ_bbs[2].unset_bit(m.0);
        self.state.occ_bbs[2].set_bit(m.1);

        if m.0 == Square::E1 && m.1 == Square::G1 && moved_piece == Piece::WhiteKing {
            self.state.piece_bbs[Piece::WhiteRook as usize].unset_bit(Square::H1);
            self.state.piece_bbs[Piece::WhiteRook as usize].set_bit(Square::F1);

            self.state.occ_bbs[self.state.turn as usize].unset_bit(Square::H1);
            self.state.occ_bbs[self.state.turn as usize].set_bit(Square::F1);

            self.state.occ_bbs[2].unset_bit(Square::H1);
            self.state.occ_bbs[2].set_bit(Square::F1);
            self.state.castling.remove(Castling::WHITE_K);
            self.state.castling.remove(Castling::WHITE_Q);
        } else if m.0 == Square::E1 && m.1 == Square::C1 && moved_piece == Piece::WhiteKing {
            self.state.piece_bbs[Piece::WhiteRook as usize].unset_bit(Square::A1);
            self.state.piece_bbs[Piece::WhiteRook as usize].set_bit(Square::D1);

            self.state.occ_bbs[self.state.turn as usize].unset_bit(Square::A1);
            self.state.occ_bbs[self.state.turn as usize].set_bit(Square::D1);

            self.state.occ_bbs[2].unset_bit(Square::A1);
            self.state.occ_bbs[2].set_bit(Square::D1);
            self.state.castling.remove(Castling::WHITE_K);
            self.state.castling.remove(Castling::WHITE_Q);
        } else if m.0 == Square::E8 && m.1 == Square::G8 && moved_piece == Piece::BlackKing {
            self.state.piece_bbs[Piece::BlackRook as usize].unset_bit(Square::H8);
            self.state.piece_bbs[Piece::BlackRook as usize].set_bit(Square::F8);

            self.state.occ_bbs[self.state.turn as usize].unset_bit(Square::H8);
            self.state.occ_bbs[self.state.turn as usize].set_bit(Square::F8);

            self.state.occ_bbs[2].unset_bit(Square::H8);
            self.state.occ_bbs[2].set_bit(Square::F8);
            self.state.castling.remove(Castling::BLACK_K);
            self.state.castling.remove(Castling::BLACK_Q);
        } else if m.0 == Square::E8 && m.1 == Square::C8 && moved_piece == Piece::BlackKing {
            self.state.piece_bbs[Piece::BlackRook as usize].unset_bit(Square::A8);
            self.state.piece_bbs[Piece::BlackRook as usize].set_bit(Square::D8);

            self.state.occ_bbs[self.state.turn as usize].unset_bit(Square::A8);
            self.state.occ_bbs[self.state.turn as usize].set_bit(Square::D8);

            self.state.occ_bbs[2].unset_bit(Square::A8);
            self.state.occ_bbs[2].set_bit(Square::D8);
            self.state.castling.remove(Castling::BLACK_K);
            self.state.castling.remove(Castling::BLACK_Q);
        }

        self.state.turn = match self.state.turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        let from_file = File::from(m.0);
        let from_rank = Rank::from(m.0);
        let to_file = File::from(m.1);
        let to_rank = Rank::from(m.1);
        if from_file == to_file && PieceKind::from(moved_piece) == PieceKind::Pawn {
            if from_rank == Rank::R2 && to_rank == Rank::R4 {
                self.state.en_passant = Some(Square::from((from_file, Rank::R3)));
            } else if from_rank == Rank::R7 && to_rank == Rank::R5 {
                self.state.en_passant = Some(Square::from((from_file, Rank::R6)));
            } else {
                self.state.en_passant = None;
            }
        } else {
            self.state.en_passant = None;
        }

        if !self.state.castling.is_empty() {
            if moved_piece == Piece::WhiteKing {
                self.state.castling.remove(Castling::WHITE_K);
                self.state.castling.remove(Castling::WHITE_Q);
            } else if moved_piece == Piece::WhiteRook {
                if m.0 == Square::H1 {
                    self.state.castling.remove(Castling::WHITE_K);
                } else if m.0 == Square::A1 {
                    self.state.castling.remove(Castling::WHITE_Q);
                }
            } else if moved_piece == Piece::BlackKing {
                self.state.castling.remove(Castling::BLACK_K);
                self.state.castling.remove(Castling::BLACK_Q);
            } else if moved_piece == Piece::BlackRook {
                if m.0 == Square::H8 {
                    self.state.castling.remove(Castling::BLACK_K);
                } else if m.0 == Square::A8 {
                    self.state.castling.remove(Castling::BLACK_Q);
                }
            }
        }

        let (checkers, attacked_squares, pinned_pieces) = self.get_check_info();
        self.state.checkers = checkers;
        self.state.attacked_squares = attacked_squares;
        self.state.pinned_pieces = pinned_pieces;
    }

    pub fn undo_move(&mut self) {
        self.state = self.previous_states.pop().unwrap();
    }

    fn get_check_info(&self) -> (BitBoard, BitBoard, BitBoard) {
        self.get_check_info_for_player(self.state.turn)
    }

    fn get_check_info_for_player(&self, player: Player) -> (BitBoard, BitBoard, BitBoard) {
        let mut checkers = BitBoard::new();
        let mut attacked_squares = BitBoard::new();
        let mut pinned_pieces = BitBoard::new();

        let king_piece = match player {
            Player::White => Piece::WhiteKing,
            Player::Black => Piece::BlackKing,
        };
        let king_bb = self.state.piece_bbs[king_piece as usize];
        let king_square = king_bb.get_lsb().unwrap();
        let to_file = File::from(king_square);
        let to_rank = Rank::from(king_square);

        let mut all_occ = self.state.occ_bbs[2];
        all_occ.unset_bit(king_square);

        for p in match player {
            Player::White => BLACK_PIECES,
            Player::Black => WHITE_PIECES,
        }
        .iter()
        {
            let piece_kind = PieceKind::from(*p);

            match piece_kind {
                PieceKind::Queen | PieceKind::Rook | PieceKind::Bishop => {
                    for from in self.state.piece_bbs[*p as usize] {
                        let moves = self.lookup_tables.lookup_moves(*p, from, all_occ);
                        attacked_squares |= moves;
                        if !(moves & king_bb).is_empty() {
                            checkers.set_bit(from);
                        }

                        let from_file = File::from(from);
                        let from_rank = Rank::from(from);
                        let same_file = from_file == to_file;
                        let same_rank = from_rank == to_rank;
                        let same_diag = (from_file as i8 - to_file as i8).abs()
                            == (from_rank as i8 - to_rank as i8).abs();

                        if (piece_kind == PieceKind::Bishop && same_diag)
                            || (piece_kind == PieceKind::Rook && (same_file || same_rank))
                            || (piece_kind == PieceKind::Queen
                                && (same_file || same_rank || same_diag))
                        {
                            let ray_bb =
                                self.lookup_tables.lookup_between_squares(from, king_square);

                            if (ray_bb & self.state.occ_bbs[2]).pop_count() == 1
                                && !(ray_bb & self.state.occ_bbs[player as usize]).is_empty()
                            {
                                pinned_pieces.0 |= ray_bb.0 & self.state.occ_bbs[player as usize].0;
                            }
                        }
                    }
                }
                PieceKind::Knight => {
                    for from in self.state.piece_bbs[*p as usize] {
                        let moves = self.lookup_tables.lookup_moves(*p, from, all_occ);
                        attacked_squares |= moves;
                        if !(moves & king_bb).is_empty() {
                            checkers.set_bit(from);
                        }
                    }
                }
                PieceKind::King => {
                    for from in self.state.piece_bbs[*p as usize] {
                        let moves = self.lookup_tables.lookup_moves(*p, from, all_occ);
                        attacked_squares |= moves;
                    }
                }
                PieceKind::Pawn => {
                    for from in self.state.piece_bbs[*p as usize] {
                        let moves = self.lookup_tables.lookup_capture_moves(*p, from);
                        attacked_squares |= moves;
                        if !(moves & king_bb).is_empty() {
                            checkers.set_bit(from);
                        }
                    }
                }
            };
        }

        (checkers, attacked_squares, pinned_pieces)
    }

    fn get_piece(&self, s: Square) -> Option<Piece> {
        PIECES
            .into_iter()
            .find(|p| self.state.piece_bbs[*p as usize].get_bit(s))
    }

    fn count_piece(&self, p: Piece) -> u32 {
        self.state.piece_bbs[p as usize].pop_count()
    }

    // FIXME: Is it strange that this is an option? Is just the vector enough cause it will be
    // empty if it's fine?
    pub fn is_valid(&self) -> Option<Vec<String>> {
        let mut errors = Vec::new();

        // Piece count validation
        if self.count_piece(Piece::WhiteKing) != 1 {
            errors.push(format!(
                "incorrect number of white kings: {}",
                self.count_piece(Piece::WhiteKing)
            ));
        }

        if self.count_piece(Piece::BlackKing) != 1 {
            errors.push(format!(
                "incorrect number of black kings: {}",
                self.count_piece(Piece::BlackKing)
            ));
        }

        if self.count_piece(Piece::WhitePawn) > 8 {
            errors.push(format!(
                "incorrect number of white pawns: {}",
                self.count_piece(Piece::WhitePawn)
            ));
        }

        if self.count_piece(Piece::BlackPawn) > 8 {
            errors.push(format!(
                "incorrect number of black pawns: {}",
                self.count_piece(Piece::BlackPawn)
            ));
        }

        let num_white_pieces = WHITE_PIECES
            .iter()
            .fold(0, |acc, p| acc + self.count_piece(*p));
        if num_white_pieces > 16 {
            errors.push(format!(
                "incorrect number of white pieces: {}",
                num_white_pieces,
            ));
        }

        let num_black_pieces = BLACK_PIECES
            .iter()
            .fold(0, |acc, p| acc + self.count_piece(*p));
        if num_black_pieces > 16 {
            errors.push(format!(
                "incorrect number of black pieces: {}",
                num_black_pieces,
            ));
        }

        // Internal state validation
        if self.state.occ_bbs[Player::White as usize]
            != WHITE_PIECES.iter().fold(BitBoard::new(), |bb, p| {
                bb | self.state.piece_bbs[*p as usize]
            })
        {
            errors.push("white occupancy bitboard doesn't match piece bitboards".to_string());
        }

        if self.state.occ_bbs[Player::Black as usize]
            != BLACK_PIECES.iter().fold(BitBoard::new(), |bb, p| {
                bb | self.state.piece_bbs[*p as usize]
            })
        {
            errors.push("black occupancy bitboard doesn't match piece bitboards".to_string());
        }

        if self.state.occ_bbs[2] != (self.state.occ_bbs[0] | self.state.occ_bbs[1]) {
            errors.push(
                "all occupancy bitboard doesn't match player occupancy bitboards".to_string(),
            );
        }

        let (checkers, attacked_squares, pinned_pieces) = self.get_check_info();

        if self.state.checkers != checkers {
            errors.push("checkers bitboard is not correct".to_string());
        }

        if self.state.attacked_squares != attacked_squares {
            errors.push("attacked squares bitboard is not correct".to_string());
        }

        if self.state.pinned_pieces != pinned_pieces {
            errors.push("pinned pieces bitboard is not correct".to_string());
        }

        for p1 in PIECES {
            for p2 in PIECES {
                if p1 != p2
                    && !(self.state.piece_bbs[p1 as usize] & self.state.piece_bbs[p2 as usize])
                        .is_empty()
                {
                    errors.push(format!("{} and {} are on the same square", p1, p2));
                }
            }
        }

        // Board Validation
        let white_king_square = self.state.piece_bbs[Piece::WhiteKing as usize]
            .get_lsb()
            .unwrap();
        let white_king_rank = Rank::from(white_king_square) as i8;
        let white_king_file = File::from(white_king_square) as i8;

        let black_king_square = self.state.piece_bbs[Piece::BlackKing as usize]
            .get_lsb()
            .unwrap();
        let black_king_rank = Rank::from(black_king_square) as i8;
        let black_king_file = File::from(black_king_square) as i8;

        let king_distance = cmp::max(
            (white_king_file - black_king_file).abs(),
            (white_king_rank - black_king_rank).abs(),
        );
        if king_distance < 2 {
            errors.push("Kings are too close together".to_string());
        }

        let num_checks = self.state.checkers.pop_count();
        if num_checks > 2 {
            errors.push(format!(
                "Only 2 checkers are possible. {} found.",
                num_checks
            ));
        }

        if num_checks == 2 {
            let piece_1 = PieceKind::from(
                self.get_piece(self.state.checkers.get_lsb().unwrap())
                    .unwrap(),
            );
            let piece_2 = PieceKind::from(
                self.get_piece(self.state.checkers.get_msb().unwrap())
                    .unwrap(),
            );

            // FIXME: Can I make this if nicer?
            if (piece_1 == PieceKind::Pawn
                && (piece_2 == PieceKind::Pawn
                    || piece_2 == PieceKind::Bishop
                    || piece_2 == PieceKind::Knight))
                || (piece_1 == PieceKind::Bishop
                    && (piece_2 == PieceKind::Bishop || piece_2 == PieceKind::Pawn)
                    || piece_1 == PieceKind::Knight
                        && (piece_2 == PieceKind::Knight || piece_2 == PieceKind::Pawn))
            {
                errors.push(format!(
                    "cannot double check with a {:?} and a {:?}",
                    piece_1, piece_2
                ));
            }
        }

        let non_active_king = match self.state.turn {
            Player::White => Piece::BlackKing,
            Player::Black => Piece::WhiteKing,
        };
        let (_, attacked_square, _) = self.get_check_info_for_player(match self.state.turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        });
        if !(attacked_square & self.state.piece_bbs[non_active_king as usize]).is_empty() {
            errors.push("non active color is in check".to_string());
        }

        if !(self.state.piece_bbs[Piece::WhitePawn as usize] & RANK_1).is_empty() {
            errors.push("cannot have white pawns on rank 1".to_string())
        }

        if !(self.state.piece_bbs[Piece::WhitePawn as usize] & RANK_8).is_empty() {
            errors.push("cannot have white pawns on rank 8".to_string())
        }

        if !(self.state.piece_bbs[Piece::BlackPawn as usize] & RANK_1).is_empty() {
            errors.push("cannot have black pawns on rank 1".to_string())
        }

        if !(self.state.piece_bbs[Piece::BlackPawn as usize] & RANK_8).is_empty() {
            errors.push("cannot have black pawns on rank 8".to_string())
        }

        if let Some(en_passant_square) = self.state.en_passant {
            let en_passant_file = File::from(en_passant_square);
            let en_passant_rank = Rank::from(en_passant_square);
            match en_passant_rank {
                Rank::R3 => {
                    if self
                        .get_piece(Square::from((en_passant_file, Rank::R3)))
                        .is_some()
                        || self
                            .get_piece(Square::from((en_passant_file, Rank::R2)))
                            .is_some()
                        || self.get_piece(Square::from((en_passant_file, Rank::R4)))
                            != Some(Piece::WhitePawn)
                    {
                        errors.push("invalid en passant square".to_string())
                    }
                }
                Rank::R6 => {
                    if self
                        .get_piece(Square::from((en_passant_file, Rank::R6)))
                        .is_some()
                        || self
                            .get_piece(Square::from((en_passant_file, Rank::R7)))
                            .is_some()
                        || self.get_piece(Square::from((en_passant_file, Rank::R5)))
                            != Some(Piece::BlackPawn)
                    {
                        errors.push("invalid en passant square".to_string())
                    }
                }
                _ => errors.push(format!("invalid en passant rank: {}", en_passant_rank)),
            }
        }

        let num_extra_white_pieces = cmp::max(0, self.count_piece(Piece::WhiteQueen) as i8 - 1)
            + cmp::max(0, self.count_piece(Piece::WhiteRook) as i8 - 2)
            + cmp::max(0, self.count_piece(Piece::WhiteKnight) as i8 - 2)
            + cmp::max(0, self.count_piece(Piece::WhiteBishop) as i8 - 2);
        let missing_white_pawns = 8 - self.count_piece(Piece::WhitePawn) as i8;
        if num_extra_white_pieces > missing_white_pawns {
            errors.push("too many promoted white pieces".to_string())
        }

        let num_extra_black_pieces = cmp::max(0, self.count_piece(Piece::BlackQueen) as i8 - 1)
            + cmp::max(0, self.count_piece(Piece::BlackRook) as i8 - 2)
            + cmp::max(0, self.count_piece(Piece::BlackKnight) as i8 - 2)
            + cmp::max(0, self.count_piece(Piece::BlackBishop) as i8 - 2);
        let missing_black_pawns = 8 - self.count_piece(Piece::BlackPawn) as i8;
        if num_extra_black_pieces > missing_black_pawns {
            errors.push("too many promoted black pieces".to_string())
        }

        if self.get_piece(Square::E1) != Some(Piece::WhiteKing) {
            if self.state.castling.contains(Castling::WHITE_Q) {
                errors.push("white shouldn't have queenside castling rights".to_string())
            }
            if self.state.castling.contains(Castling::WHITE_K) {
                errors.push("white shouldn't have kingside castling rights".to_string())
            }
        } else {
            if self.get_piece(Square::A1) != Some(Piece::WhiteRook)
                && self.state.castling.contains(Castling::WHITE_Q)
            {
                errors.push("white shouldn't have queenside castling rights".to_string())
            }

            if self.get_piece(Square::H1) != Some(Piece::WhiteRook)
                && self.state.castling.contains(Castling::WHITE_K)
            {
                errors.push("white shouldn't have kingside castling rights".to_string())
            }
        }

        if self.get_piece(Square::E8) != Some(Piece::BlackKing) {
            if self.state.castling.contains(Castling::BLACK_Q) {
                errors.push("black shouldn't have queenside castling rights".to_string())
            }
            if self.state.castling.contains(Castling::BLACK_K) {
                errors.push("black shouldn't have kingside castling rights".to_string())
            }
        } else {
            if self.get_piece(Square::A8) != Some(Piece::BlackRook)
                && self.state.castling.contains(Castling::BLACK_Q)
            {
                errors.push("black shouldn't have queenside castling rights".to_string())
            }

            if self.get_piece(Square::H8) != Some(Piece::BlackRook)
                && self.state.castling.contains(Castling::BLACK_K)
            {
                errors.push("black shouldn't have kingside castling rights".to_string())
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push('\n');
        for rank in RANKS.iter().rev() {
            s.push_str(&format!("{}    ", rank));
            for file in FILES {
                let square = Square::from((file, *rank));
                let piece = self.get_piece(square);
                let piece_repr = match piece {
                    Some(p) => p.to_string(),
                    None => ".".to_string(),
                };
                s.push_str(&format!(" {} ", piece_repr));
            }
            s.push('\n');
        }
        s.push('\n');
        s.push_str("      a  b  c  d  e  f  g  h");
        s.push('\n');

        f.pad(&s)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard,
        board::{Board, BoardState, Castling},
        core::{
            Player, Square, EN_PASSANT_FEN, IN_CHECK_FEN, POS_2_KIWIPETE_FEN, POS_3_FEN, POS_5_FEN,
            POS_6_FEN, STARTING_POS_FEN,
        },
        lookup_tables::LookupTables,
    };

    const STARTING_BOARD_STATE: BoardState = BoardState {
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
        castling: Castling::all(),
        en_passant: None,
        half_moves: 0,
        full_moves: 1,
        checkers: bitboard::BitBoard(0),
        attacked_squares: bitboard::BitBoard(9151313343305220096),
        pinned_pieces: bitboard::BitBoard(0),
    };

    const EN_PASSANT_BOARD_STATE: BoardState = BoardState {
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
        castling: Castling::all(),
        en_passant: Some(Square::D6),
        half_moves: 0,
        full_moves: 3,
        checkers: bitboard::BitBoard(0),
        attacked_squares: bitboard::BitBoard(9151313525111521280),
        pinned_pieces: bitboard::BitBoard(0),
    };

    const POS_2_KIWIPETE_BOARD_STATE: BoardState = BoardState {
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
        castling: Castling::all(),
        en_passant: None,
        half_moves: 0,
        full_moves: 0,
        checkers: bitboard::BitBoard(0),
        attacked_squares: bitboard::BitBoard(18427602327210643456),
        pinned_pieces: bitboard::BitBoard(0),
    };

    const POS_3_BOARD_STATE: BoardState = BoardState {
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
        castling: Castling::empty(),
        en_passant: None,
        half_moves: 0,
        full_moves: 0,
        checkers: bitboard::BitBoard(0),
        attacked_squares: bitboard::BitBoard(9259553660634923008),
        pinned_pieces: bitboard::BitBoard(8589934592),
    };

    const POS_5_BOARD_STATE: BoardState = BoardState {
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
        castling: Castling::from_bits_truncate(Castling::WHITE_K.bits() | Castling::WHITE_Q.bits()),
        en_passant: None,
        half_moves: 1,
        full_moves: 8,
        checkers: bitboard::BitBoard(0),
        attacked_squares: bitboard::BitBoard(9151313686139830408),
        pinned_pieces: bitboard::BitBoard(0),
    };

    const POS_6_BOARD_STATE: BoardState = BoardState {
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
        castling: Castling::empty(),
        en_passant: None,
        half_moves: 0,
        full_moves: 10,
        checkers: bitboard::BitBoard(0),
        attacked_squares: bitboard::BitBoard(18446180846641684480),
        pinned_pieces: bitboard::BitBoard(8192),
    };

    const IN_CHECK_BOARD_STATE: BoardState = BoardState {
        turn: Player::White,
        piece_bbs: [
            bitboard::BitBoard(129),
            bitboard::BitBoard(2097154),
            bitboard::BitBoard(36),
            bitboard::BitBoard(8),
            bitboard::BitBoard(16),
            bitboard::BitBoard(134280960),
            bitboard::BitBoard(9295429630892703744),
            bitboard::BitBoard(4755801206503243776),
            bitboard::BitBoard(288230376185266176),
            bitboard::BitBoard(576460752303423488),
            bitboard::BitBoard(1152921504606846976),
            bitboard::BitBoard(67290111619891200),
        ],
        occ_bbs: [
            bitboard::BitBoard(136378303),
            bitboard::BitBoard(16136133582111375360),
            bitboard::BitBoard(16136133582247753663),
        ],
        castling: Castling::all(),
        en_passant: None,
        half_moves: 2,
        full_moves: 3,
        checkers: bitboard::BitBoard(33554432),
        attacked_squares: bitboard::BitBoard(9133299415094986768),
        pinned_pieces: bitboard::BitBoard(0),
    };

    #[test]
    fn from_fen_test() {
        struct TestCase {
            name: &'static str,
            fen: &'static str,
            expected_state: BoardState,
        }
        let l = LookupTables::generate();

        let test_cases = vec![
            TestCase {
                name: "starting position",
                fen: STARTING_POS_FEN,
                expected_state: STARTING_BOARD_STATE,
            },
            TestCase {
                name: "en passant square",
                fen: EN_PASSANT_FEN,
                expected_state: EN_PASSANT_BOARD_STATE,
            },
            TestCase {
                name: "kiwipete",
                fen: POS_2_KIWIPETE_FEN,
                expected_state: POS_2_KIWIPETE_BOARD_STATE,
            },
            TestCase {
                name: "position 3",
                fen: POS_3_FEN,
                expected_state: POS_3_BOARD_STATE,
            },
            TestCase {
                name: "position 5",
                fen: POS_5_FEN,
                expected_state: POS_5_BOARD_STATE,
            },
            TestCase {
                name: "position 6",
                fen: POS_6_FEN,
                expected_state: POS_6_BOARD_STATE,
            },
            TestCase {
                name: "in check position",
                fen: IN_CHECK_FEN,
                expected_state: IN_CHECK_BOARD_STATE,
            },
        ];

        for test_case in test_cases {
            let b = Board::from_fen(test_case.fen, &l).unwrap();

            if test_case.name == "position 5" {
                println!("{}\n{}", b, b.state.pinned_pieces);
            }

            assert_eq!(
                b.state, test_case.expected_state,
                "{} failed",
                test_case.name
            );
        }
    }

    #[test]
    fn fen_test() {
        struct TestCase {
            name: &'static str,
            state: BoardState,
            expected_fen: &'static str,
        }
        let l = LookupTables::generate();

        let test_cases = vec![
            TestCase {
                name: "starting position",
                state: STARTING_BOARD_STATE,
                expected_fen: STARTING_POS_FEN,
            },
            TestCase {
                name: "en passant square",
                state: EN_PASSANT_BOARD_STATE,
                expected_fen: EN_PASSANT_FEN,
            },
            TestCase {
                name: "kiwipete",
                state: POS_2_KIWIPETE_BOARD_STATE,
                expected_fen:
                    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0",
            },
            TestCase {
                name: "position 3",
                state: POS_3_BOARD_STATE,
                expected_fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0",
            },
            TestCase {
                name: "position 5",
                state: POS_5_BOARD_STATE,
                expected_fen: POS_5_FEN,
            },
            TestCase {
                name: "position 6",
                state: POS_6_BOARD_STATE,
                expected_fen: POS_6_FEN,
            },
            TestCase {
                name: "in check position",
                state: IN_CHECK_BOARD_STATE,
                expected_fen: IN_CHECK_FEN,
            },
        ];

        for test_case in test_cases {
            let mut b = Board::new(&l);
            b.state = test_case.state;

            assert_eq!(b.fen(), test_case.expected_fen, "{} failed", test_case.name);
        }
    }
}
