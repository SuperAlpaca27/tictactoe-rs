mod board;
use board::{Board, Piece};
use std::io::Write;

fn clear_term() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    clear_term();
    'game: loop {
        let mut board = Board::new();
        let mut current_player = Piece::X;
        println!("{}", board);
        loop {
            'input: loop {
                let mut buffer = String::new();
                println!(
                    "Choose a board position to place '{}' (1-9): ",
                    current_player
                );
                std::io::stdin().read_line(&mut buffer).unwrap();
                let pos = buffer.trim().parse::<usize>();
                match pos {
                    Ok(pos) => match board.insert(pos, current_player) {
                        Ok(_) => break 'input,
                        Err(err) => println!("Error: {}", err),
                    },
                    Err(err) => println!("Error: {}", err),
                }
            }
            if let Some(winner) = board.check_win() {
                clear_term();
                println!("{}", board);
                match winner {
                    Piece::Empty => println!("It was a draw!"),
                    _ => println!("The winner was: {}", winner),
                }
                println!("Restarting the game...");
                std::thread::sleep(std::time::Duration::from_secs(5));
                //Scuffed clear screen
                clear_term();
                continue 'game;
            }
            //Scuffed clear screen
            clear_term();
            println!("{}", board);
            std::io::stdout().flush().unwrap();
            current_player = match current_player {
                Piece::X => Piece::O,
                Piece::O => Piece::X,
                _ => unreachable!(),
            }
        }
    }
}
