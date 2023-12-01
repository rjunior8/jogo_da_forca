#[cfg(test)]
mod tests {
    use std::fmt::Arguments;
    use jogo_da_forca::{word_contains_letter, get_occurrences, get_blank_chars, print_sequence, print_gibbet, print_head, print_head_and_right_arm, print_head_right_arm_and_left_arm, print_head_right_arm_left_arm_and_stem, print_head_right_arm_left_arm_stem_and_right_leg, print_head_right_arm_left_arm_stem_right_leg_and_left_leg, print_default, print_gibbet_status, win, lost, end, prompt};
    use jogo_da_forca::constants::gibbet_phases::GIBBET;
    // use jogo_da_forca::game::game::{Game, HangmanGame};
    use jogo_da_forca::logger::logger::Logger;
    use jogo_da_forca::puppet::gibbet::Gibbet::{Head, HeadAndRightArm, HeadRightArmAndLeftArm, HeadRightArmLeftArmAndStem, HeadRightArmLeftArmStemAndRightLeg, HeadRightArmLeftArmStemRightLegAndLeftLeg};

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

    #[test]
    fn prompt_it_passes() {
        let input = b"a";
        let mut output = Vec::new();
        let answer = prompt(&input[..], &mut output, "Type a letter: ");
        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!("Type a letter: ", output);
        assert_eq!("a", answer);
    }

    #[test]
    fn print_gibbet_properly() {
        let mut logger = DummyLogger::default();
        let expected = GIBBET;
        print_gibbet(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_head_properly() {
        let mut logger = DummyLogger::default();
        let expected = Head;
        print_head(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_head_and_right_arm_properly() {
        let mut logger = DummyLogger::default();
        let expected = HeadAndRightArm;
        print_head_and_right_arm(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_head_right_arm_and_left_arm_properly() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmAndLeftArm;
        print_head_right_arm_and_left_arm(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_head_right_arm_left_arm_and_stem_properly() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmLeftArmAndStem;
        print_head_right_arm_left_arm_and_stem(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_head_right_arm_left_arm_stem_and_right_leg_properly() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmLeftArmStemAndRightLeg;
        print_head_right_arm_left_arm_stem_and_right_leg(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_head_right_arm_left_arm_stem_right_leg_and_left_leg_properly() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmLeftArmStemRightLegAndLeftLeg;
        print_head_right_arm_left_arm_stem_right_leg_and_left_leg(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_default_properly() {
        let mut logger = DummyLogger::default();
        let expected = "Invalid";
        print_default(&mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_zero_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = GIBBET;
        print_gibbet_status(0, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_one_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = Head;
        print_gibbet_status(1, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_two_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = HeadAndRightArm;
        print_gibbet_status(2, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_three_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmAndLeftArm;
        print_gibbet_status(3, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_four_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmLeftArmAndStem;
        print_gibbet_status(4, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_five_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmLeftArmStemAndRightLeg;
        print_gibbet_status(5, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_six_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = HeadRightArmLeftArmStemRightLegAndLeftLeg;
        print_gibbet_status(6, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn print_gibbet_status_when_attempt_is_invalid_prints_gibbet() {
        let mut logger = DummyLogger::default();
        let expected = "Invalid";
        print_gibbet_status(7, &mut logger);
        assert_eq!(expected.to_string().as_str(), logger.0[0]);
    }

    #[test]
    fn win_when_won_it_passes() {
        let word = "Hello";
        let formed_word_by_hits: Vec<String> = vec!["h".to_string(), "e".to_string(), "l".to_string(), "l".to_string(), "o".to_string()];
        let expected: bool = true;
        let actual = win(word, &formed_word_by_hits);
        assert_eq!(expected, actual);
    }

    #[test]
    fn win_when_not_won_it_passes() {
        let word = "Hello";
        let formed_word_by_hits: Vec<String> = vec!["e".to_string(), "l".to_string(), "l".to_string(), "o".to_string()];
        let expected: bool = false;
        let actual = win(word, &formed_word_by_hits);
        assert_eq!(expected, actual);
    }

    #[test]
    fn lost_when_lose() {
        let attempts: u8 = 6;
        let expected: bool = true;
        let actual = lost(attempts);
        assert_eq!(expected, actual);
    }

    #[test]
    fn lost_when_not_lose() {
        let attempts: u8 = 1;
        let expected: bool = false;
        let actual = lost(attempts);
        assert_eq!(expected, actual);
    }

    #[test]
    fn end_when_won() {
        let mut logger = DummyLogger::default();
        let status = "WON";
        let word = "Hello";
        let expected_1 = String::from("You WON");
        let expected_2 = String::from("\nThe word is: Hello");
        end(status, word, &mut logger);
        assert_eq!(expected_1, logger.0[0]);
        assert_eq!(expected_2, logger.0[1]);
    }

    #[test]
    fn end_when_lose() {
        let mut logger = DummyLogger::default();
        let status = "LOSE";
        let word = "Hello";
        let expected_1 = String::from("You LOSE");
        let expected_2 = String::from("\nThe word is: Hello");
        end(status, word, &mut logger);
        assert_eq!(expected_1, logger.0[0]);
        assert_eq!(expected_2, logger.0[1]);
    }
}
