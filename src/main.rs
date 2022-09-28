use rand::prelude::*;
use std::env;
use std::fs;

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
    let formatted_str: String = convert_to_random_uppercase(&raw_str);

    println!("{}", formatted_str);
}

fn read_file(file_path: &String) -> String {
    let file_content: String = fs::read_to_string(file_path).expect("File not found");
    return file_content;
}

fn convert_to_random_uppercase(input: &String) -> String {
    // Stored on the heap (dynamic size)
    let mut converted_str: String = String::new();

    for char in input.chars() {
        // If the char for current iteration is not an alphabetic,
        // just push the char and continue the loop;
        if !char.is_alphabetic() {
            converted_str.push(char);
            continue;
        }

        // Randomize if the char should be uppercase or not.
        // The value is either 0 or 1.
        // 0 means to_lowercase,
        // 1 means to_uppercase.
        let rand_ratio: i32 = rand::thread_rng().gen_range(0..=1);

        // If the value is 0, convert the character into to_lowercase
        if rand_ratio == 0 {
            converted_str.push(char.to_ascii_lowercase());
        }
        // If not (means 1) then convert the character into to_uppercase
        else {
            converted_str.push(char.to_ascii_uppercase());
        }
    } // converted_str is auto-dropped by rust here

    // Return the converted_str
    return converted_str;
}
