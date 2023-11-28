#[cfg(test)]
mod tests {
    use jogo_da_forca::game::game::{Game, GameInterface};

    fn get_game_instance() -> Game {
        let hits: Vec<String> = Vec::new();
        let errors: Vec<String> = Vec::new();
        let formed_word_by_hits: Vec<String> = Vec::new();
        let word = "abc";
        let attempts: u8 = 0;
        let game: Game = GameInterface::new(word, formed_word_by_hits, hits, errors, attempts);
        game
    }

    #[test]
    fn word_contains_letter_when_contains_it_passes() {
        let letter: String = String::from("a");
        let game = get_game_instance();
        assert_eq!(true, game.word_contains_letter(letter));
    }

    #[test]
    fn word_contains_letter_when_does_not_contain_it_fails() {
        let letter: String = String::from("x");
        let game = get_game_instance();
        assert_eq!(false, game.word_contains_letter(letter));
    }
}
