use crate::bitboard;
use crate::core;
use crate::core::{
    File, Piece, PieceKind, Player, Rank, Square, NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE,
    PLAYERS, SQUARES,
};

const ROOK_MAGIC_SHIFTS: [(u64, u64); 64] = [
    (9403533769828237312, 12),
    (2350887802654171152, 12),
    (9367496296170397728, 12),
    (4505816372611074, 12),
    (1155314076272828432, 12),
    (19140337071167504, 12),
    (2314852411796358208, 12),
    (720576494463631396, 12),
    (2314990947030542416, 12),
    (288371200613157520, 12),
    (37735307788944576, 12),
    (671038562023835776, 12),
    (40611565828833796, 12),
    (6917538313429590280, 12),
    (612518141054189617, 12),
    (4612814122256957568, 12),
    (844725603012624, 12),
    (433157555997638697, 12),
    (1166436426792894530, 12),
    (9245907661567363096, 12),
    (2305860809873819136, 12),
    (18034189752336648, 12),
    (72061994433284610, 12),
    (19423422933180673, 12),
    (2305860609452810307, 12),
    (96759708680224, 12),
    (2323999313492971520, 12),
    (13871371123310008328, 12),
    (72084023119446018, 12),
    (6953562382157349026, 12),
    (9818129213205528586, 12),
    (14087827119988031496, 12),
    (54052336292864000, 12),
    (10526337299459867168, 12),
    (37460707254280, 12),
    (288249222470828064, 12),
    (9259427222287061056, 12),
    (2883430211717169472, 12),
    (577624035750838592, 12),
    (3479039783134560384, 12),
    (9227893503548227584, 12),
    (35185445863440, 12),
    (3544350501074371616, 12),
    (1742893193400158480, 12),
    (9360503398736000, 12),
    (10137497216750080, 12),
    (4681720561420304, 12),
    (11529497071874745864, 12),
    (288798548828389648, 12),
    (16334525135356032, 12),
    (37155867304936449, 12),
    (15010643254055010432, 12),
    (4511433714958400, 12),
    (616413857325058, 12),
    (82753647447933056, 12),
    (54352192657703552, 12),
    (145276830771843330, 12),
    (4901201998686257282, 12),
    (4075771544188829701, 12),
    (289919243330789378, 12),
    (36048622588276817, 12),
    (4611703645442986501, 12),
    (220712735787188354, 12),
    (9169936119660610, 12),
];

const BISHOP_MAGIC_SHIFTS: [(u64, u64); 64] = [
    (180152922049388546, 12),
    (40836557443072, 12),
    (3494934066609782912, 12),
    (11529295447999127554, 12),
    (2784481447940, 12),
    (19703549051011073, 12),
    (9223380858736541697, 12),
    (6341640025747227136, 12),
    (73680969303302, 12),
    (14988151083837693957, 12),
    (8359252688810805248, 12),
    (2413965142988718088, 12),
    (4508083623690240, 12),
    (9024817765294104, 12),
    (5916472673567360, 12),
    (585539557522409472, 12),
    (585784611378855940, 12),
    (146412068011050052, 12),
    (2314965797447270464, 12),
    (1179995896111697920, 12),
    (549448505221974016, 12),
    (1155322850887352835, 12),
    (433192746610622592, 12),
    (4644337389469728, 12),
    (5332335077001282306, 12),
    (578797251154411531, 12),
    (4760587380627342336, 12),
    (18296981723022112, 12),
    (2667292087305158664, 12),
    (9009398282715488, 12),
    (288863703562194944, 12),
    (281552302901280, 12),
    (9225131272639111209, 12),
    (2306125756842903552, 12),
    (4468921991177, 12),
    (562984850293296, 12),
    (2379066102828498960, 12),
    (738652542942184192, 12),
    (9512730857715270020, 12),
    (9042933449789760, 12),
    (27030462643835680, 12),
    (72656896600834112, 12),
    (2305852492776474112, 12),
    (1227793883048443920, 12),
    (800728201584650, 12),
    (1215982930212425732, 12),
    (10376646003959792744, 12),
    (324347151293482016, 12),
    (72132506874299392, 12),
    (10520452782145733060, 12),
    (152207597683869704, 12),
    (612525017299157504, 12),
    (2305843799488729382, 12),
    (297244793100107776, 12),
    (9233021385318633472, 12),
    (145264177997676866, 12),
    (1153062654680764416, 12),
    (18014501639168512, 12),
    (1126518793437826, 12),
    (2323857957485350946, 12),
    (9223374236152791296, 12),
    (324402664815460640, 12),
    (595042542840447584, 12),
    (648958151562920970, 12),
];

pub struct LookupTables {
    knight_moves_table: [bitboard::BitBoard; 64],
    pawn_captures_table: [[bitboard::BitBoard; 64]; 2],
    pawn_moves_table: [[[bitboard::BitBoard; 64]; 2]; 2],
    king_moves_table: [bitboard::BitBoard; 64],
    rook_moves_mask: [bitboard::BitBoard; 64],
    bishop_moves_mask: [bitboard::BitBoard; 64],
    // FIXME: What is the correct type to use here? I'd like to use [[bitboard::BitBoard; 64]; 64]
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
                let mut can_double = false;
                let file = File::from(s);
                let rank = Rank::from(s);
                let player = Player::from(p);
                if rank == Rank::R2 && player == Player::White {
                    can_double = !all_occupancy.get_bit(Square::from((file, Rank::R3)));
                } else if rank == Rank::R7 && player == Player::Black {
                    can_double = !all_occupancy.get_bit(Square::from((file, Rank::R6)));
                }

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
    (blocker_bitboard * magic_number) >> (64 - shifts)
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

        if can_double {
            moves |= bb << 16;
        }

        bitboard::BitBoard(moves)
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb >> 8;

        if can_double {
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
