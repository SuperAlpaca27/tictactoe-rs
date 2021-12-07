use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Piece {
    Empty,
    X,
    O,
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Empty => write!(f, " "),
            Piece::X => write!(f, "X"),
            Piece::O => write!(f, "O"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Board {
    board: [Piece; 9],
}

impl Board {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, pos: usize, piece: Piece) -> Result<(), &'static str> {
        if pos >= 1 && pos <= 9 {
            let pos = pos - 1;
            match self.board[pos] {
                Piece::Empty => {
                    self.board[pos] = piece;
                    Ok(())
                }
                _ => Err("There is already a piece in that position"),
            }
        } else {
            Err("Position must be between 1-9")
        }
    }

    fn check_slice(slice: &[&Piece; 3]) -> Option<Piece> {
        if slice[0] == slice[1] && slice[1] == slice[2] {
            match slice[0] {
                Piece::X => return Some(Piece::X),
                Piece::O => return Some(Piece::O),
                Piece::Empty => return None,
            }
        }
        None
    }

    pub fn check_win(&self) -> Option<Piece> {
        if self.board.iter().all(|piece| *piece != Piece::Empty) {
            return Some(Piece::Empty);
        }
        //Horizontal
        for i in (0..9).step_by(3) {
            println!("{}", i);
            let check =
                Board::check_slice(&[&self.board[i], &self.board[i + 1], &self.board[i + 2]]);
            if let Some(piece) = check {
                return Some(piece);
            }
        }
        //Vertical
        for i in 0..3 {
            let check =
                Board::check_slice(&[&self.board[i], &self.board[i + 3], &self.board[i + 6]]);
            if let Some(piece) = check {
                return Some(piece);
            }
        }
        //Diagonals
        let check = Board::check_slice(&[&self.board[0], &self.board[4], &self.board[8]]);
        if let Some(piece) = check {
            return Some(piece);
        }

        let check = Board::check_slice(&[&self.board[2], &self.board[4], &self.board[6]]);
        if let Some(piece) = check {
            return Some(piece);
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, piece) in self.board.iter().enumerate() {
            if (i + 1) % 3 == 0 {
                if *piece == Piece::Empty {
                    write!(f, "|{}|\n", i + 1)?
                } else {
                    write!(f, "|{}|\n", piece)?
                }
            } else {
                if *piece == Piece::Empty {
                    write!(f, "|{}", i + 1)?
                } else {
                    write!(f, "|{}", piece)?
                }
            }
        }
        Ok(())
    }
}
