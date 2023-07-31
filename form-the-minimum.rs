//Task
//Given a list of digits,
//return the smallest number that could be formed from these digits,
//using the digits only once (ignore duplicates).

//Notes:
//Only positive integers will be passed to the function (> 0 ), no negatives or zeros.
//Input >> Output Examples
//minValue ({1, 3, 1})  ==> return (13)
//Explanation:
//(13) is the minimum number could be formed from {1, 3, 1} , Without duplications

//minValue({5, 7, 5, 9, 7})  ==> return (579)
//Explanation:
//(579) is the minimum number could be formed from {5, 7, 5, 9, 7} , Without duplications

fn min_value(mut digits: Vec<i32>) -> i32 {
    let mut valid_digits = String::new();
    let mut valid_vector = Vec::new();
    
    for d in digits.iter() {
        if !valid_vector.contains(d) {
            valid_vector.push(*d);
        }
    }
    valid_vector.sort();
    
    for d in valid_vector.iter() {
        valid_digits = format!("{valid_digits}{d}");
    }
    
    println!("{valid_digits}");
    return valid_digits.parse().unwrap();
}
