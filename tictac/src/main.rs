use std::cmp::PartialEq;
use std::io;
use std::process::Command;

fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Не удалось очистить терминал");
}
pub struct Game {
    board: [[Symbol; 3]; 3],
    winner: Player,
}
enum Player {
    Nil,
    First,
    Second,
}

#[derive(PartialEq, Copy, Clone)]
enum Symbol {
    Nil,
    Cross,
    Zero,
}

fn make_pl(some: Symbol) -> Player {
    match some {
        Symbol::Cross => Player::First,
        Symbol::Zero => Player::Second,
        Symbol::Nil => Player::Nil,
    }
}
impl Game {
    pub fn new() -> Self {
        Game {
            board: [
                [Symbol::Nil, Symbol::Nil, Symbol::Nil],
                [Symbol::Nil, Symbol::Nil, Symbol::Nil],
                [Symbol::Nil, Symbol::Nil, Symbol::Nil],
            ],
            winner: Player::Nil,
        }
    }

    pub fn check(&mut self) {
        let board = &self.board;
        if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != Symbol::Nil {
            self.winner = make_pl(board[0][0]);
        }
        if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != Symbol::Nil {
            self.winner = make_pl(board[0][2]);
        }
        for i in 0..=2 {
            if board[i][0] == board[i][1]
                && board[i][2] == board[i][1]
                && board[i][0] != Symbol::Nil
            {
                self.winner = make_pl(board[i][0]);
            }
        }
        for j in 0..=2 {
            if board[0][j] == board[1][j]
                && board[1][j] == board[2][j]
                && board[0][j] != Symbol::Nil
            {
                self.winner = make_pl(board[0][j]);
            }
        }
    }
    pub fn make_move(&mut self, x: usize, y: usize, player: u8) {
        if player == 1 {
            self.board[x][y] = Symbol::Cross;
        } else {
            self.board[x][y] = Symbol::Zero;
        }
        self.check();
    }
    pub fn render(&self) {
        clear_terminal();
        println!("-------------");
        for row in &self.board {
            for cell in row {
                let symbol = match cell {
                    Symbol::Nil => ' ',   // Пустая клетка
                    Symbol::Cross => 'X', // Крестик
                    Symbol::Zero => 'O',  // Нолик
                };
                print!("| {} ", symbol);
            }
            println!("|");
            println!("-------------");
        }
    }
}

fn input() -> (usize, usize) {
    println!("Введите два числа (x y):");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let values: Vec<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok()) // Используем filter_map для безопасного парсинга
        .collect();

    if values.len() != 2 {
        println!("Ошибка: Пожалуйста, введите ровно два числа.");
        // Возвращаем значения по умолчанию или обрабатываем ошибку по-другому
        return (0, 0); // Или можно использовать Result для более гибкой обработки
    }

    (values[0], values[1])
}

fn main() -> Result<(), u8> {
    clear_terminal();
    let mut board = Game::new();
    let mut moves: u8 = 0;
    board.render();
    loop {
        let (mut y, mut x) = input();
        while y > 3 || y < 1 || x > 3 || x < 1 || board.board[x - 1][y - 1] != Symbol::Nil {
            println!("Type a correct index");
            (y, x) = input();
        }
        if moves % 2 == 0 {
            board.make_move(x - 1, y - 1, 1);
        } else {
            board.make_move(x - 1, y - 1, 2);
        }
        board.render();

        match board.winner {
            Player::First => {
                //clear_terminal();
                println!("Выиграли крестики");
                return Ok::<(), u8>(());
            }
            Player::Second => {
                //clear_terminal();
                println!("Выиграли нолики");
                return Ok(());
            }
            Player::Nil => {}
        }
        moves += 1;
        if moves == 9 {
            println!("Ничья");
            return Ok(());
        }
    }
}
