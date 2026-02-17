use std::{time::Duration, vec};

type Point = (usize, usize);
type Board = Vec<Vec<Cell>>;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Game {
    // board[x][y]
    current_board: Board,
    alt_board: Board,
    tick: usize,
    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        let board: Board = (0..width)
            .map(|_| (0..height).map(|_| Cell::Dead).collect())
            .collect();

        Self {
            current_board: board.clone(),
            alt_board: board,
            tick: 0,
            width,
            height,
        }
    }

    pub fn engender(&mut self, cells: &[Point]) {
        cells
            .into_iter()
            .for_each(|(x, y)| self.current_board[*x][*y] = Cell::Alive);
    }

    pub fn count_neighbors(&self, point: Point) -> u8 {
        let x = point.0 as isize;
        let y = point.1 as isize;

        let mut count = 0;

        (-1 as isize..=1).for_each(|dx| {
            if x + dx < 0 || x + dx >= self.width as isize {
                return;
            }

            (-1 as isize..=1).for_each(|dy| {
                if (dx == 0 && dy == 0) || y + dy < 0 || y + dy >= self.height as isize {
                    return;
                }

                if self.current_board[(x + dx) as usize][(y + dy) as usize] == Cell::Alive {
                    count += 1;
                }
            });
        });

        count
    }

    pub fn tick(&mut self) {
        for (x, column) in self.current_board.iter().enumerate() {
            for (y, _cell) in column.iter().enumerate() {
                let alive_neighbors = self.count_neighbors((x, y));

                self.alt_board[x][y] = match (self.current_board[x][y].clone(), alive_neighbors) {
                    // Underpopulation for Live cells
                    // Nothing for Dead cells
                    (_, 0..2) => Cell::Dead,
                    (Cell::Alive, 2..=3) => Cell::Alive,

                    (Cell::Dead, 2) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,

                    // Overpopulation for Live cells
                    // Nothing for Dead cells
                    (_, _) => Cell::Dead,
                };
            }
        }

        self.current_board = self.alt_board.clone();

        self.tick += 1;
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.height * self.width * 4);

        s.push_str(format!("Tick: {}\n", self.tick).as_str());
        for y in 0..self.height {
            let mut row = String::with_capacity(self.width * 3);
            row.push('|');
            for x in 0..self.width {
                let cell = match self.current_board[x][y] {
                    Cell::Alive => 'x',
                    Cell::Dead => ' ',
                };
                row.push(cell);
                row.push('|');
            }
            s.push_str(&row);
            s.push('\n');
        }

        s
    }
}

fn main() {
    let mut game = Game::new(20, 20);
    game.engender(&[(0, 2), (1, 2), (0, 3), (1, 4), (2, 3), (3, 2), (2, 2)]);

    loop {
        let board_str = game.to_string();
        println!("{}", board_str);
        game.tick();
        if game
            .current_board
            .iter()
            .flatten()
            .all(|cell| *cell == Cell::Dead)
        {
            println!("Life ended after {} ticks", game.tick);
            break;
        }
        std::thread::sleep(Duration::new(0, 100_000_000));
    }
}
