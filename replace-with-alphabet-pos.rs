//In this kata you are required to, given a string,
//replace every letter with its position in the alphabet.

//If anything in the text isn't a letter,
//ignore it and don't return it.

//"a" = 1, "b" = 2, etc.

//Example
//alphabet_position("The sunset sets at twelve o' clock.")
//Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" ( as a string )

fn alphabet_position(text: &str) -> String {
    let mut output_str = String::new();
    let string = text.to_lowercase();
    
    for (i,c) in string.chars().enumerate() {
        
        if c.is_alphabetic() {
            let value = (c as u32) - 96;
            output_str = format!("{output_str} {value}");
        }

    }
    output_str.trim().to_string()
}
