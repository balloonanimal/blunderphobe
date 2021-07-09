use crate::bitboard::Bitboard;
use crate::boardstructs::{File, Rank, Square};

use super::moveboards::{bishop_move_board_slow, rook_move_board_slow};

use smallvec::SmallVec;

pub struct SquareMagic {
    table: [Bitboard; 4096],
    mask: Bitboard,
    magic_num: u64,
    shift: u8,
}

impl SquareMagic {
    pub fn lookup(&self, blockers: Bitboard) -> Bitboard {
        let relevant_blockers = blockers & self.mask;
        let idx = magic_hash(relevant_blockers, self.magic_num, self.shift);
        self.table[idx as usize]
    }
}

// TODO: this is expensive, there's probably a better way
fn create_sqr_array<T, F>(f: F) -> [T; 64]
where
    F: FnMut(Square) -> T,
{
    (0..64)
        .map(|i| Square::new(i))
        .map(f)
        .collect::<SmallVec<[T; 64]>>()
        .into_inner()
        .unwrap_or_else(|v| {
            panic!(
                "Expected a vector of size 64, instead got one of size {}.\
             This state is (hopefully) unreachable, but the compiler doesn't know that.",
                v.len()
            )
        })
}

fn bitboard_combinations(bb: Bitboard) -> Vec<Bitboard> {
    let mut combinations = Vec::<Bitboard>::new();
    let squares = bb.into_iter().collect::<Vec<Square>>();
    for combo_idx in 0..(u64::pow(2, bb.len().into())) {
        let mut bb = Bitboard::new();
        // not actually a bitboard representing a board, we use it instead to
        // index a combination
        let combo_bb = Bitboard::from(combo_idx);
        for i in 0..squares.len() {
            if combo_bb.contains(Square::new(i as u8)) {
                bb.insert(squares[i as usize]);
            }
        }
        combinations.push(bb);
    }
    combinations
}

pub fn gen_bishop_magics() -> [SquareMagic; 64] {
    create_sqr_array(gen_bishop_magic_for_sqr)
}

fn gen_bishop_magic_for_sqr(sqr: Square) -> SquareMagic {
    let mask = {
        bishop_move_board_slow(sqr, Bitboard::new())
            & Bitboard::from(Rank::new(0)).invert()
            & Bitboard::from(Rank::new(7)).invert()
            & Bitboard::from(File::new(0)).invert()
            & Bitboard::from(File::new(7)).invert()
    };
    let combinations = bitboard_combinations(mask);
    let moves = combinations
        .iter()
        .map(|&blockers| bishop_move_board_slow(sqr, blockers))
        .collect::<Vec<Bitboard>>();
    gen_square_magic(&combinations, &moves)
}

pub fn gen_rook_magics() -> [SquareMagic; 64] {
    create_sqr_array(gen_rook_magic_for_sqr)
}

fn gen_rook_magic_for_sqr(sqr: Square) -> SquareMagic {
    let mask = {
        rook_move_board_slow(sqr, Bitboard::new())
            & Bitboard::from(Rank::new(0)).invert()
            & Bitboard::from(Rank::new(7)).invert()
            & Bitboard::from(File::new(0)).invert()
            & Bitboard::from(File::new(7)).invert()
    };
    let combinations = bitboard_combinations(mask);
    let moves = combinations
        .iter()
        .map(|&blockers| rook_move_board_slow(sqr, blockers))
        .collect::<Vec<Bitboard>>();
    gen_square_magic(&combinations, &moves)
}

fn magic_hash(key: Bitboard, magic: u64, shift: u8) -> u64 {
    (u64::from(key) * magic) << shift
}

fn magic_pushes_up(magic: u64, mask: Bitboard) -> bool {
    let prod = magic * u64::from(mask);
    let num_high_bits = prod & 0xff00000000000000;
    num_high_bits > 6
}

fn gen_square_magic(possible_blockers: &[Bitboard], possible_moves: &[Bitboard]) -> SquareMagic {
    let mask = possible_moves[0];
    let shift = 64 - mask.len();
    // The maximum size that a hash key can be is 12, for a corner rook move.
    // Therefore our table only needs to be 4096 = 2^12 large
    let mut table = [Bitboard::new(); 4096];
    let mut magic: Option<u64> = None;
    while let None = magic {
        // zero out table
        for bb in table.iter_mut() {
            *bb = Bitboard::new()
        }
        // Use & thrice to get a random value with a low number of non-zero bits
        let magic_guess = rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>();
        magic = Some(magic_guess);
        // We want our magic number to "push up" the bits of mask so that the top byte has many non-zero bits
        if !magic_pushes_up(magic_guess, mask) {
            continue;
        }
        for (blockers, moves) in possible_blockers.iter().zip(possible_moves) {
            let idx = magic_hash(*blockers, magic_guess, shift) as usize;
            if table[idx] == Bitboard::new() {
                table[idx] = *moves;
            } else if table[idx] == *moves {
                // Do nothing, this is a good hash collision where two blocker
                // bitboards share the same resultant move bitboard and also
                // happen to map to the same index
            } else {
                magic = None;
                break;
            }
        }
    }
    SquareMagic {
        table,
        mask,
        magic_num: magic.unwrap(),
        shift,
    }
}
