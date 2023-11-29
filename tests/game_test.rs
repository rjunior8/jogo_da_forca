#[cfg(test)]
mod tests {
    use jogo_da_forca::game::game::{Game, HangmanGame};

    fn get_game_instance() -> HangmanGame {
        let game: HangmanGame = Game::new("abcDEF", Vec::new(), Vec::new(), Vec::new(), 0);
        game
    }

    #[test]
    fn word_contains_letter_when_contains_it_passes() {
        let letter: String = String::from("a");
        let game = get_game_instance();
        assert_eq!(true, game.word_contains_letter(letter));
    }

    #[test]
    fn word_contains_letter_when_does_not_contain_it_passes() {
        let letter: String = String::from("x");
        let game = get_game_instance();
        assert_eq!(false, game.word_contains_letter(letter));
    }

    #[test]
    fn get_occurrences_when_is_not_empty_it_passes() {
        let letter: String = String::from("a");
        let game = get_game_instance();
        assert_eq!(false, game.get_occurrences(letter).is_empty());
    }

    #[test]
    fn get_occurrences_when_is_in_the_first_position_it_passes() {
        let letter: String = String::from("a");
        let game = get_game_instance();
        assert_eq!(vec![0], game.get_occurrences(letter));
    }

    #[test]
    fn get_occurrences_when_is_in_the_second_position_it_passes() {
        let letter: String = String::from("b");
        let game = get_game_instance();
        assert_eq!(vec![1], game.get_occurrences(letter));
    }

    #[test]
    fn get_occurrences_when_is_in_the_third_position_it_passes() {
        let letter: String = String::from("c");
        let game = get_game_instance();
        assert_eq!(vec![2], game.get_occurrences(letter));
    }

    #[test]
    fn get_occurrences_when_is_empty_it_passes() {
        let letter: String = String::from("x");
        let game = get_game_instance();
        assert_eq!(true, game.get_occurrences(letter).is_empty());
    }

    #[test]
    fn get_occurrences_when_input_is_upper_case_and_contained_word_is_lower_case_it_passes() {
        let letter: String = String::from("A");
        let game = get_game_instance();
        assert_eq!(false, game.get_occurrences(letter).is_empty());
    }

    #[test]
    fn get_occurrences_when_input_is_upper_case_and_contained_word_is_lower_case_is_in_the_first_position_it_passes() {
        let letter: String = String::from("A");
        let game = get_game_instance();
        assert_eq!(vec![0], game.get_occurrences(letter));
    }

    #[test]
    fn get_occurrences_when_input_is_lower_case_and_contained_word_is_upper_case_it_passes() {
        let letter: String = String::from("d");
        let game = get_game_instance();
        assert_eq!(false, game.get_occurrences(letter).is_empty());
    }

    #[test]
    fn get_occurrences_when_input_is_lower_case_and_contained_word_is_upper_case_is_in_the_fourth_position_it_passes() {
        let letter: String = String::from("d");
        let game = get_game_instance();
        assert_eq!(vec![3], game.get_occurrences(letter));
    }
}
