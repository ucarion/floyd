use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub white: Army,
    pub black: Army,
    pub to_play: Color,

    pub white_can_oo: bool,
    pub white_can_ooo: bool,
    pub black_can_oo: bool,
    pub black_can_ooo: bool,

    pub en_passant: Option<Square>,
    pub fifty_move_clock: u64,
    pub full_move_clock: u64
}

#[derive(Debug)]
pub struct Piece {
    color: Color,
    piece_type: PieceType
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, Debug)]
pub struct Army {
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queens: Bitboard,
    pub king: Bitboard
}

pub type Bitboard = u64;

// 0..63
#[derive(Debug)]
pub struct Square(u8);

impl Board {
    pub fn empty() -> Board {
        let empty_army = Army {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            king: 0
        };

        Board {
            white: empty_army,
            black: empty_army,
            to_play: Color::White,
            white_can_oo: false,
            white_can_ooo: false,
            black_can_oo: false,
            black_can_ooo: false,
            en_passant: None,
            fifty_move_clock: 0,
            full_move_clock: 0
        }
    }

    pub fn piece_at(&self, square: &Square) -> Option<Piece> {
        let b = square.to_bitboard();
        let (color, piece_type) = if self.white.pawns & b != 0 {
            (Color::White, PieceType::Pawn)
        } else if self.white.knights & b != 0 {
            (Color::White, PieceType::Knight)
        } else if self.white.bishops & b != 0 {
            (Color::White, PieceType::Bishop)
        } else if self.white.rooks & b != 0 {
            (Color::White, PieceType::Rook)
        } else if self.white.queens & b != 0 {
            (Color::White, PieceType::Queen)
        } else if self.white.king & b != 0 {
            (Color::White, PieceType::King)
        } else if self.black.pawns & b != 0 {
            (Color::Black, PieceType::Pawn)
        } else if self.black.knights & b != 0 {
            (Color::Black, PieceType::Knight)
        } else if self.black.bishops & b != 0 {
            (Color::Black, PieceType::Bishop)
        } else if self.black.rooks & b != 0 {
            (Color::Black, PieceType::Rook)
        } else if self.black.queens & b != 0 {
            (Color::Black, PieceType::Queen)
        } else if self.black.king & b != 0 {
            (Color::Black, PieceType::King)
        } else {
            return None;
        };

        Some(Piece { color: color, piece_type: piece_type })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = "+---+---+---+---+---+---+---+---+\n";
        try!(write!(f, "{}", line));

        for rank in (0..8).rev() {
            try!(write!(f, "|"));

            for file in (0..8) {
                let sq = Square::from_coords(file, rank);

                println!("{:?}", sq);

                match self.piece_at(&sq) {
                    Some(piece) => try!(write!(f, " {} |", piece)),
                    None => try!(write!(f, "   |"))
                };

            }

            try!(write!(f, "\n"));
            try!(write!(f, "{}", line));
        }

        Ok(())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_write = match (&self.color, &self.piece_type) {
            (&Color::White, &PieceType::Pawn) => 'P',
            (&Color::White, &PieceType::Knight) => 'N',
            (&Color::White, &PieceType::Bishop) => 'B',
            (&Color::White, &PieceType::Rook) => 'R',
            (&Color::White, &PieceType::Queen) => 'Q',
            (&Color::White, &PieceType::King) => 'K',
            (&Color::Black, &PieceType::Pawn) => 'p',
            (&Color::Black, &PieceType::Knight) => 'n',
            (&Color::Black, &PieceType::Bishop) => 'b',
            (&Color::Black, &PieceType::Rook) => 'r',
            (&Color::Black, &PieceType::Queen) => 'q',
            (&Color::Black, &PieceType::King) => 'k'
        };

        write!(f, "{}", to_write)
    }
}

impl Square {
    /// Makes a Square from a (file, rank) pair. To represent "a8", pass (0, 7).
    pub fn from_coords(file: u8, rank: u8) -> Square {
        Square(file + rank * 8)
    }

    /// Makes a Square from Standard Algebraic Notation (e.g. "a8").
    pub fn from_san(san: &str) -> Square {
        let san: Vec<_> = san.chars().collect();
        let file = match san[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Unknown file: {:?}", san[0])
        };

        let rank = match san[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => panic!("Unknown rank: {:?}", san[1])
        };

        Square::from_coords(file, rank)
    }

    pub fn to_bitboard(&self) -> Bitboard {
        1 << self.0
    }

    pub fn left(&self, amount: u8) -> Square {
        Square(self.0 - amount)
    }

    pub fn right(&self, amount: u8) -> Square {
        Square(self.0 + amount)
    }


    pub fn down(&self, amount: u8) -> Square {
        Square(self.0 - amount * 8)
    }
}

