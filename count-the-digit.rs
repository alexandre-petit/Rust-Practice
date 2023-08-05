//Take an integer n (n >= 0) and a digit d (0 <= d <= 9) as an integer.

//Square all numbers k (0 <= k <= n) between 0 and n.

//Count the numbers of digits d used in the writing of all the k**2.

//Implement the function taking n and d as parameters and returning this count.

//Examples:
//n = 10, d = 1 
//the k*k are 0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
//We are using the digit 1 in: 1, 16, 81, 100. The total count is then 4.

//The function, when given n = 25 and d = 1 as argument,
//should return 11 since the k*k that contain the digit 1 are:
//1, 16, 81, 100, 121, 144, 169, 196, 361, 441.
//So there are 11 digits 1 for the squares of numbers between 0 and 25.
//Note that 121 has twice the digit 1.

fn nb_dig(n: i32, d: i32) -> i32 {
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

    for i in 0..n+1 {
        let square = i * i;
        let num_str = format!("{square}");
        let count_digit = num_str.chars().filter(|&c| c == digit).count();
        counter += count_digit;
    }
    
    counter as i32
}
