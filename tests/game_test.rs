#[cfg(test)]
mod tests {
    use std::fmt::Arguments;
    use jogo_da_forca::{word_contains_letter, get_occurrences, get_blank_chars, print_sequence};
    // use jogo_da_forca::game::game::{Game, HangmanGame};
    use jogo_da_forca::logger::logger::Logger;

    #[derive(Default)]
    struct DummyLogger(Vec<String>);
    impl Logger for DummyLogger {
        fn print(&mut self, value: Arguments<'_>) {
            self.0.push(value.to_string());
        }
    }

    // fn get_game_instance() -> HangmanGame {
    //     let game: HangmanGame = Game::new("abcDEF", Vec::new(), Vec::new(), Vec::new(), 0);
    //     game
    // }

    #[test]
    fn word_contains_letter_when_contains_it_passes() {
        let letter: String = String::from("a");
        let expected: bool = true;
        let actual = word_contains_letter(letter, String::from("abc"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn word_contains_letter_when_does_not_contain_it_passes() {
        let letter: String = String::from("x");
        let expected: bool = false;
        let actual = word_contains_letter(letter, String::from("abc"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_is_not_empty_it_passes() {
        let letter: String = String::from("a");
        let expected: bool = false;
        let actual = get_occurrences(letter, "abc").is_empty();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_is_in_the_first_position_it_passes() {
        let letter: String = String::from("a");
        let expected = vec![0];
        let actual = get_occurrences(letter, "abc");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_is_in_the_second_position_it_passes() {
        let letter: String = String::from("b");
        let expected = vec![1];
        let actual = get_occurrences(letter, "abc");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_is_in_the_third_position_it_passes() {
        let letter: String = String::from("c");
        let expected = vec![2];
        let actual = get_occurrences(letter, "abc");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_there_is_no_is_empty_it_passes() {
        let letter: String = String::from("x");
        let expected: bool = true;
        let actual = get_occurrences(letter, "abc").is_empty();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_input_is_upper_case_and_contained_word_is_lower_case_it_passes() {
        let letter: String = String::from("A");
        let expected: bool = false;
        let actual = get_occurrences(letter, "abcDEF").is_empty();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_input_is_upper_case_and_contained_word_is_lower_case_is_in_the_first_position_it_passes() {
        let letter: String = String::from("A");
        let expected = vec![0];
        let actual = get_occurrences(letter, "abcDEF");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_input_is_lower_case_and_contained_word_is_upper_case_it_passes() {
        let letter: String = String::from("d");
        let expected: bool = false;
        let actual = get_occurrences(letter, "abcDEF").is_empty();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_occurrences_when_input_is_lower_case_and_contained_word_is_upper_case_is_in_the_fourth_position_it_passes() {
        let letter: String = String::from("d");
        let expected = vec![3];
        let actual = get_occurrences(letter, "abcDEF");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_blank_chars_when_size_is_not_zero_validate_is_not_empty_it_passes() {
        let size: usize = 5;
        let expected: bool = false;
        let actual = get_blank_chars(size).is_empty();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_blank_chars_when_size_is_five_returns_a_vector_with_five_blanks_it_passes() {
        let size: usize = 5;
        let expected: usize = 5;
        let actual = get_blank_chars(size).len();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_blank_chars_when_size_is_three_returns_three_blanks_it_passes() {
        let size: usize = 3;
        let expected = vec![" ", " ", " "];
        let actual = get_blank_chars(size);
        assert_eq!(expected, actual);
    }

    #[test]
    fn print_sequence_when_is_a_sequence_of_three_underscores_seperated_by_space_it_passes() {
        let mut logger = DummyLogger::default();
        let expected = format!("_ _ _ ");
        print_sequence(String::from(" "), vec!["_".to_string(), "_".to_string(), "_".to_string()], &mut logger);
        assert_eq!(expected.as_str(), logger.0[0]);
    }
}
