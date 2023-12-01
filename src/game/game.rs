use std::fmt::Debug;

use crate::logger::logger::{StdoutLogger};
use crate::constants::attempts::SIX;
use crate::{get_blank_chars, get_occurrences, print_gibbet_status, print_sequence, read_user_input, word_contains_letter};

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

    fn end(&self, status: &str) -> ();

    fn clear_screen() -> ();

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

    fn win(&self) -> bool;

    fn lost(&self) -> bool;

    fn run(&mut self) -> ();

    fn end_of_game(&self) -> ();
}

impl Game for HangmanGame {
    fn new(word: &'static str, formed_word_by_hits: Vec<String>, hits: Vec<String>, errors: Vec<String>, attempts: u8) -> HangmanGame {
        HangmanGame {word, formed_word_by_hits, hits, errors, attempts}
    }

    fn start(&mut self) -> () {
        HangmanGame::<>::clear_screen();
        self.formed_word_by_hits = get_blank_chars(self.word.len());
        self.run();
    }

    fn end(&self, status: &str) -> () {
        println!("You {}", status);
        println!("\nThe word is: {}", self.word);
    }

    fn clear_screen() -> () {
        clearscreen::clear().unwrap();
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
        println!("\n\t\tJogo da Forca");
        self.print_errors();
        println!("\n");
        print_gibbet_status(self.attempts);
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

    fn win(&self) -> bool {
        let word: String = self.word.to_lowercase();
        word.eq(&self.formed_word_by_hits.concat())
    }

    fn lost(&self) -> bool {
        self.attempts == SIX
    }

    fn run(&mut self) -> () {
        while !self.win() && !self.lost() {
            self.print_game();
            self.prompt_user();
            HangmanGame::<>::clear_screen();
        }
        self.end_of_game();
    }

    fn end_of_game(&self) -> () {
        if self.win() {
            self.end("WON");
        } else if self.lost() {
            self.end("LOSE");
        }
    }
}
