use bracket_lib::prelude::*;

use flappy::game_state::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()
        .unwrap();

    main_loop(context, State::default())
}
