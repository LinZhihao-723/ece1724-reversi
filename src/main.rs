mod game_manager;

use game_manager::GameManager;

fn main() {
    let mut mgr = GameManager::new();
    loop {
        mgr.print_board(false);
        match mgr.ask_for_input() {
            Ok(next_move) => {
                mgr.make_move(&next_move);
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
        if mgr.advance_to_next_turn() {
            mgr.print_game_result();
            break;
        }
    }
}
