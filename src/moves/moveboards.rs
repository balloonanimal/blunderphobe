use crate::bitboard::Bitboard;

use crate::boardstructs::{Direction, Piece, Shiftable, Square};

use super::magic::{SquareMagic, gen_bishop_magics, gen_rook_magics};

use lazy_static::lazy_static;

// slow iterative generation of moveboards, only called during the initialization to generate magic bitboards
pub fn bishop_move_board_slow(sqr: Square, blockers: Bitboard) -> Bitboard {
    let mut moves = Bitboard::new();
    for direction in &[
        Direction::UpRight,
        Direction::DownRight,
        Direction::DownLeft,
        Direction::UpLeft,
    ] {
        let mut destination = sqr.shift(*direction);
        while let Some(s) = destination {
            moves.insert(s);
            if blockers.contains(s) {
                break;
            }
            destination = sqr.shift(*direction);
        }
    }
    moves
}

pub fn rook_move_board_slow(sqr: Square, blockers: Bitboard) -> Bitboard {
    let mut moves = Bitboard::new();
    for direction in &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let mut destination = sqr.shift(*direction);
        while let Some(s) = destination {
            moves.insert(s);
            if blockers.contains(s) {
                break;
            }
            destination = sqr.shift(*direction);
        }
    }
    moves
}

// This struct stores bitboards that map from a square to all the squares a
// piece on that square threatens, assuming an empty board
pub struct MoveBoards {
    // boards for non-sliding move generation
    king_moves: [Bitboard; 64],
    knight_moves: [Bitboard; 64],

    // magics for sliding move generation
    bishop_magics: [SquareMagic; 64],
    rook_magics: [SquareMagic; 64],
    // threat boards for sliding pieces
    // not actually used for move generation
    // bishop_threats: [Bitboard; 64],
    // rook_threats: [Bitboard; 64],
}

// initialization
impl MoveBoards {
    fn gen_king_moves() -> [Bitboard; 64] {
        let mut boards = [Bitboard::new(); 64];
        for sqr_idx in 0..64 {
            let sqr = Square::new(sqr_idx);
            for direction in &[
                Direction::Up,
                Direction::UpLeft,
                Direction::Left,
                Direction::DownLeft,
                Direction::Down,
                Direction::DownRight,
                Direction::Right,
                Direction::UpRight,
            ] {
                if let Some(s) = sqr.shift(*direction) {
                    boards[sqr_idx as usize].insert(s);
                }
            }
        }
        boards
    }

    fn gen_knight_moves() -> [Bitboard; 64] {
        let mut boards = [Bitboard::new(); 64];
        for sqr_idx in 0..64 {
            let sqr = Square::new(sqr_idx);
            for [d1, d2, d3] in &[
                [Direction::Up, Direction::Up, Direction::Right],
                [Direction::Right, Direction::Right, Direction::Up],
                [Direction::Right, Direction::Right, Direction::Down],
                [Direction::Down, Direction::Down, Direction::Right],
                [Direction::Down, Direction::Down, Direction::Left],
                [Direction::Left, Direction::Left, Direction::Down],
                [Direction::Left, Direction::Left, Direction::Up],
                [Direction::Up, Direction::Up, Direction::Left],
            ] {
                // TODO: Look at some map-ish way of doing this. Map doesn't work bc
                // Option<Square>.map(|s| s.shift(d)) :: Option<Option<Square>>
                let mut destination = sqr.shift(*d1);
                if let Some(s) = destination {
                    destination = s.shift(*d2);
                }
                if let Some(s) = destination {
                    destination = s.shift(*d3);
                }
                if let Some(s) = destination {
                    boards[sqr_idx as usize].insert(s);
                }
            }
        }
        boards
    }

    // NOTE: This function is only intended to be called once in a program's execution, you don't need multiple
    // TODO: make this constant
    fn new() -> MoveBoards {
        let king_moves = MoveBoards::gen_king_moves();
        let knight_moves = MoveBoards::gen_knight_moves();
        let bishop_magics = gen_bishop_magics();
        let rook_magics = gen_rook_magics();

        MoveBoards {
            king_moves,
            knight_moves,
            bishop_magics,
            rook_magics,
        }
    }

    // TODO: look into a version of this without blockers
    pub fn move_board(&self, sqr: Square, piece: Piece, blockers: Bitboard) -> Bitboard {
        match piece {
            Piece::None => {
                panic!("Can't use move_board on None piece")
            }
            Piece::WhitePawn | Piece::BlackPawn => {
                panic!("Can't use move_board to get pawn moves, use TODO method instead")
            }
            Piece::WhiteKing | Piece::BlackKing => self.king_moves[u8::from(sqr) as usize],
            Piece::WhiteKnight | Piece::BlackKnight => self.knight_moves[u8::from(sqr) as usize],
            Piece::WhiteBishop | Piece::BlackBishop => {
                self.bishop_magics[u8::from(sqr) as usize].lookup(blockers)
            }
            Piece::WhiteRook | Piece::BlackRook => {
                self.rook_magics[u8::from(sqr) as usize].lookup(blockers)
            }
            Piece::WhiteQueen | Piece::BlackQueen => {
                self.bishop_magics[u8::from(sqr) as usize].lookup(blockers)
                    | self.rook_magics[u8::from(sqr) as usize].lookup(blockers)
            }
        }
    }
}

lazy_static! {
    pub static ref MOVEBOARDS: MoveBoards = MoveBoards::new();
}

// impl Position {
//     pub fn moves(&self) -> PrettySmallVec<Move> {
//         let pseudolegal_moves = self.pseudolegal_moves();
//         pseudolegal_moves
//             .into_iter()
//             .filter_map(|m| self.try_convert_to_legal_move(m).ok())
//             .collect()
//     }

//     pub fn try_convert_to_legal_move(&self, mv: PseudolegalMove) -> Result<Move, MoveError> {
//         unimplemented!();
//     }

//     pub fn pseudolegal_moves(&self) -> PrettySmallVec<PseudolegalMove> {
//         let mut v = PrettySmallVec::<PseudolegalMove>::new();
//         todo!()
//         // self.pawn_moves(&mut v);
//         // self.pawn_moves(&mut v);
//     }
// }

// pub fn pseudolegal_moves(&self) -> Vec<PseudoLegalMove> {
//     let v = Vec<PseudoLegalMove>::new();
//     pawn_moves(&self, &mut v);
//     knight_moves(&self, &mut v);
//     bishop_moves(&self, &mut v);
//     rook_moves(&self, &mut v);
//     queen_moves(&self, &mut v);
//     king_moves(&self, &mut v);
//     v
// }

// pub fn pawn_moves(&self, &mut v: Vec<PseudoLegalMove>) {
//     let (pawn_char, enemy_pawn_char, up_dir, starting_rank, promotion_rank) = match self.current_player {
//         Player::White => 'P', 'p', 12, 2, 8
//         Player::Black => 'p', 'P', -12, 7, 1
//     }
// }
// }

// fn moveboard_for(&self, piece: Piece, sqr: Square) -> Bitboard {
//     match piece {
//         Piece::WhiteKnight => self.knight_threats[u8::from(sqr) as usize],
//         Piece::WhiteBishop => self.bishop_threats[u8::from(sqr) as usize],
//         Piece::WhiteRook => self.rook_threats[u8::from(sqr) as usize],
//         Piece::WhiteQueen => {
//             self.bishop_threats[u8::from(sqr) as usize]
//                 & self.rook_threats[u8::from(sqr) as usize]
//         }
//         Piece::WhiteKing => self.king_threats[u8::from(sqr) as usize],
//         Piece::BlackKnight => self.knight_threats[u8::from(sqr) as usize],
//         Piece::BlackBishop => self.bishop_threats[u8::from(sqr) as usize],
//         Piece::BlackRook => self.rook_threats[u8::from(sqr) as usize],
//         Piece::BlackQueen => {
//             self.bishop_threats[u8::from(sqr) as usize]
//                 & self.rook_threats[u8::from(sqr) as usize]
//         }
//         Piece::BlackKing => self.king_threats[u8::from(sqr) as usize],
//         _ => panic!("Don't use MoveBoards for pawn moves, pawns are weird."),
//     }
// }
