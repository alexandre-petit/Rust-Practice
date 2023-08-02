//Define a method hello that returns "Hello, Name!" to a given name,
//or says Hello, World! if name is not given (or passed as an empty String).

//Assuming that name is a String and it checks for user typos
//to return a name with a first capital letter (Xxxx).

fn hello(name: &str) -> String {
    let mut base = "Hello, ".to_string();
    let mut ending = "World!".to_string();
    
    if name != "" {
        let mut title_cased = "".to_string();
        for (i,c) in name.chars().enumerate() {
            if i == 0 {
                let letter = c.to_ascii_uppercase();
                title_cased = format!("{letter}");
            } else {
                let letter = c.to_ascii_lowercase();
                title_cased = format!("{title_cased}{letter}");
            }
        }
        ending = format!("{title_cased}!");
    }
    base = format!("{base}{ending}");
    
    base
}
