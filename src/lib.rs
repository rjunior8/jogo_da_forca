use std::io::{BufRead, stdin, stdout, Write};
use crate::constants::attempts::SIX;
use crate::logger::logger::{Logger, StdoutLogger};
use crate::puppet::gibbet::Gibbet;

pub mod constants {
    pub mod attempts;
    pub mod gibbet_phases;
}

pub mod game {
    pub mod game;
}

pub mod logger {
    pub mod logger;
}

pub mod puppet {
    pub mod gibbet;
}

pub mod random_word {
    pub mod random_word;
}

pub fn word_contains_letter(letter: String, word: String) -> bool {
    let binding = word.to_lowercase();
    let word: String = binding;
    word.contains(&letter)
}

pub fn get_occurrences(letter: String, word: &'static str) -> Vec<usize> {
    let mut occurrences: Vec<usize> = Vec::new();
    for (index, character) in word.chars().enumerate() {
        if letter.eq_ignore_ascii_case(character.to_string().as_str()) {
            occurrences.push(index);
        }
    };
    occurrences
}

pub fn get_blank_chars(size: usize) -> Vec<String> {
    let mut blank_chars: Vec<String> = Vec::new();
    for _ in 1..=size {
        blank_chars.push(" ".to_string())
    }
    blank_chars
}

pub fn print_sequence(separator: String, sequence: Vec<String>, logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", sequence.iter().map(|n| format!("{n}{separator}")).fold(String::new(), |acc, arg| acc + arg.as_str())));
}

pub fn prompt<R, W>(mut reader: R, mut writer: W, question: &str) -> String
    where
        R: BufRead,
        W: Write,
{
    write!(&mut writer, "{}", question).expect("Unable to write");
    stdout().flush().unwrap();
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}

#[cfg(not(tarpaulin_include))]
fn get_answer() -> String {
    let stdio = stdin();
    let input = stdio.lock();
    let output = stdout();
    let answer = prompt(input, output, "Choose a letter: ");
    answer
}

pub fn print_gibbet(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::Gibbet));
}

pub fn print_head(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::Head));
}

pub fn print_head_and_right_arm(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::HeadAndRightArm));
}

pub fn print_head_right_arm_and_left_arm(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::HeadRightArmAndLeftArm));
}

pub fn print_head_right_arm_left_arm_and_stem(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::HeadRightArmLeftArmAndStem));
}

pub fn print_head_right_arm_left_arm_stem_and_right_leg(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::HeadRightArmLeftArmStemAndRightLeg));
}

pub fn print_head_right_arm_left_arm_stem_right_leg_and_left_leg(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("{}", Gibbet::HeadRightArmLeftArmStemRightLegAndLeftLeg));
}

pub fn print_default(logger: &mut dyn Logger) -> () {
    logger.print(format_args!("Invalid"));
}

pub fn print_gibbet_status(attempts: u8, logger: &mut dyn Logger) -> () {
    match attempts {
        0 => print_gibbet(logger),
        1 => print_head(logger),
        2 => print_head_and_right_arm(logger),
        3 => print_head_right_arm_and_left_arm(logger),
        4 => print_head_right_arm_left_arm_and_stem(logger),
        5 => print_head_right_arm_left_arm_stem_and_right_leg(logger),
        6 => print_head_right_arm_left_arm_stem_right_leg_and_left_leg(logger),
        _ => print_default(logger),
    }
}

pub fn clear_screen() -> () {
    clearscreen::clear().unwrap();
}

pub fn win(word: &'static str, formed_word_by_hits: &Vec<String>) -> bool {
    let word: String = word.to_lowercase();
    word.eq(&formed_word_by_hits.concat())
}

pub fn lost(attempts: u8) -> bool {
    attempts == SIX
}

pub fn end(status: &str, word: &'static str, logger: &mut dyn Logger) -> () {
    logger.print(format_args!("You {status}"));
    logger.print(format_args!("\nThe word is: {word}"));
}

#[cfg(not(tarpaulin_include))]
pub fn end_of_game(word: &'static str, formed_word_by_hits: &Vec<String>, attempts: u8) -> () {
    let mut std_out_logger = StdoutLogger;
    if win(word, &formed_word_by_hits) {
        end("WON", word, &mut std_out_logger);
    } else if lost(attempts) {
        end("LOSE", word, &mut std_out_logger);
    }
}
