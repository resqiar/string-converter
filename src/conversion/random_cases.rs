use rand::prelude::*;

pub fn to_random_cases(input: &String) -> String {
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
        // Should we use gen_bool instead??
        let rand_ratio: u8 = rand::thread_rng().gen_range(0..=1);

        // If the value is 0, convert the character into to_lowercase
        if rand_ratio == 0 {
            converted_str.push(char.to_ascii_lowercase());
        }
        // If not (means 1) then convert the character into to_uppercase
        else {
            converted_str.push(char.to_ascii_uppercase());
        }

        // Above is equivalent to this code below (not sure which one is best)
        // match rand_ratio {
        //     0 => converted_str.push(char.to_ascii_lowercase()),
        //     1 => converted_str.push(char.to_ascii_uppercase()),
        //     _ => (),
        // }
    } // converted_str is auto-dropped by rust here

    // Return the converted_str
    return converted_str;
}
