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
}

fn main() {
    
    let mut board = Board::new();
    board.display();
}
