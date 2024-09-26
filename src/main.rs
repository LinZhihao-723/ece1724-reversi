mod game_manager;

use game_manager::GameManager;

fn main() {
    let mut game_manager = GameManager::new();
    game_manager.print_board();
}
