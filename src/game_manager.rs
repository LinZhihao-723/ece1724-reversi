const BOARD_DIMENSION: usize = 8;
const TURN_WHITE: bool = true;
const TURN_BLACK: bool = false;

const PIECE_WHITE: char = 'W';
const PIECE_BLACK: char = 'B';
const PIECE_EMPTY: char = '.';

const BOARD_LABEL: [char; BOARD_DIMENSION] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const DELTAS: [[i8; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

type GameBoardArray = [[char; BOARD_DIMENSION]; BOARD_DIMENSION];
use std::io::Write;

pub struct Position {
    pub m_row: i8,
    pub m_col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Position {
        Position {
            m_row: row,
            m_col: col,
        }
    }

    pub fn is_in_bound(&self) -> bool {
        self.m_row >= 0
            && self.m_row < BOARD_DIMENSION as i8
            && self.m_col >= 0
            && self.m_col < BOARD_DIMENSION as i8
    }

    pub fn out_of_bounds(&self) -> bool {
        false == self.is_in_bound()
    }
}

struct GameBoard {
    m_board: GameBoardArray,
}

impl GameBoard {
    fn new() -> GameBoard {
        let mut board = [[PIECE_EMPTY; BOARD_DIMENSION]; BOARD_DIMENSION];
        board[3][3] = PIECE_WHITE;
        board[4][4] = PIECE_WHITE;
        board[4][3] = PIECE_BLACK;
        board[3][4] = PIECE_BLACK;

        GameBoard { m_board: board }
    }

    fn at(&self, position: &Position) -> char {
        self.m_board[position.m_row as usize][position.m_col as usize]
    }

    fn is_valid_direction(
        &self,
        curr_pos: &Position,
        row_delta: i8,
        col_delta: i8,
        color_self: char,
        color_opponent: char,
    ) -> bool {
        let mut next_pos = Position::new(curr_pos.m_row + row_delta, curr_pos.m_col + col_delta);
        if next_pos.out_of_bounds() || color_opponent != self.at(&next_pos) {
            return false;
        }
        loop {
            next_pos.m_row += row_delta;
            next_pos.m_col += col_delta;
            if next_pos.out_of_bounds() {
                return false;
            }
            let curr_piece = self.at(&next_pos);
            if color_self == curr_piece {
                return true;
            }
            if curr_piece == PIECE_EMPTY {
                return false;
            }
        }
    }

    fn is_available_move(&self, pos: &Position, color_self: char, color_opponent: char) -> bool {
        if PIECE_EMPTY != self.at(pos) {
            return false;
        }

        for delta in &DELTAS {
            let row_delta = delta[0];
            let col_delta = delta[1];
            if self.is_valid_direction(pos, col_delta, row_delta, color_self, color_opponent) {
                return true;
            }
        }

        false
    }

    fn print(&self) {
        print!("  ");
        for &c in &BOARD_LABEL {
            print!("{}", c);
        }
        print!("\n");

        let mut row_idx: usize = 0;
        for row in &self.m_board {
            print!("{} ", BOARD_LABEL[row_idx]);
            for &place in row {
                print!("{}", place);
            }
            print!("\n");
            row_idx += 1;
        }
    }

    fn print_with_next_available_moves(&self, next_available_moves: &BoardLUT) {
        print!("  ");
        for &c in &BOARD_LABEL {
            print!("{}", c);
        }
        print!("\n");

        for row in 0..BOARD_DIMENSION {
            print!("{} ", BOARD_LABEL[row]);
            for col in 0..BOARD_DIMENSION {
                let pos = Position::new(row as i8, col as i8);
                if next_available_moves.contains(&pos) {
                    print!("*");
                } else {
                    print!("{}", self.m_board[row][col]);
                }
            }
            print!("\n");
        }
    }

    fn make_move(&mut self, pos: &Position, color_self: char, color_opponent: char) -> usize {
        let mut num_pieces_flipped: usize = 0;
        for delta in &DELTAS {
            let row_delta = delta[0];
            let col_delta = delta[1];
            if self.is_valid_direction(pos, row_delta, col_delta, color_self, color_opponent) {
                let mut next_pos = Position::new(pos.m_row + row_delta, pos.m_col + col_delta);
                while color_self != self.at(&next_pos) {
                    self.m_board[next_pos.m_row as usize][next_pos.m_col as usize] = color_self;
                    num_pieces_flipped += 1;
                    next_pos.m_row += row_delta;
                    next_pos.m_col += col_delta;
                }
            }
        }

        self.m_board[pos.m_row as usize][pos.m_col as usize] = color_self;
        num_pieces_flipped
    }

    fn generate_next_available_moves(&self, turn: bool) -> BoardLUT {
        let mut next_available_moves = BoardLUT::new();
        let (color_self, color_opponent) = get_color_from_turn(turn);

        for row in 0..BOARD_DIMENSION {
            for col in 0..BOARD_DIMENSION {
                let pos = Position::new(row as i8, col as i8);
                if false == self.is_available_move(&pos, color_self, color_opponent) {
                    continue;
                }
                next_available_moves.insert(&pos);
            }
        }
        next_available_moves
    }
}

struct BoardLUT {
    m_lut: [[bool; BOARD_DIMENSION]; BOARD_DIMENSION],
    m_num_set: usize,
}

impl BoardLUT {
    fn new() -> BoardLUT {
        BoardLUT {
            m_lut: [[false; BOARD_DIMENSION]; BOARD_DIMENSION],
            m_num_set: 0,
        }
    }

    fn insert(&mut self, pos: &Position) {
        self.m_lut[pos.m_row as usize][pos.m_col as usize] = true;
        self.m_num_set += 1;
    }

    fn contains(&self, pos: &Position) -> bool {
        self.m_lut[pos.m_row as usize][pos.m_col as usize]
    }

    fn is_empty(&self) -> bool {
        0 == self.m_num_set
    }
}

pub struct GameManager {
    m_board: GameBoard,
    m_turn: bool,
    m_game_over: bool,
    m_num_white: usize,
    m_num_black: usize,
    m_next_available_moves: BoardLUT,
}

impl GameManager {
    // Factory function
    pub fn new() -> GameManager {
        const INITIAL_TURN: bool = TURN_BLACK;
        let game_board = GameBoard::new();
        let next_available_moves = game_board.generate_next_available_moves(INITIAL_TURN);

        GameManager {
            m_board: game_board,
            m_turn: INITIAL_TURN,
            m_game_over: false,
            m_num_white: 2,
            m_num_black: 2,
            m_next_available_moves: next_available_moves,
        }
    }

    pub fn print_board(&self, print_available_moves: bool) {
        if print_available_moves {
            self.m_board
                .print_with_next_available_moves(&self.m_next_available_moves);
        } else {
            self.m_board.print();
        }
    }

    pub fn make_move(&mut self, pos: &Position) -> bool {
        if false == self.m_next_available_moves.contains(pos) {
            return false;
        }

        let (color_self, color_opponent) = get_color_from_turn(self.m_turn);
        let num_piece_flipped = self.m_board.make_move(pos, color_self, color_opponent);
        if 0 == num_piece_flipped {
            return false;
        }
        self.update_count(num_piece_flipped);

        true
    }

    pub fn advance_to_next_turn(&mut self) -> bool {
        if self.m_game_over {
            return false;
        }
        let (curr_color, next_color) = get_color_from_turn(self.m_turn);

        self.switch_turn();
        self.m_next_available_moves = self.m_board.generate_next_available_moves(self.m_turn);
        if false == self.m_next_available_moves.is_empty() {
            return true;
        }
        println!("{} player has no valid move.", next_color);

        self.switch_turn();
        self.m_next_available_moves = self.m_board.generate_next_available_moves(self.m_turn);
        if false == self.m_next_available_moves.is_empty() {
            return true;
        }
        println!("{} player has no valid move.", curr_color);

        self.m_game_over = true;
        false
    }

    pub fn ask_for_input(&mut self) -> Result<Position, String> {
        let (color_self, _) = get_color_from_turn(self.m_turn);
        print!("Enter move for colour {} (RowCol): ", { color_self });
        std::io::stdout().flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read lint.");
        if 3 != input.len() {
            return Err(String::from("Invalid move. Try again."));
        }

        let row_char = input.chars().nth(0).unwrap();
        let col_char = input.chars().nth(1).unwrap();
        if false == is_valid_pos_char(row_char) || false == is_valid_pos_char(col_char) {
            return Err(String::from("Invalid move. Try again."));
        }

        let pos = Position::new(row_char as i8 - 'a' as i8, col_char as i8 - 'a' as i8);
        if false == self.m_next_available_moves.contains(&pos) {
            return Err(String::from("Invalid move. Try again."));
        }

        Ok(pos)
    }

    pub fn print_game_result(&self) {
        if false == self.m_game_over {
            println!("Game isn't over.");
            return;
        }
        if self.m_num_black == self.m_num_white {
            println!("Draw!");
        } else if self.m_num_black > self.m_num_white {
            println!(
                "Black wins by {} points!",
                self.m_num_black - self.m_num_white
            );
        } else {
            println!(
                "White wins by {} points!",
                self.m_num_white - self.m_num_black
            );
        }
    }

    fn switch_turn(&mut self) {
        self.m_turn = !self.m_turn;
    }

    fn update_count(&mut self, num_piece_flipped: usize) {
        if TURN_WHITE == self.m_turn {
            self.m_num_white += num_piece_flipped + 1;
            self.m_num_black -= num_piece_flipped;
        } else {
            self.m_num_white -= num_piece_flipped;
            self.m_num_black += num_piece_flipped + 1;
        }
    }
}

fn get_color_from_turn(turn: bool) -> (char, char) {
    if TURN_WHITE == turn {
        return (PIECE_WHITE, PIECE_BLACK);
    }
    (PIECE_BLACK, PIECE_WHITE)
}

fn is_valid_pos_char(c: char) -> bool {
    'a' <= c && c <= 'h'
}
