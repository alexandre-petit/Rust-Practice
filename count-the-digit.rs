//version 1: max buffer size reached
fn nb_dig(n: i32, d: i32) -> i32 {
    let mut squares = Vec::new();
    let digit: char;
    
    let d_str = format!("{d}");
    if let Some(digit_str) = d_str.chars().next() {
        digit = digit_str;
        println!("done!");
    } else {
        println!("Something went wrong");
        digit = 'l';
    }
    let mut counter = 0;    
    println!("digit: {digit}");

    for i in 0..n+1 {
        squares.push(i * i);
    }
    
    for square in squares.iter() {
        let num_str = format!("{square}");
        println!("{square}");
        let count_digit = num_str.chars().filter(|&c| c == digit).count();
        println!("count: {count_digit}");
        counter += count_digit;
    }
    counter as i32
}
