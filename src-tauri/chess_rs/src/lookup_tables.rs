use crate::bitboard;
use crate::core;
use crate::core::{
    File, Piece, PieceKind, Player, Rank, Square, NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE,
    PLAYERS, SQUARES,
};

const ROOK_MAGIC_SHIFTS: [(u64, u64); 64] = [
    (36028935531692048, 12),
    (4629701038676910082, 12),
    (5908758445539999750, 12),
    (2310347502396637696, 12),
    (144119594712826644, 12),
    (144115808732185601, 12),
    (9853876534446653761, 12),
    (7241790679695757440, 12),
    (9232388032747900930, 12),
    (576467402045788160, 12),
    (562986473328672, 12),
    (9026990732514362, 12),
    (37579108447944705, 12),
    (70644731740288, 12),
    (2738470050570245154, 12),
    (4612530448744088098, 12),
    (45036133713707040, 12),
    (8071576466787172360, 12),
    (9367856678286000133, 12),
    (17832721281076, 12),
    (1297047687832543237, 12),
    (1237488764992, 12),
    (1161929597231695874, 12),
    (153122547326582800, 12),
    (288301916348227584, 12),
    (35219805833232, 12),
    (40532414128226368, 12),
    (112590336966672388, 12),
    (5206763701881342004, 12),
    (1170980019405344, 12),
    (6921048599392684034, 12),
    (4755801765385879808, 12),
    (306250315572709378, 12),
    (9900168318976, 12),
    (36033232110686233, 12),
    (2252006005800992, 12),
    (72340237279305734, 12),
    (9370869327009416194, 12),
    (4611688492824010817, 12),
    (9227876220601893162, 12),
    (200551200113426448, 12),
    (17767340311040, 12),
    (5769119972710285824, 12),
    (306253572919037952, 12),
    (82472237759791112, 12),
    (5629503862747202, 12),
    (567897765269520, 12),
    (576461860673472516, 12),
    (162131923073762336, 12),
    (1155314110766842112, 12),
    (4940765451916428290, 12),
    (74520534444933380, 12),
    (90142928227664064, 12),
    (577586660816978180, 12),
    (144116323030402048, 12),
    (9020531957301888, 12),
    (282024772636993, 12),
    (4666108557278609441, 12),
    (297800664947360834, 12),
    (1230045682620040258, 12),
    (11538257722994589954, 12),
    (22799481770608641, 12),
    (576777966844772353, 12),
    (13907116203437998114, 12),
];

const BISHOP_MAGIC_SHIFTS: [(u64, u64); 64] = [
    (81981821524378632, 12),
    (36081622432456713, 12),
    (2450106648541286408, 12),
    (144713425614817802, 12),
    (44616120274944, 12),
    (5779106133205024, 12),
    (36065149756641312, 12),
    (378307046052757537, 12),
    (88055219458, 12),
    (9225767323019772048, 12),
    (576478728916385792, 12),
    (153264862142140552, 12),
    (4741660803584, 12),
    (613528016797712, 12),
    (563534069564424, 12),
    (1157513101940695041, 12),
    (4575894664937728, 12),
    (864727157073780760, 12),
    (2526545886675206272, 12),
    (10376860923842266184, 12),
    (2054767437697536000, 12),
    (4612820735902220800, 12),
    (18016601840297000, 12),
    (72831671732308098, 12),
    (2392812219039808, 12),
    (288516801749323792, 12),
    (18015498080093184, 12),
    (585520882751902080, 12),
    (4398066630920, 12),
    (72145976290198016, 12),
    (1766048814700988416, 12),
    (1337573779433930816, 12),
    (2379317876425277472, 12),
    (2305992543132819472, 12),
    (7151717312629178946, 12),
    (613061330132287490, 12),
    (5198350255959049472, 12),
    (74309464723834880, 12),
    (2306195957048541204, 12),
    (360728885089928416, 12),
    (18161537646660, 12),
    (8937833238688, 12),
    (292804482029322310, 12),
    (633336036856928, 12),
    (9259546043228423200, 12),
    (76843287402645536, 12),
    (867022402343700512, 12),
    (149780020160643074, 12),
    (75154024940445696, 12),
    (508977145972830720, 12),
    (2377901170230841472, 12),
    (734114235693941256, 12),
    (4503602379378688, 12),
    (2476982003741098148, 12),
    (297602666963140753, 12),
    (3553905185261698, 12),
    (45177008784933128, 12),
    (18015534566084872, 12),
    (4611688218593600512, 12),
    (321057797992452, 12),
    (360437813142884624, 12),
    (11260201810272386, 12),
    (211123832361218, 12),
    (184726751170084928, 12),
];

pub struct LookupTables {
    knight_moves_table: [bitboard::BitBoard; 64],
    pawn_captures_table: [[bitboard::BitBoard; 64]; 2],
    pawn_moves_table: [[[bitboard::BitBoard; 64]; 2]; 2],
    pawn_double_mask: [bitboard::BitBoard; 64],
    king_moves_table: [bitboard::BitBoard; 64],
    rook_moves_mask: [bitboard::BitBoard; 64],
    bishop_moves_mask: [bitboard::BitBoard; 64],
    // FIXME: What is the correct type to use here? I'd like to use [[bitboard::BitBoard; 4096]; 64]
    // but that blows up the stack
    rook_moves_table: Vec<Vec<bitboard::BitBoard>>,
    bishop_moves_table: Vec<Vec<bitboard::BitBoard>>,
    between_sqaures_table: [[bitboard::BitBoard; 64]; 64],
    line_table: [[bitboard::BitBoard; 64]; 64],
}

impl std::fmt::Debug for LookupTables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LookupTables")
    }
}

impl LookupTables {
    pub fn generate() -> Self {
        let rook_moves_mask = gen_sliding_moves_mask(true);
        let bishop_moves_mask = gen_sliding_moves_mask(false);
        let rook_moves_table = gen_sliding_moves(&rook_moves_mask, true);
        let bishop_moves_table = gen_sliding_moves(&bishop_moves_mask, false);
        let line_table = gen_lines(&rook_moves_table, &bishop_moves_table);
        Self {
            knight_moves_table: gen_knight_moves(),
            pawn_moves_table: gen_pawn_moves(),
            pawn_captures_table: gen_pawn_capture_moves(),
            pawn_double_mask: gen_pawn_double_masks(),
            king_moves_table: gen_king_moves(),
            rook_moves_mask,
            bishop_moves_mask,
            rook_moves_table,
            bishop_moves_table,
            between_sqaures_table: gen_between_squares(),
            line_table,
        }
    }

    pub fn lookup_moves(
        &self,
        p: Piece,
        s: Square,
        all_occupancy: bitboard::BitBoard,
    ) -> bitboard::BitBoard {
        match PieceKind::from(p) {
            PieceKind::Rook => {
                let blockers_key = all_occupancy & self.rook_moves_mask[s as usize];
                let (magic_number, shifts) = ROOK_MAGIC_SHIFTS[s as usize];
                let k = apply_magic_number(blockers_key, magic_number, shifts);
                self.rook_moves_table[s as usize][k.0 as usize]
            }
            PieceKind::Knight => self.knight_moves_table[s as usize],
            PieceKind::Bishop => {
                let blockers_key = all_occupancy & self.bishop_moves_mask[s as usize];
                let (magic_number, shifts) = BISHOP_MAGIC_SHIFTS[s as usize];
                let k = apply_magic_number(blockers_key, magic_number, shifts);
                self.bishop_moves_table[s as usize][k.0 as usize]
            }
            PieceKind::Queen => {
                let rook_blockers_key = all_occupancy & self.rook_moves_mask[s as usize];
                let (rook_magic_number, rook_shifts) = ROOK_MAGIC_SHIFTS[s as usize];
                let rook_k = apply_magic_number(rook_blockers_key, rook_magic_number, rook_shifts);
                let rook_moves = self.rook_moves_table[s as usize][rook_k.0 as usize];

                let bishop_blockers_key = all_occupancy & self.bishop_moves_mask[s as usize];
                let (bishop_magic_number, bishop_shifts) = BISHOP_MAGIC_SHIFTS[s as usize];
                let k = apply_magic_number(bishop_blockers_key, bishop_magic_number, bishop_shifts);
                let bishop_moves = self.bishop_moves_table[s as usize][k.0 as usize];

                rook_moves | bishop_moves
            }
            PieceKind::King => self.king_moves_table[s as usize],
            PieceKind::Pawn => {
                let player = Player::from(p);
                let pawn_double_mask = self.pawn_double_mask[s as usize];
                let can_double = (pawn_double_mask & all_occupancy).is_empty();

                self.pawn_moves_table[if can_double { 1 } else { 0 }][player as usize][s as usize]
            }
        }
    }

    pub fn lookup_capture_moves(&self, p: Piece, s: Square) -> bitboard::BitBoard {
        match PieceKind::from(p) {
            PieceKind::Pawn => {
                self.pawn_captures_table[if p == Piece::WhitePawn { 0 } else { 1 }][s as usize]
            }
            _ => panic!("lookup_capture_moves is only supported for Pawns"),
        }
    }

    pub fn lookup_between_squares(&self, from: Square, to: Square) -> bitboard::BitBoard {
        self.between_sqaures_table[from as usize][to as usize]
    }

    pub fn lookup_line(&self, from: Square, to: Square) -> bitboard::BitBoard {
        self.line_table[from as usize][to as usize]
    }
}

pub fn apply_magic_number(
    blocker_bitboard: bitboard::BitBoard,
    magic_number: u64,
    shifts: u64,
) -> bitboard::BitBoard {
    blocker_bitboard.wrapping_mul(magic_number) >> (64 - shifts)
}

fn gen_knight_moves() -> [bitboard::BitBoard; 64] {
    let mut moves = Vec::with_capacity(64);
    for s in SQUARES {
        moves.push(gen_knight_move(s as u64))
    }
    moves.try_into().unwrap()
}

fn gen_knight_move(s: u64) -> bitboard::BitBoard {
    let bb = 1 << s;
    let mut moves = 0;
    moves |= (bb << 17) & NOT_A_FILE;
    moves |= (bb << 15) & NOT_H_FILE;
    moves |= (bb << 10) & NOT_AB_FILE;
    moves |= (bb << 6) & NOT_GH_FILE;
    moves |= (bb >> 6) & NOT_AB_FILE;
    moves |= (bb >> 10) & NOT_GH_FILE;
    moves |= (bb >> 15) & NOT_A_FILE;
    moves |= (bb >> 17) & NOT_H_FILE;
    bitboard::BitBoard(moves)
}

fn gen_pawn_moves() -> [[[bitboard::BitBoard; 64]; 2]; 2] {
    let mut moves = [[[bitboard::BitBoard::new(); 64]; 2]; 2];
    for s in SQUARES {
        for can_double in [true, false] {
            for player in PLAYERS {
                moves[if can_double { 1 } else { 0 }][player as usize][s as usize] =
                    gen_pawn_move(s as u64, can_double, player == Player::White);
            }
        }
    }

    moves
}

fn gen_pawn_move(s: u64, can_double: bool, is_white: bool) -> bitboard::BitBoard {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb << 8;

        if can_double && Rank::from(Square::try_from(s as u8).unwrap()) == Rank::R2 {
            moves |= bb << 16;
        }

        bitboard::BitBoard(moves)
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb >> 8;

        if can_double && Rank::from(Square::try_from(s as u8).unwrap()) == Rank::R7 {
            moves |= bb >> 16;
        }

        bitboard::BitBoard(moves)
    }
}

fn gen_pawn_capture_moves() -> [[bitboard::BitBoard; 64]; 2] {
    let mut white_moves = Vec::with_capacity(64);
    let mut black_moves = Vec::with_capacity(64);
    for s in SQUARES {
        white_moves.push(gen_pawn_capture_move(s as u64, true));
        black_moves.push(gen_pawn_capture_move(s as u64, false));
    }
    [
        white_moves.try_into().unwrap(),
        black_moves.try_into().unwrap(),
    ]
}

fn gen_pawn_double_masks() -> [bitboard::BitBoard; 64] {
    let mut masks = [bitboard::BitBoard::new(); 64];

    for s in SQUARES {
        let rank = Rank::from(s);
        if rank == Rank::R2 {
            masks[s as usize].set_bit(Square::from((File::from(s), Rank::R3)));
        } else if rank == Rank::R7 {
            masks[s as usize].set_bit(Square::from((File::from(s), Rank::R6)));
        }
    }

    masks
}

fn gen_pawn_capture_move(s: u64, is_white: bool) -> bitboard::BitBoard {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= (bb << 9) & NOT_A_FILE;
        moves |= (bb << 7) & NOT_H_FILE;
        bitboard::BitBoard(moves)
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= (bb >> 9) & NOT_H_FILE;
        moves |= (bb >> 7) & NOT_A_FILE;
        bitboard::BitBoard(moves)
    }
}

fn gen_king_moves() -> [bitboard::BitBoard; 64] {
    let mut moves = Vec::with_capacity(64);
    for s in SQUARES {
        moves.push(gen_king_move(s as u64))
    }
    moves.try_into().unwrap()
}

fn gen_king_move(s: u64) -> bitboard::BitBoard {
    let bb = 1 << s;
    let mut moves = 0;
    moves |= bb << 8;
    moves |= bb >> 8;
    moves |= (bb >> 9) & NOT_H_FILE;
    moves |= (bb >> 7) & NOT_A_FILE;
    moves |= (bb << 9) & NOT_A_FILE;
    moves |= (bb << 7) & NOT_H_FILE;
    moves |= (bb << 1) & NOT_A_FILE;
    moves |= (bb >> 1) & NOT_H_FILE;

    bitboard::BitBoard(moves)
}

fn gen_sliding_moves(
    move_masks: &[bitboard::BitBoard; 64],
    is_rook: bool,
) -> Vec<Vec<bitboard::BitBoard>> {
    let mut moves = vec![vec![bitboard::BitBoard::new(); 4096]; 64];
    for (s, mask) in move_masks.iter().enumerate() {
        let total_blocker_combs = 2_u64.pow(mask.pop_count());
        for raw_blocker in 0..total_blocker_combs {
            let blocker_bitboard = create_blocker_bitboard(*mask, raw_blocker);

            let (magic_number, shifts) = if is_rook {
                ROOK_MAGIC_SHIFTS[s]
            } else {
                BISHOP_MAGIC_SHIFTS[s]
            };
            let k = apply_magic_number(blocker_bitboard, magic_number, shifts);

            moves[s][k.0 as usize] = gen_sliding_move(s as u8, blocker_bitboard.0, is_rook);
        }
    }
    moves
}

pub fn create_blocker_bitboard(
    move_mask: bitboard::BitBoard,
    raw_blocker: u64,
) -> bitboard::BitBoard {
    let raw_blocker_bitboard = bitboard::BitBoard(raw_blocker);
    let mut blocker_bitboard = bitboard::BitBoard::new();
    let mut blocker_index = 0;
    let mut blocker_mask = move_mask;
    while let Some(mask_index) = blocker_mask.pop_lsb() {
        let blocker_set = raw_blocker_bitboard.get_bit(Square::try_from(blocker_index).unwrap());
        if blocker_set {
            blocker_bitboard.set_bit(mask_index);
        } else {
            blocker_bitboard.unset_bit(mask_index);
        }
        blocker_index += 1
    }

    blocker_bitboard
}

fn gen_sliding_move(s: u8, blockers: u64, is_rook: bool) -> bitboard::BitBoard {
    let directions = if is_rook {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
    } else {
        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
    };

    let square = Square::try_from(s).unwrap();
    let mut moves_bitboard = bitboard::BitBoard::new();
    let blockers_bitboard = bitboard::BitBoard(blockers);
    for dir in directions {
        let mut current_rank: i8 = (Rank::from(square) as i8) + 1;
        let mut current_file: i8 = (File::from(square) as i8) + 1;
        for _ in 1..8 {
            current_rank += dir.1;
            current_file += dir.0;

            if !(1..=8).contains(&current_rank) || !(1..=8).contains(&current_file) {
                break;
            }

            let bit_index =
                Square::try_from(((current_file - 1) + (current_rank - 1) * 8) as u8).unwrap();

            moves_bitboard.set_bit(bit_index);

            if blockers_bitboard.get_bit(bit_index) {
                break;
            }
        }
    }

    moves_bitboard
}

pub fn gen_sliding_moves_mask(is_rook: bool) -> [bitboard::BitBoard; 64] {
    let mut moves = Vec::with_capacity(64);
    for s in SQUARES {
        moves.push(gen_sliding_move_mask(s as u8, is_rook));
    }
    moves.try_into().unwrap()
}

fn gen_sliding_move_mask(s: u8, is_rook: bool) -> bitboard::BitBoard {
    let directions = if is_rook {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
    } else {
        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
    };

    let square = Square::try_from(s).unwrap();
    let mut moves_bitboard = bitboard::BitBoard::new();

    for dir in directions {
        let mut current_rank: i8 = (Rank::from(square) as i8) + 1;
        let mut current_file: i8 = (File::from(square) as i8) + 1;
        for _ in 1..8 {
            current_rank += dir.1;
            current_file += dir.0;

            if !(1..=8).contains(&current_rank) || !(1..=8).contains(&current_file) {
                break;
            }

            let bit_index =
                Square::try_from(((current_file - 1) + (current_rank - 1) * 8) as u8).unwrap();

            moves_bitboard.set_bit(bit_index);
        }
    }

    let file = File::from(square);
    let rank = Rank::from(square);
    if file != File::A {
        moves_bitboard.0 &= !core::FILE_A;
    }
    if file != File::H {
        moves_bitboard.0 &= !core::FILE_H;
    }
    if rank != Rank::R1 {
        moves_bitboard.0 &= !core::RANK_1
    }
    if rank != Rank::R8 {
        moves_bitboard.0 &= !core::RANK_8
    }

    moves_bitboard
}

fn gen_between_squares() -> [[bitboard::BitBoard; 64]; 64] {
    let mut moves = [[bitboard::BitBoard::new(); 64]; 64];
    for from in SQUARES {
        for to in SQUARES {
            let from_file = File::from(from);
            let from_rank = Rank::from(from);
            let to_file = File::from(to);
            let to_rank = Rank::from(to);
            let same_file = from_file == to_file;
            let same_rank = from_rank == to_rank;
            let same_diag =
                (from_file as i8 - to_file as i8).abs() == (from_rank as i8 - to_rank as i8).abs();
            if from != to && (same_file || same_rank || same_diag) {
                moves[from as usize][to as usize] = gen_between_squares_inner(from, to);
            } else {
                moves[from as usize][to as usize] = bitboard::BitBoard::new();
            }
        }
    }

    moves
}

fn gen_between_squares_inner(from: Square, to: Square) -> bitboard::BitBoard {
    let from_file = File::from(from) as u8;
    let from_rank = Rank::from(from) as u8;
    let to_file = File::from(to) as u8;
    let to_rank = Rank::from(to) as u8;

    let direction: (i8, i8) = if from_file == to_file {
        if from_rank < to_rank {
            (0, 1)
        } else {
            (0, -1)
        }
    } else if from_rank == to_rank {
        if from_file < to_file {
            (1, 0)
        } else {
            (-1, 0)
        }
    } else if from_file < to_file {
        if from_rank < to_rank {
            (1, 1)
        } else {
            (1, -1)
        }
    } else if from_rank < to_rank {
        (-1, 1)
    } else {
        (-1, -1)
    };

    let mut moves = bitboard::BitBoard::new();
    let mut current_file = from_file as i8 + 1;
    let mut current_rank = from_rank as i8 + 1;
    for _ in 0..8 {
        current_file += direction.0;
        current_rank += direction.1;

        if !(1..=8).contains(&current_rank) || !(1..=8).contains(&current_file) {
            break;
        }

        if current_file as u8 - 1 == to_file && current_rank as u8 - 1 == to_rank {
            break;
        }

        let bit_index =
            Square::try_from(((current_file - 1) + (current_rank - 1) * 8) as u8).unwrap();

        moves.set_bit(bit_index);
    }

    moves
}

fn gen_lines(
    rook_moves_table: &Vec<Vec<bitboard::BitBoard>>,
    bishop_moves_table: &Vec<Vec<bitboard::BitBoard>>,
) -> [[bitboard::BitBoard; 64]; 64] {
    let mut lines = [[bitboard::BitBoard::new(); 64]; 64];
    for from in SQUARES {
        for to in SQUARES {
            let from_file = File::from(from);
            let from_rank = Rank::from(from);
            let to_file = File::from(to);
            let to_rank = Rank::from(to);
            let same_file = from_file == to_file;
            let same_rank = from_rank == to_rank;
            let same_diag =
                (from_file as i8 - to_file as i8).abs() == (from_rank as i8 - to_rank as i8).abs();

            if same_rank || same_file {
                lines[from as usize][to as usize] =
                    rook_moves_table[from as usize][0] & rook_moves_table[to as usize][0];
            } else if same_diag {
                lines[from as usize][to as usize] =
                    bishop_moves_table[from as usize][0] & bishop_moves_table[to as usize][0];
            }
        }
    }
    lines
}
