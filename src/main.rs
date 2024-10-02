mod game_manager;

fn main() {
    let mut mgr = game_manager::GameManager::new();
    mgr.print_board(false);
    loop {
        match mgr.ask_for_input() {
            Ok(next_move) => {
                mgr.make_move(&next_move);
                mgr.print_board(false);
            }
            Err(err) => {
                println!("{}", err);
                mgr.print_board(false);
                continue;
            }
        }
        if false == mgr.advance_to_next_turn() {
            mgr.print_game_result();
            break;
        }
    }
}
