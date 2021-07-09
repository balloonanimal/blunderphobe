use crate::bitboard::Bitboard;
use std::convert::From;

// Rank
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Rank(u8);

impl Rank {
    pub fn new(value: u8) -> Rank {
        assert!(value < 8);
        Rank(value)
    }
}

impl From<Rank> for u8 {
    fn from(rank: Rank) -> u8 {
        rank.0
    }
}

impl From<Rank> for Bitboard {
    fn from(rank: Rank) -> Bitboard {
        let mut bb = Bitboard::new();
        for file in 0..7 {
            bb.insert(Square::from_coords(rank, File::new(file)));
        }
        bb
    }
}

// File
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct File(u8);

impl File {
    pub fn new(value: u8) -> File {
        assert!(value < 8);
        File(value)
    }
}

impl From<File> for u8 {
    fn from(file: File) -> u8 {
        file.0
    }
}

impl From<File> for Bitboard {
    fn from(file: File) -> Bitboard {
        let mut bb = Bitboard::new();
        for rank in 0..7 {
            bb.insert(Square::from_coords(Rank::new(rank), file));
        }
        bb
    }
}

// Square
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Square(u8);

impl Square {
    pub fn new(value: u8) -> Square {
        assert!(value <= 63);
        Square(value)
    }
}

#[derive(Debug)]
pub enum SquareError {
    InvalidString(String),
}

impl From<Square> for u8 {
    fn from(sqr: Square) -> u8 {
        sqr.0
    }
}

impl Square {
    pub fn from_coords(rank: Rank, file: File) -> Square {
        Square::new(u8::from(rank) * 8 + u8::from(file))
    }

    pub fn try_from_str(s: &str) -> Result<Square, SquareError> {
        if s.len() == 2 {
            let b = s.as_bytes();
            let rankchar = b[1];
            let filechar = b[0];
            if 97 <= filechar && filechar <= 104 && 49 <= rankchar && rankchar <= 56 {
                let rank = Rank(rankchar - 49);
                let file = File(filechar - 97);
                return Ok(Square::from_coords(rank, file));
            };
        };
        Err(SquareError::InvalidString(String::from(s)))
    }

    pub fn rank(self) -> Rank {
        Rank(self.0 / 8)
    }

    pub fn file(self) -> File {
        File(self.0 % 8)
    }
}

// Direction
#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub trait Shiftable {
    type ShiftType;
    fn shift(&self, direction: Direction) -> Self::ShiftType;
}

impl Shiftable for Rank {
    type ShiftType = Option<Rank>;

    fn shift(&self, direction: Direction) -> Self::ShiftType {
        let r = self.0;
        match direction {
            Direction::Up => {
                if r < 7 {
                    Some(Rank::new(r + 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if r > 0 {
                    Some(Rank::new(r - 1))
                } else {
                    None
                }
            }
            Direction::UpRight => self.shift(Direction::Up),
            Direction::DownRight => self.shift(Direction::Down),
            Direction::DownLeft => self.shift(Direction::Down),
            Direction::UpLeft => self.shift(Direction::Up),
            Direction::Right | Direction::Left => Some(*self),
        }
    }
}

impl Shiftable for File {
    type ShiftType = Option<File>;

    fn shift(&self, direction: Direction) -> Self::ShiftType {
        let f = self.0;
        match direction {
            Direction::Right => {
                if f < 7 {
                    Some(File::new(f + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if f > 0 {
                    Some(File::new(f - 1))
                } else {
                    None
                }
            }
            Direction::UpRight => self.shift(Direction::Right),
            Direction::DownRight => self.shift(Direction::Right),
            Direction::DownLeft => self.shift(Direction::Left),
            Direction::UpLeft => self.shift(Direction::Left),
            Direction::Up | Direction::Down => Some(*self),
        }
    }
}

impl Shiftable for Square {
    type ShiftType = Option<Square>;

    fn shift(&self, direction: Direction) -> Self::ShiftType {
        if let (Some(rank), Some(file)) = (self.rank().shift(direction), self.file().shift(direction)) {
            Some(Square::from_coords(rank, file))
        } else {
            None
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    None,
}

// impl From<Piece> for char {
//     fn from(piece: Piece) -> char {
//         match piece {
//             Piece::WhitePawn => 'P',
//             Piece::WhiteKnight => 'N',
//             Piece::WhiteBishop => 'B',
//             Piece::WhiteRook => 'R',
//             Piece::WhiteQueen => 'Q',
//             Piece::WhiteKing => 'K',
//             Piece::BlackPawn => 'p',
//             Piece::BlackKnight => 'n',
//             Piece::BlackBishop => 'b',
//             Piece::BlackRook => 'r',
//             Piece::BlackQueen => 'q',
//             Piece::BlackKing => 'k',
//             Piece::None => '.',
//         }
//     }
// }
