use std::env;
use std::fs;

use crate::conversion::random_cases::to_random_cases;

mod conversion;

fn main() {
    // Collect the arguments from the cli invocations
    // for example: cargo run [args]
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return println!("Please specify the file path!");
    }

    // Get the index of 1 in the args, why 1 not 0?
    // because the first args is reserved for the program's binary name.
    // see: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#reading-the-argument-values
    let file_path: &String = &args[1];

    // read the file and convert them into a single string
    let raw_str: String = read_file(&file_path.to_string());

    // format with random capital character
    let formatted_str: String = to_random_cases(&raw_str);

    println!("{}", formatted_str);
}

fn read_file(file_path: &String) -> String {
    let file_content: String = fs::read_to_string(file_path).expect("File not found");
    return file_content;
}
