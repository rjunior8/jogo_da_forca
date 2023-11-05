use jogo_da_forca::random_word;
use crate::game::game::{Game, GameInterface};

mod constants {
    pub mod attempts;
    pub mod gibbet_phases;
}

mod game {
    pub mod game;
}

mod puppet {
    pub mod gibbet;
}

fn main() {
    let hits: Vec<String> = Vec::new();
    let errors: Vec<String> = Vec::new();
    let formed_word_by_hits: Vec<String> = Vec::new();
    let word = random_word::random_word::get_random_word();
    let attempts: i8 = 0;
    let mut game: Game = GameInterface::new(word, formed_word_by_hits, hits, errors, attempts);
    game.start();
}
