use std::error::Error;

use quadri::game::Game;

// Readme for getting started/playing
// TODO: isolate winner state and turn state
// TODO: better way to print winning quadris...
// TODO: Tests
// TODO: GUI
// TODO: Handle errors intelligently, none of this String mess

// What this looks like:
// -- red/blue | capital/lowercase | X/O | underli
// x_ x- X_ X-   <- red
// x_ x- X_ X-   <- blue
// o_ o- O_ O-   <- red
// o_ o- O_ O-   <- blue

// -- red/blue | capital/lowercase | X/O | underline/bold
// +----+----+----+----+
// | x_ | x- | X_ | X- |  <- red
// +----+----+----+----+
// | x_ | x- | X_ | X- |  <- blue
// +----+----+----+----+
// | o_ | o- | O_ | O- |  <- red
// +----+----+----+----+
// | o_ | o- | O_ | O- |  <- blue
// +----+----+----+----+

/// yell  grey  yell  grey
// +---+---+---+---+
// | ○ | ○ | ● | ● |  <- red
// +---+---+---+---+
// | ○ | ○ | ● | ● |  <- blue
// +---+---+---+---+
// | ▯ | ▯ | ▮ | ▮ |  <- red
// +---+---+---+---+
// | ▯ | ▯ | ▮ | ▮ |  <- blue
// +---+---+---+---+

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new_cli_game();
    game.play_game()
}
