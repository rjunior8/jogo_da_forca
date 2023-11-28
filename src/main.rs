use jogo_da_forca::random_word::random_word::get_random_word;
use jogo_da_forca::game::game::{Game, GameInterface};

fn main() {
    let hits: Vec<String> = Vec::new();
    let errors: Vec<String> = Vec::new();
    let formed_word_by_hits: Vec<String> = Vec::new();
    let word = get_random_word();
    let attempts: u8 = 0;
    let mut game: Game = GameInterface::new(word, formed_word_by_hits, hits, errors, attempts);
    game.start();
}
