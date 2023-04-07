use std::io::{self, Write};

struct Board {
    grid: [[char; 7]; 6 ],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [['#'; 7]; 6],
        }
    }

    fn display(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            for col in row.iter() {
                print!("{}", col);
            }
            println!("");
        }
    }
    
    fn place_mark(&mut self, col: usize, mark: char) {
        for row in self.grid.iter_mut().rev() {
            if row[col] == '#' {
               row[col] = mark;
               break;
            }
            else {
                continue;
            }
        } 
    }
    fn check_for_win(&self, mark: char) -> bool {
        for i in 0..6 {
        for j in 0..7 {
            if self.grid[i][j] == mark {
                if j <= 3 && self.grid[i][j+1] == mark && self.grid[i][j+2] == mark && self.grid[i][j+3] == mark {
                    return true;
                }
                if i <= 2 && self.grid[i+1][j] == mark &&self.grid[i+2][j] == mark && self.grid[i+3][j] == mark {
                    return true;
                }
                if i <= 2 && j <= 3 && self.grid[i+1][j+1] == mark && self.grid[i+2][j+2] == mark && self.grid[i+3][j+3] == mark {
                    return true;
                }
                if i >= 3 && j <= 3 && self.grid[i-1][j+1] == mark && self.grid[i-2][j+2] == mark && self.grid[i-3][j+3] == mark {
                    return true;
                }
            }
        }
    }
    return false;
}

}

fn main() {
    
    let mut board = Board::new();
    let mut current_player = 'O';

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
    
    loop {
        board.display();
        
        //get input
        print!("Player {}, enter your move: ", current_player);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read line");

        if input.len() != 2 {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
            println!("Invalid input");
            continue;
        }

        let col: usize = input.trim().parse().unwrap();

        board.place_mark(col - 1, current_player);
        
        if board.check_for_win(current_player) {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
            board.display();
            println!("{} won the game!", current_player);
            break;
        }

        if current_player == 'X'{
            current_player = 'O';
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
        }
        else {
            current_player = 'X';
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
        }

    }
}
