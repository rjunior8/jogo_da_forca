use std::fmt::Debug;

use crate::logger::logger::{StdoutLogger};
use crate::{clear_screen, end_of_game, get_blank_chars, get_occurrences, lost, print_gibbet_status, print_sequence, read_user_input, win, word_contains_letter};

#[derive(Debug, Clone)]
pub struct HangmanGame {
    word: &'static str,
    formed_word_by_hits: Vec<String>,
    hits: Vec<String>,
    errors: Vec<String>,
    attempts: u8,
}

pub trait Game {
    fn new(word: &'static str, formed_word_by_hits: Vec<String>, hits: Vec<String>, errors: Vec<String>, attempts: u8) -> Self;
    fn start(&mut self) -> ();

    fn add_letter_to_hits(&mut self, letter: String) -> ();

    fn add_letter_to_empty_word(&mut self, letter: String) -> ();

    fn add_letter_to_errors(&mut self, letter: String) -> ();

    fn print_hits(&mut self) -> ();

    fn print_errors(&mut self) -> ();

    fn correct_answer(&mut self, input: String) -> ();

    fn wrong_answer(&mut self, input: String) -> ();

    fn print_game(&mut self) -> ();

    fn verify_answer(&mut self, input: String) -> ();

    fn prompt_user(&mut self) -> ();

    fn run(&mut self) -> ();
}

impl Game for HangmanGame {
    fn new(word: &'static str, formed_word_by_hits: Vec<String>, hits: Vec<String>, errors: Vec<String>, attempts: u8) -> HangmanGame {
        HangmanGame {word, formed_word_by_hits, hits, errors, attempts}
    }

    fn start(&mut self) -> () {
        clear_screen();
        self.formed_word_by_hits = get_blank_chars(self.word.len());
        self.run();
    }

    fn add_letter_to_hits(&mut self, letter: String) -> () {
        self.hits.push(letter);
    }

    fn add_letter_to_empty_word(&mut self, letter: String) -> () {
        let occurrences: Vec<usize> = get_occurrences(letter.clone(), self.word);
        for index in occurrences.iter() {
            self.formed_word_by_hits[*index] = letter.clone();
        }
    }

    fn add_letter_to_errors(&mut self, letter: String) -> () {
        self.errors.push(letter);
    }

    fn print_hits(&mut self) -> () {
        let blank_chars: Vec<String> = get_blank_chars(self.word.len());
        let _self = &mut *self.formed_word_by_hits;
        let mut std_out_logger = StdoutLogger;
        print_sequence(" ".to_string(), _self.to_vec(), &mut std_out_logger);
        print_sequence("_".to_string(), blank_chars, &mut std_out_logger);
    }

    fn print_errors(&mut self) -> () {
        let _self = &mut *self.errors;
        let mut std_out_logger = StdoutLogger;
        print_sequence(" ".to_string(), _self.to_vec(), &mut std_out_logger);
    }

    fn correct_answer(&mut self, input: String) -> () {
        self.add_letter_to_hits(input.clone());
        self.add_letter_to_empty_word(input.clone());
    }

    fn wrong_answer(&mut self, input: String) -> () {
        self.attempts += 1;
        self.add_letter_to_errors(input);
    }

    fn print_game(&mut self) -> () {
        let mut std_out_logger = StdoutLogger;
        println!("\n\t\tJogo da Forca");
        self.print_errors();
        println!("\n");
        print_gibbet_status(self.attempts, &mut std_out_logger);
        println!("\n");
        self.print_hits();
        println!("\n");
    }

    fn verify_answer(&mut self, input: String) -> () {
        if word_contains_letter(input.to_lowercase().to_string(), self.word.to_lowercase()) {
            self.correct_answer(input.to_lowercase().to_string());
        } else {
            self.wrong_answer(input.to_lowercase().to_string());
        }
    }

    fn prompt_user(&mut self) -> () {
        let mut input = read_user_input();
        input.pop();
        self.verify_answer(input);
    }

    fn run(&mut self) -> () {
        while !win(&self.word, &self.formed_word_by_hits) && !lost(self.attempts) {
            self.print_game();
            self.prompt_user();
            clear_screen();
        }
        end_of_game(self.word, &self.formed_word_by_hits, self.attempts);
    }
}
