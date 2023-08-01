//The first input array is the key to the correct answers to an exam,
//like ["a", "a", "b", "d"]. The second one contains a student's submitted answers.

//The two arrays are not empty and are the same length.
//Return the score for this array of answers, giving +4 for each correct answer,
//-1 for each incorrect answer, and +0 for each blank answer,
//represented as an empty string (in C the space character is used).

//If the score < 0, return 0.

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut total = 0;
    for (answer, result) in arr_a.iter().zip(arr_b.iter()) {
        if result == &"" {
            println!("blank answer!");
        } else if answer == result {
            println!("Good Answer!");
            total += 4;
        } else {
            println!("Wrong Answer!");
            total -= 1;
        }
    }
    if total < 0 {
        total = 0;
    }
    
    total
}
