use board::{Board, Color, Square};

pub fn parse_fen(fen: &str) -> Option<Board> {
    let mut board = Board::empty();
    let fen_parts: Vec<_> = fen.split(' ').collect();

    parse_piece_placement(fen_parts[0], &mut board);
    parse_side_to_play(fen_parts[1], &mut board);
    parse_castling(fen_parts[2], &mut board);
    parse_en_passant(fen_parts[3], &mut board);
    parse_fifty_move(fen_parts[4], &mut board);
    parse_full_move(fen_parts[5], &mut board);

    Some(board)
}

pub fn to_fen(board: &Board) -> String {

}

fn parse_piece_placement(placement: &str, board: &mut Board) {
    let mut pos = Square::from_coords(0, 7);
    for fen_char in placement.chars() {
        if fen_char == '/' {
            pos = pos.left(8);
            pos = pos.down(1);
        } else if let Some(n) = fen_char.to_digit(10) {
            pos = pos.right(n as u8);
        } else {
            let to_change = match fen_char {
                'P' => &mut board.white.pawns,
                'N' => &mut board.white.knights,
                'B' => &mut board.white.bishops,
                'R' => &mut board.white.rooks,
                'Q' => &mut board.white.queens,
                'K' => &mut board.white.king,
                'p' => &mut board.black.pawns,
                'n' => &mut board.black.knights,
                'b' => &mut board.black.bishops,
                'r' => &mut board.black.rooks,
                'q' => &mut board.black.queens,
                'k' => &mut board.black.king,
                _ => panic!("Unknown piece: {}", fen_char)
            };

            *to_change |= pos.to_bitboard();
            pos = pos.right(1);
        }
    }
}

fn parse_castling(castling: &str, board: &mut Board) {
    if castling.contains('K') {
        board.white_can_oo = true;
    }

    if castling.contains('Q') {
        board.white_can_ooo = true;
    }

    if castling.contains('k') {
        board.black_can_oo = true;
    }

    if castling.contains('q') {
        board.black_can_ooo = true;
    }
}

fn parse_side_to_play(side_to_play: &str, board: &mut Board) {
    board.to_play = match side_to_play {
        "w" => Color::White,
        "b" => Color::Black,
        _ => panic!("Unknown side to play: {}", side_to_play)
    };
}

fn parse_en_passant(en_passant: &str, board: &mut Board) {
    board.en_passant = match en_passant {
        "-" => None,
        _ => Some(Square::from_san(en_passant))
    };
}

fn parse_fifty_move(fifty_move_clock: &str, board: &mut Board) {
    board.fifty_move_clock = fifty_move_clock.parse().unwrap();
}

fn parse_full_move(full_move_clock: &str, board: &mut Board) {
    board.full_move_clock = full_move_clock.parse().unwrap();
}

#[test]
fn test_fen_starting_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = parse_fen(fen).unwrap();

    println!("");
    println!("{}", board);
    panic!();
}
