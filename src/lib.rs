use std::io::{stdin, stdout, Write};
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

fn read_user_input() -> String {
    print!("Choose a letter: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
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

pub fn print_default() -> () {
    println!("Invalid");
}

fn print_gibbet_status(attempts: u8) -> () {
    let mut std_out_logger = StdoutLogger;
    match attempts {
        0 => print_gibbet(&mut std_out_logger),
        1 => print_head(&mut std_out_logger),
        2 => print_head_and_right_arm(&mut std_out_logger),
        3 => print_head_right_arm_and_left_arm(&mut std_out_logger),
        4 => print_head_right_arm_left_arm_and_stem(&mut std_out_logger),
        5 => print_head_right_arm_left_arm_stem_and_right_leg(&mut std_out_logger),
        6 => print_head_right_arm_left_arm_stem_right_leg_and_left_leg(&mut std_out_logger),
        _ => print_default(),
    }
}
