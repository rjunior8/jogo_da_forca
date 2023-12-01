use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn get_random_word() -> &'static str {
    let words = ["Apple", "World", "Dog", "Sky"];
    let mut rng = thread_rng();
    words.choose(&mut rng).unwrap()
}
