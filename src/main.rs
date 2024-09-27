mod game_manager;

fn main() {
    let mut mgr = game_manager::GameManager::new();
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
        if false == mgr.advance_to_next_turn() {
            mgr.print_game_result();
            break;
        }
    }
}
