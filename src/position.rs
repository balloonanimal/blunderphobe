use crate::bitboard::{Bitboard};
use crate::piece::Piece;
use crate::boardstructs::Square;
use crate::moves::Move;

pub enum Player {
    White,
    Black,
}

pub struct Bitboards {
    white: Bitboard,
    black: Bitboard,
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
}

impl Bitboards {
    pub fn piece_bb(&self, piece: Piece) -> Bitboard {
        match piece {
            Piece::WhitePawn => self.white & self.pawns,
            Piece::WhiteKnight => self.white & self.knights,
            Piece::WhiteBishop => self.white & self.bishops,
            Piece::WhiteRook => self.white & self.rooks,
            Piece::WhiteQueen => self.white & self.queens,
            Piece::WhiteKing => self.white & self.kings,
            Piece::BlackPawn => self.black & self.pawns,
            Piece::BlackKnight => self.black & self.knights,
            Piece::BlackBishop => self.black & self.bishops,
            Piece::BlackRook => self.black & self.rooks,
            Piece::BlackQueen => self.black & self.queens,
            Piece::BlackKing => self.black & self.kings,
            Piece::None => (self.white | self.black).invert(),
        }
    }
}

impl From<&PieceBoard> for Bitboards {
    fn from(pieces: &PieceBoard) -> Bitboards {
        let mut white = Bitboard::new();
        let mut black = Bitboard::new();
        let mut pawns = Bitboard::new();
        let mut knights = Bitboard::new();
        let mut bishops = Bitboard::new();
        let mut rooks = Bitboard::new();
        let mut queens = Bitboard::new();
        let mut kings = Bitboard::new();

        for (idx, piece) in pieces.into_iter().enumerate() {
            let sqr = Square::new(idx as u8);
            let sqr_bb = Bitboard::from(sqr);
            match piece {
                Piece::WhitePawn => {
                    white |= sqr_bb;
                    pawns |= sqr_bb;
                }
                Piece::WhiteKnight => {
                    white |= sqr_bb;
                    knights |= sqr_bb;
                }
                Piece::WhiteBishop => {
                    white |= sqr_bb;
                    bishops |= sqr_bb;
                }
                Piece::WhiteRook => {
                    white |= sqr_bb;
                    rooks |= sqr_bb;
                }
                Piece::WhiteQueen => {
                    white |= sqr_bb;
                    queens |= sqr_bb;
                }
                Piece::WhiteKing => {
                    white |= sqr_bb;
                    kings |= sqr_bb;
                }
                Piece::BlackPawn => {
                    black |= sqr_bb;
                    pawns |= sqr_bb;
                }
                Piece::BlackKnight => {
                    black |= sqr_bb;
                    knights |= sqr_bb;
                }
                Piece::BlackBishop => {
                    black |= sqr_bb;
                    bishops |= sqr_bb;
                }
                Piece::BlackRook => {
                    black |= sqr_bb;
                    rooks |= sqr_bb;
                }
                Piece::BlackQueen => {
                    black |= sqr_bb;
                    queens |= sqr_bb;
                }
                Piece::BlackKing => {
                    black |= sqr_bb;
                    kings |= sqr_bb;
                }
                Piece::None => {}
            }
        }
        Bitboards {
            white: white,
            black: black,
            pawns: pawns,
            knights: knights,
            bishops: bishops,
            rooks: rooks,
            queens: queens,
            kings: kings,
        }
    }
}


pub type PieceBoard = [Piece; 64];

pub struct Position {
    board: PieceBoard,
    bitboards: Bitboards,
    current_player: Player,
}

impl Position {
    pub fn starting_position() -> Position {
        let board = [
            Piece::WhiteRook,
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteQueen,
            Piece::WhiteKing,
            Piece::WhiteBishop,
            Piece::WhiteKnight,
            Piece::WhiteRook,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::None,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackRook,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackQueen,
            Piece::BlackKing,
            Piece::BlackBishop,
            Piece::BlackKnight,
            Piece::BlackRook,
        ];

        Position {
            board: board,
            bitboards: Bitboards::from(&board),
            current_player: Player::White,
        }
    }

    // FIXME: check for valid fen / write tests
    pub fn from_fen(fen: &str) -> Position {
        unimplemented!()
    }

    pub fn piece_on(&self, sqr: Square) -> Piece {
        self.board[u8::from(sqr) as usize]
    }

    pub fn set_piece_on(&mut self, piece: Piece, sqr: Square) {
        self.board[u8::from(sqr) as usize] = piece
    }

    pub fn make_move(&mut self, mv: Move) {
        let piece = self.piece_on(mv.from());
        self.set_piece_on(Piece::None, mv.from());
        self.set_piece_on(piece, mv.to());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_types() {
        assert_eq!(std::mem::size_of::<Square>(), 1);
        assert_eq!(std::mem::size_of::<Piece>(), 1);
    }

    #[test]
    fn piece_on() {
        let board = Position::starting_position();
        assert_eq!(
            board.piece_on(Square::try_from_str("a1").unwrap()),
            Piece::WhiteRook
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("b1").unwrap()),
            Piece::WhiteKnight
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("c1").unwrap()),
            Piece::WhiteBishop
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("d1").unwrap()),
            Piece::WhiteQueen
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("e1").unwrap()),
            Piece::WhiteKing
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("f1").unwrap()),
            Piece::WhiteBishop
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("g1").unwrap()),
            Piece::WhiteKnight
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("h1").unwrap()),
            Piece::WhiteRook
        );

        assert_eq!(
            board.piece_on(Square::try_from_str("a8").unwrap()),
            Piece::BlackRook
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("b8").unwrap()),
            Piece::BlackKnight
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("c8").unwrap()),
            Piece::BlackBishop
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("d8").unwrap()),
            Piece::BlackQueen
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("e8").unwrap()),
            Piece::BlackKing
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("f8").unwrap()),
            Piece::BlackBishop
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("g8").unwrap()),
            Piece::BlackKnight
        );
        assert_eq!(
            board.piece_on(Square::try_from_str("h8").unwrap()),
            Piece::BlackRook
        );

        // assert_eq!(board.piece_on(Square::try_from(26)), Piece::None);
        // assert_eq!(board.piece_on(Square::try_from(27)), Piece::None);
        // assert_eq!(board.piece_on(Square::try_from(32)), Piece::None);
        // assert_eq!(board.piece_on(Square::try_from(33)), Piece::None);
    }

    #[test]
    fn make_move() {
        let mut board = Position::starting_position();
        let sqr_e2 = Square::try_from_str("e2").unwrap();
        let sqr_e4 = Square::try_from_str("e4").unwrap();
        let sqr_d7 = Square::try_from_str("d7").unwrap();
        let sqr_d5 = Square::try_from_str("d5").unwrap();
        board.make_move(Move {
            from: sqr_e2,
            to: sqr_e4,
            promotion: false,
            capture: false,
        });
        assert_eq!(board.piece_on(sqr_e2), Piece::None);
        assert_eq!(board.piece_on(sqr_e4), Piece::WhitePawn);

        board.make_move(Move {
            from: sqr_d7,
            to: sqr_d5,
            promotion: false,
            capture: false,
        });
        assert_eq!(board.piece_on(sqr_e2), Piece::None);
        assert_eq!(board.piece_on(sqr_e4), Piece::WhitePawn);
        assert_eq!(board.piece_on(sqr_d7), Piece::None);
        assert_eq!(board.piece_on(sqr_d5), Piece::BlackPawn);
    }
}
