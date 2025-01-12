use std::fmt;

// The order of the enum is important because it is used to index arrays.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

impl TryFrom<char> for Piece {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece::WhitePawn),
            'p' => Ok(Piece::BlackPawn),
            'N' => Ok(Piece::WhiteKnight),
            'n' => Ok(Piece::BlackKnight),
            'B' => Ok(Piece::WhiteBishop),
            'b' => Ok(Piece::BlackBishop),
            'R' => Ok(Piece::WhiteRook),
            'r' => Ok(Piece::BlackRook),
            'Q' => Ok(Piece::WhiteQueen),
            'q' => Ok(Piece::BlackQueen),
            'K' => Ok(Piece::WhiteKing),
            'k' => Ok(Piece::BlackKing),
            _ => Err("Invalid char piece"),
        }
    }
}

impl From<Piece> for char {
    fn from(val: Piece) -> Self {
        match val {
            Piece::WhitePawn => 'P',
            Piece::BlackPawn => 'p',
            Piece::WhiteKnight => 'N',
            Piece::BlackKnight => 'n',
            Piece::WhiteBishop => 'B',
            Piece::BlackBishop => 'b',
            Piece::WhiteRook => 'R',
            Piece::BlackRook => 'r',
            Piece::WhiteQueen => 'Q',
            Piece::BlackQueen => 'q',
            Piece::WhiteKing => 'K',
            Piece::BlackKing => 'k',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_str = match self {
            Piece::WhitePawn => "♙",
            Piece::BlackPawn => "♟",
            Piece::WhiteKnight => "♘",
            Piece::BlackKnight => "♞",
            Piece::WhiteBishop => "♗",
            Piece::BlackBishop => "♝",
            Piece::WhiteRook => "♖",
            Piece::BlackRook => "♜",
            Piece::WhiteQueen => "♕",
            Piece::BlackQueen => "♛",
            Piece::WhiteKing => "♔",
            Piece::BlackKing => "♚",
        };
        write!(f, "{piece_str}")
    }
}

// Converts a string with pieces into vector of Piece. Starts with pieces on A8, A7, etc.
// Empty squares are represented with dots.
// The string may have line breaks, spaces etc, they are just ignored.
pub fn parse(value: &str) -> Vec<Option<Piece>> {
    value
        .chars()
        .filter_map(|c| match c.try_into() {
            Ok(p) => Some(Some(p)),
            Err(_) => {
                if c == '.' {
                    Some(None)
                } else {
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        assert_eq!(Piece::WhitePawn as usize, 0);
        assert_eq!(Piece::BlackPawn as usize, 1);
        assert_eq!(Piece::WhiteKnight as usize, 2);
        assert_eq!(Piece::BlackKnight as usize, 3);
        assert_eq!(Piece::WhiteBishop as usize, 4);
        assert_eq!(Piece::BlackBishop as usize, 5);
        assert_eq!(Piece::WhiteRook as usize, 6);
        assert_eq!(Piece::BlackRook as usize, 7);
        assert_eq!(Piece::WhiteQueen as usize, 8);
        assert_eq!(Piece::BlackQueen as usize, 9);
        assert_eq!(Piece::WhiteKing as usize, 10);
        assert_eq!(Piece::BlackKing as usize, 11);
    }
}
