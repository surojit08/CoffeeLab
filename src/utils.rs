use std::io::{self, BufRead, StdinLock, Write};

pub fn prompt_user(message: &str) {
    println!("? {}", message);
    print!("$: ");
    io::stdout().flush().unwrap();
}

pub fn give_details_of_output(message: &String) {
    println!(">> {}", message);
}

pub fn warn_user(message: &str) {
    println!("! {}", message);
}


pub fn read_numeral_input(handle: &mut StdinLock) -> u8 {
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse::<u8>().unwrap()
}

pub fn read_string_input(handle: &mut StdinLock) -> String {
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().to_string()
}