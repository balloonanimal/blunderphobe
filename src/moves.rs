mod magic;

mod moveboards;
use moveboards::{MoveBoards, MOVEBOARDS};

use crate::boardstructs::{Direction, File, Piece, Rank, Shiftable, Square};
use crate::position::Position;

use smallvec::SmallVec;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Move {
    from: Square,
    to: Square,
    promotion: bool,
    capture: bool,
}

impl Move {
    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
struct PseudolegalMove(Move);

// NOTE: There are always (as far as we know) fewer than 256 chess moves in a
//       position, so we can use a (pretty) small vector here and store on the
//       stack for a performance benefit. See
//       https://www.chessprogramming.org/Encoding_Moves#MoveIndex
type MoveVec<T> = SmallVec<[T; 256]>;

#[derive(Debug)]
pub enum MoveError {
    IllegalMove(PseudolegalMove),
}

pub fn generate_moves(pos: Position) -> MoveVec<Move>{

    todo!()
}
