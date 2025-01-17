use super::Board;

use crate::{
    common::{Piece, Square},
    moves::Move,
};

impl Board {
    // Generate all possible moves from this board.
    pub fn generate_moves_for(&self, pieces: &[Piece]) -> Vec<Move> {
        // Pseudo-legal or legal ones?

        let mut moves_list = Vec::new();

        for &moving_pieces in pieces
            .iter()
            .filter(|p| self.get_side_to_move() == p.get_color())
        {
            let own_bb = self.all[self.get_side_to_move() as usize];
            let opposite_bb = self.all[self.opposite_side() as usize];

            let mut pieces_bb = self.pieces[moving_pieces as usize];
            while !pieces_bb.is_zero() {
                let from_bb = pieces_bb.get_ls1b();
                let from_square = from_bb.get_index().into();
                let mut moves_bb = match moving_pieces {
                    Piece::WhiteKing | Piece::BlackKing => from_bb.get_king_moves(own_bb),
                    Piece::WhiteKnight | Piece::BlackKnight => from_bb.get_knight_moves(own_bb),
                    Piece::WhitePawn => from_bb.get_white_pawn_moves(self.occupied, opposite_bb),
                    Piece::BlackPawn => from_bb.get_black_pawn_moves(self.occupied, opposite_bb),
                    Piece::WhiteBishop | Piece::BlackBishop => {
                        from_bb.get_bishop_moves(self.occupied, own_bb)
                    }
                    Piece::WhiteRook | Piece::BlackRook => {
                        from_bb.get_rook_moves(self.occupied, own_bb)
                    }
                    Piece::WhiteQueen | Piece::BlackQueen => {
                        from_bb.get_queen_moves(self.occupied, own_bb)
                    }
                };

                // Generate moves.
                while !moves_bb.is_zero() {
                    let to_bb = moves_bb.get_ls1b();
                    let to_square: Square = to_bb.get_index().into();
                    let is_capture = opposite_bb.contains(to_bb);

                    let mv = Move::new(from_square, to_square, None, moving_pieces, is_capture);
                    moves_list.push(mv);

                    moves_bb = moves_bb.reset_ls1b();
                }

                pieces_bb = pieces_bb.reset_ls1b();
            }
        }
        moves_list
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        self.generate_moves_for(&Piece::ALL_PIECES)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::Piece::*, common::Square::*};

    use super::*;
    #[test]
    fn test_generate_moves_white_king() {
        let board: Board = "2k5/8/8/8/8/8/2Pp4/2K5 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhiteKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C1, B1, WhiteKing),
                Move::quiet(C1, D1, WhiteKing),
                Move::quiet(C1, B2, WhiteKing),
                Move::capture(C1, D2, WhiteKing),
            ]
        );
    }

    #[test]
    fn test_generate_moves_black_king() {
        let board: Board = "2k5/2Pp4/8/8/8/8/8/2K5 b - - 0 1".into();
        let moves = board.generate_moves_for(&[BlackKing]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C8, B7, BlackKing),
                Move::capture(C8, C7, BlackKing),
                Move::quiet(C8, B8, BlackKing),
                Move::quiet(C8, D8, BlackKing),
            ]
        );
    }

    #[test]
    fn test_generate_moves_white_knight() {
        let board: Board = "8/8/6p1/5N2/8/1N6/8/8 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhiteKnight]);
        assert_eq!(
            moves,
            &[
                Move::quiet(B3, A1, WhiteKnight),
                Move::quiet(B3, C1, WhiteKnight),
                Move::quiet(B3, D2, WhiteKnight),
                Move::quiet(B3, D4, WhiteKnight),
                Move::quiet(B3, A5, WhiteKnight),
                Move::quiet(B3, C5, WhiteKnight),
                Move::quiet(F5, E3, WhiteKnight),
                Move::quiet(F5, G3, WhiteKnight),
                Move::quiet(F5, D4, WhiteKnight),
                Move::quiet(F5, H4, WhiteKnight),
                Move::quiet(F5, D6, WhiteKnight),
                Move::quiet(F5, H6, WhiteKnight),
                Move::quiet(F5, E7, WhiteKnight),
                Move::quiet(F5, G7, WhiteKnight),
            ]
        );
    }

    #[test]
    fn test_generate_moves_white_pawn() {
        let board: Board = "8/8/8/8/4N3/n1pB2P1/PPPPPPPP/8 w - - 0 1".into();
        let moves = board.generate_moves_for(&[WhitePawn]);
        assert_eq!(
            moves,
            &[
                Move::capture(B2, A3, WhitePawn),
                Move::quiet(B2, B3, WhitePawn),
                Move::capture(B2, C3, WhitePawn),
                Move::quiet(B2, B4, WhitePawn),
                Move::capture(D2, C3, WhitePawn),
                Move::quiet(E2, E3, WhitePawn),
                Move::quiet(F2, F3, WhitePawn),
                Move::quiet(F2, F4, WhitePawn),
                Move::quiet(H2, H3, WhitePawn),
                Move::quiet(H2, H4, WhitePawn),
                Move::quiet(G3, G4, WhitePawn),
            ]
        );
    }

    #[test]
    fn test_generate_moves_black_pawn() {
        let board: Board = "8/pppppppp/n1pB2P1/4N3/8/8/8/8 b - - 0 1".into();
        let moves = board.generate_moves_for(&[BlackPawn]);
        assert_eq!(
            moves,
            &[
                Move::quiet(C6, C5, BlackPawn),
                Move::quiet(B7, B5, BlackPawn),
                Move::quiet(B7, B6, BlackPawn),
                Move::capture(C7, D6, BlackPawn),
                Move::capture(E7, D6, BlackPawn),
                Move::quiet(E7, E6, BlackPawn),
                Move::quiet(F7, F5, BlackPawn),
                Move::quiet(F7, F6, BlackPawn),
                Move::capture(F7, G6, BlackPawn),
                Move::quiet(H7, H5, BlackPawn),
                Move::capture(H7, G6, BlackPawn),
                Move::quiet(H7, H6, BlackPawn),
            ]
        );
    }
}
