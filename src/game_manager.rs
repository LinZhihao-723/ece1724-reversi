const BOARD_DIMENSION: usize = 8;
const TURN_WHITE: bool = true;
const TURN_BLACK: bool = false;

const PIECE_WHITE: char = 'W';
const PIECE_BLACK: char = 'B';
const PIECE_EMPTY: char = '.';

const BOARD_LABEL: [char; BOARD_DIMENSION] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub struct Move {
    pub m_row: usize,
    pub m_col: usize,
}

pub struct GameManager {
    m_board: [[char; BOARD_DIMENSION]; BOARD_DIMENSION],
    m_turn: bool,
    m_num_white: usize,
    m_num_black: usize,
    m_next_available_moves: Vec<Move>,
}

impl GameManager {
    pub fn new() -> GameManager {
        let mut board = [[PIECE_EMPTY; BOARD_DIMENSION]; BOARD_DIMENSION];
        board[3][3] = PIECE_WHITE;
        board[4][4] = PIECE_WHITE;
        board[4][3] = PIECE_BLACK;
        board[3][4] = PIECE_BLACK;

        GameManager {
            m_board: board,
            m_turn: TURN_BLACK,
            m_num_white: 2,
            m_num_black: 2,
            m_next_available_moves: Vec::new(),
        }
    }

    pub fn print_board(&self) {
        println!("  abcedfgh");
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
}
