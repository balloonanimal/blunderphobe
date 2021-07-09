use crate::boardstructs::{Square, Direction, Rank, File, Shiftable};
use std::convert::From;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

#[derive(PartialEq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn len(self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn invert(self) -> Self {
        Self(!self.0)
    }

    pub fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn contains(self, sqr: Square) -> bool {
        let sqrbb = Self::from(sqr);
        self.intersection(sqrbb) == sqrbb
    }

    pub fn is_disjoint(self, other: Self) -> bool {
        (self.0 & other.0) == 0
    }

    pub fn is_subset(self, other: Self) -> bool {
        (self.0 & other.0) == self.0
    }

    pub fn is_supset(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn insert(&mut self, sqr: Square) -> bool {
        let contains = self.contains(sqr);
        *self = self.union(Self::from(sqr));
        contains
    }

    pub fn remove(&mut self, sqr: Square) -> bool {
        let contains = self.contains(sqr);
        *self = self.union(Self::from(sqr).invert());
        contains
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> u64 {
        bitboard.0
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Bitboard {
        Bitboard(value)
    }
}

impl From<Square> for Bitboard {
    fn from(sqr: Square) -> Self {
        Bitboard::from(0x1 << u8::from(sqr))
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

pub struct BitboardIntoIterator {
    bb: Bitboard,
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIntoIterator { bb: self }
    }
}

impl Iterator for BitboardIntoIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bb.is_empty() {
            return None;
        }
        let idx = self.bb.0.trailing_zeros();
        let sqr = Square::new(idx as u8);
        self.bb.remove(sqr);
        Some(sqr)
    }
}


// NOTE: Maybe doesn't belong in this module?
impl Shiftable for Bitboard {
    type ShiftType = Self;

    fn shift(&self, direction: Direction) -> Self::ShiftType {
        match direction {
            Direction::Up => { let maskedbb = *self & Self::from(Rank::new(7));
                    Self(maskedbb.0 << 8)
            }
            Direction::Right => { let maskedbb = *self & Self::from(File::new(7));
                    Self(maskedbb.0 << 1)
            }
            Direction::Down => { let maskedbb = *self & Self::from(Rank::new(0));
                    Self(maskedbb.0 >> 8)
            }
            Direction::Left => { let maskedbb = *self & Self::from(File::new(0));
                    Self(maskedbb.0 >> 1)
            }
            Direction::UpRight => { self.shift(Direction::Up).shift(Direction::Right) }
            Direction::DownRight => { self.shift(Direction::Down).shift(Direction::Right) }
            Direction::DownLeft => { self.shift(Direction::Up).shift(Direction::Left) }
            Direction::UpLeft => { self.shift(Direction::Up).shift(Direction::Left) }
        }
    }
}
