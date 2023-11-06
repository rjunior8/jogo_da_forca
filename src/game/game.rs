use std::fmt::Debug;
use std::io::stdin;

use crate::puppet::gibbet::Gibbet;

#[derive(Debug, Clone)]
pub struct Game {
    word: &'static str,
    formed_word_by_hits: Vec<String>,
    hits: Vec<String>,
    errors: Vec<String>,
    attempts: i8,
}

pub trait GameInterface {
    fn new(word: &'static str, formed_word_by_hits: Vec<String>, hits: Vec<String>, errors: Vec<String>, attempts: i8) -> Self;
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

// fn buffer_from_iter<I, B>(iter: I) -> B
//     where I: IntoIterator<Item = B::Item>,
//           B: GameInterface
// {
//     let mut iter = iter.into_iter();
//     let (lower, _) = iter.size_hint();
//     let mut ans = B::with_capacity(lower);
//     while let Some(x) = iter.next() {
//         ans.push(x);
//     }
//     ans
// }
//
// impl std::iter::FromIterator<<Game as GameInterface>::Item> for Game {
//     fn from_iter<I>(iter: I) -> Self
//         where I: IntoIterator<Item = <Game as GameInterface>::Item>
//     {
//         buffer_from_iter(iter)
//     }
// }

// impl<T, I> GameInterface for Game where Vec<(T, I)>: FromIterator<(usize, &'static str)> {
impl GameInterface for Game {
    fn new(word: &'static str, formed_word_by_hits: Vec<String>, hits: Vec<String>, errors: Vec<String>, attempts: i8) -> Game {
        Game{word, formed_word_by_hits, hits, errors, attempts}
    }

    fn start(&mut self) -> () {
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

    // fn word_contains_letter(&self, letter: String) -> bool {
    //     let binding = self.word.to_lowercase();
    //     let word: &str = binding.as_str();
    //     // let word: String = self.word.to_lowercase().to_string();
    //     word.contains(&letter.to_string())
    // }

    fn word_contains_letter(&self, letter: String) -> bool {
        let binding = self.word.to_lowercase();
        let word: String = binding;
        // let word: String = self.word.to_lowercase().to_string();
        word.contains(&letter)
    }

    fn get_occurrences(&self, letter: String) -> Vec<usize> {
        // let word: &str = self.word.to_lowercase().as_str();
        // word.match_indices(*letter).collect()
        // self.word.to_lowercase().match_indices(&letter.to_string())
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

    // fn add_letter_to_empty_word(&self, letter: char) -> () {
    //     let occurrences: Vec<_> = self.get_occurrences(letter);
    //     for (index, _) in occurrences.iter().collect::<Vec<_>>() {
    //         self.formed_word_by_hits[index] = letter;
    //     };
    //     ()
    // }

    fn add_letter_to_empty_word(&mut self, letter: String) -> () {
        let occurrences: Vec<usize> = self.get_occurrences(letter.clone());
        for index in occurrences.iter() {
            self.formed_word_by_hits[*index] = letter.clone();
        };
        ()
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
        // let sep: &'static char = separator;
        // let mut test = vec![sequence.iter()];
        // let joined: String = test.iter().collect::<Vec<_>>().join(separator);
        // let joined: String = sequence.to_vec().iter().collect::<String>();
        // let joined: String = sequence.iter().copied().combinations(2).collect::<Vec<_>>();
        // let joined = sequence.concat();
        println!("{:?}{separator}", sequence.iter());
    }

    fn print_hits(&mut self) -> () {
        let blank_chars: Vec<String> = Game::<>::get_blank_chars(self.word.len());
        let _self = &mut *self.formed_word_by_hits;
        Game::<>::print_sequence(" ".to_string(), _self.to_vec());
        Game::<>::print_sequence("_".to_string(),blank_chars);
    }

    fn print_errors(&mut self) -> () {
        let _self = &mut *self.errors;
        Game::<>::print_sequence(" ".to_string(), _self.to_vec());
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

    // fn verify_answer(&mut self) -> () {
    //     let input: String = Game::<>::prompt_user();
    //     if self.word_contains_letter(&input.to_lowercase().chars().next().expect("No input")) {
    //         self.correct_answer(&input.to_lowercase().chars().next().expect("No input"));
    //     } else {
    //         self.wrong_answer(&input.to_lowercase().chars().next().expect("No input"));
    //     }
    // }

    fn verify_answer(&mut self, input: String) -> () {
        if self.word_contains_letter(input.to_lowercase().to_string()) {
            self.correct_answer(input.to_lowercase().to_string());
        } else {
            self.wrong_answer(input.to_lowercase().to_string());
        }
    }

    fn prompt_user(&mut self) -> () {
        let mut input = Game::<>::read_user_input();
        input.pop();
        self.verify_answer(input);
    }

    fn win(&self) -> bool {
        let word: String = self.word.to_lowercase();
        word.eq(&self.formed_word_by_hits.concat())
    }

    fn lost(&self) -> bool {
        self.attempts == 6
    }

    fn run(&mut self) -> () {
        while !self.win() && !self.lost() {
            self.print_game();
            self.prompt_user();
            Game::<>::clear_screen();
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
