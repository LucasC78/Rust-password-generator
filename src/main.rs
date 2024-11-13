use rand::{Rng, distributions::Alphanumeric};
use rand::prelude::SliceRandom;

const SPECIAL_CHARACTERS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*', '_', '-'];

fn generate_random_password(length: usize) -> Vec<char> {
    let rng = rand::thread_rng(); 
    let rng_clone = rng.clone(); 
    let mut password_chars: Vec<char> = rng_clone
        .sample_iter(&Alphanumeric)
        .take(length - SPECIAL_CHARACTERS.len())
        .map(char::from)
        .collect();
    let mut rng_clone = rng.clone(); 
    password_chars.extend(SPECIAL_CHARACTERS.choose_multiple(&mut rng_clone, SPECIAL_CHARACTERS.len()));
    let mut rng = rand::thread_rng(); 
    password_chars.shuffle(&mut rng); 
    password_chars
}

fn main() {
    let password_length = 16;
    let password_chars = generate_random_password(password_length);
    let password: String = password_chars.iter().collect();
    println!("Password generated : {}", password);
}
