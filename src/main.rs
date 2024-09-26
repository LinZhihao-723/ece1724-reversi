mod game_manager;

use game_manager::GameManager;
use game_manager::Position;

fn main() {
    let mut game_manager = GameManager::new();
    game_manager.print_board(true);
    game_manager.make_move(&Position::new(3, 2));
    if (game_manager.check_game_over()) {
        return;
    }
    game_manager.print_board(true);
}
