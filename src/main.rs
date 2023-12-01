use jogo_da_forca::random_word::random_word::get_random_word;
use jogo_da_forca::game::game::{Game, HangmanGame};

fn main() {
    let mut game: HangmanGame = Game::new(get_random_word(), Vec::new(), Vec::new(), Vec::new(), 0);
    game.play();
}
