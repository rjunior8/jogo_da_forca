use std::fmt::Debug;
use std::io::stdin;

use crate::puppet::gibbet::Gibbet;
use crate::constants::attempts::SIX;

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

    fn word_contains_letter(&self, letter: String) -> bool;

    fn get_occurrences(&self, letter: String) -> Vec<usize>;

    fn add_letter_to_hits(&mut self, letter: String) -> ();

    fn add_letter_to_empty_word(&mut self, letter: String) -> ();

    fn add_letter_to_errors(&mut self, letter: String) -> ();

    fn get_blank_chars(size: usize) -> Vec<String>;

    fn print_sequence(separator: String, sequence: Vec<String>) -> ();

    fn print_hits(&mut self) -> ();

    fn print_errors(&mut self) -> ();

    fn correct_answer(&mut self, input: String) -> ();

    fn wrong_answer(&mut self, input: String) -> ();

    fn read_user_input() -> String;

    fn print_gibbet_status(&self) -> ();

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
        self.formed_word_by_hits = Self::get_blank_chars(self.word.len());
        self.run();
    }

    fn end(&self, status: &str) -> () {
        println!("You {}", status);
        println!("\nThe word is: {}", self.word);
    }

    fn clear_screen() -> () {
        clearscreen::clear().unwrap();
    }

    fn word_contains_letter(&self, letter: String) -> bool {
        let binding = self.word.to_lowercase();
        let word: String = binding;
        word.contains(&letter)
    }

    fn get_occurrences(&self, letter: String) -> Vec<usize> {
        let mut occurrences: Vec<usize> = Vec::new();
        for (index, character) in self.word.chars().enumerate() {
            if letter.eq_ignore_ascii_case(character.to_string().as_str()) {
                occurrences.push(index);
            }
        };
        occurrences
    }

    fn add_letter_to_hits(&mut self, letter: String) -> () {
        self.hits.push(letter);
    }

    fn add_letter_to_empty_word(&mut self, letter: String) -> () {
        let occurrences: Vec<usize> = self.get_occurrences(letter.clone());
        for index in occurrences.iter() {
            self.formed_word_by_hits[*index] = letter.clone();
        }
    }

    fn add_letter_to_errors(&mut self, letter: String) -> () {
        self.errors.push(letter);
    }

    fn get_blank_chars(size: usize) -> Vec<String> {
        let mut blank_chars: Vec<String> = Vec::new();
        for _ in 1..=size {
            blank_chars.push(" ".to_string())
        }
        blank_chars
    }

    fn print_sequence(separator: String, sequence: Vec<String>) -> () {
        println!("{}", sequence.iter().map(|n| format!("{}{separator}", n)).fold(String::new(), |acc, arg| acc + arg.as_str()));
    }

    fn print_hits(&mut self) -> () {
        let blank_chars: Vec<String> = HangmanGame::<>::get_blank_chars(self.word.len());
        let _self = &mut *self.formed_word_by_hits;
        HangmanGame::<>::print_sequence(" ".to_string(), _self.to_vec());
        HangmanGame::<>::print_sequence("_".to_string(), blank_chars);
    }

    fn print_errors(&mut self) -> () {
        let _self = &mut *self.errors;
        HangmanGame::<>::print_sequence(" ".to_string(), _self.to_vec());
    }

    fn correct_answer(&mut self, input: String) -> () {
        self.add_letter_to_hits(input.clone());
        self.add_letter_to_empty_word(input.clone());
    }

    fn wrong_answer(&mut self, input: String) -> () {
        self.attempts += 1;
        self.add_letter_to_errors(input);
    }

    fn read_user_input() -> String {
        println!("Choose a letter: ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.to_owned()
    }

    fn print_gibbet_status(&self) -> () {
        match self.attempts {
            0 => println!("{}", Gibbet::Gibbet),
            1 => println!("{}", Gibbet::Head),
            2 => println!("{}", Gibbet::HeadAndRightArm),
            3 => println!("{}", Gibbet::HeadRightArmAndLeftArm),
            4 => println!("{}", Gibbet::HeadRightArmLeftArmAndStem),
            5 => println!("{}", Gibbet::HeadRightArmLeftArmStemAndRightLeg),
            6 => println!("{}", Gibbet::HeadRightArmLeftArmStemRightLegAndLeftLeg),
            _ => println!("Invalid"),
        }
    }

    fn print_game(&mut self) -> () {
        println!("\n\t\tJogo da Forca");
        self.print_errors();
        println!("\n");
        self.print_gibbet_status();
        println!("\n");
        self.print_hits();
        println!("\n");
    }

    fn verify_answer(&mut self, input: String) -> () {
        if self.word_contains_letter(input.to_lowercase().to_string()) {
            self.correct_answer(input.to_lowercase().to_string());
        } else {
            self.wrong_answer(input.to_lowercase().to_string());
        }
    }

    fn prompt_user(&mut self) -> () {
        let mut input = HangmanGame::<>::read_user_input();
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
