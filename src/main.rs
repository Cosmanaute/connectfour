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
                print!(" {} ", col);
            }
            println!("");
        }
    }
    
    //iterates backwards(from bottom) in a column to check if any cell is empty and places mark in cell if so
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
                //checks horizontal
                if j <= 3 && self.grid[i][j+1] == mark && self.grid[i][j+2] == mark && self.grid[i][j+3] == mark {
                    return true;
                }
                //checks vertical
                if i <= 2 && self.grid[i+1][j] == mark &&self.grid[i+2][j] == mark && self.grid[i+3][j] == mark {
                    return true;
                }
                //checks diagonal
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
    let mut current_player = '$';

    //sends escape sequence that clears the terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
    
    loop {

        board.display();
        
        //gets input
        print!("Player {}, enter your move: ", current_player);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read line");
        
        if input.trim().parse::<i32>().is_err() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
            println!("Must be numeric!");
            continue;
        }

        //parses the input(string) to a usize(u64/u32)
        let col: usize = input.trim().parse().unwrap();

        if col < 1 || col > 7 {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
            println!("May not be less than 1 or greater than 7!");
            continue;
        }

        board.place_mark(col - 1, current_player);
        
        if board.check_for_win(current_player) {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
            board.display();
            println!("{} won the game!", current_player);
            break;
        }

        if current_player == '@'{
            current_player = '$';
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
        }
        else {
            current_player = '@';
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
        }
    }
}
