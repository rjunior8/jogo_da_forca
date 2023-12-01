use std::fmt::Arguments;

pub trait Logger {
    fn print(&mut self, value: Arguments<'_>);
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
    fn print(&mut self, value: Arguments<'_>) {
        println!("{}", value);
    }
}
