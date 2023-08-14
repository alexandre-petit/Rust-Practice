//Scenario
//Now that the competition gets tough it will Sort out the men from the boys .

//Men are the Even numbers and Boys are the odd!alt!alt
//Task
//Given an array/list [] of n integers,
//Separate The even numbers from the odds,
//or Separate the men from the boys!alt!alt
//Notes
//Return an array/list where Even numbers come first then odds

//Since , Men are stronger than Boys ,
//Then Even numbers in ascending order While odds in descending .

//Array/list size is at least 4 .

//Array/list numbers could be a mixture of positives , negatives .

//Have no fear , It is guaranteed that no Zeroes will exists .!alt
//Repetition of numbers in the array/list could occur,
//So (duplications are not included when separating).

use std::collections::HashSet;

fn men_from_boys(xs: &[i16]) -> Vec<i16> {
    let mut even = Vec::new();
    let mut odd = Vec::new();
    
    let unique: HashSet<_> = xs.iter().cloned().collect();
    
    let mut output_vec = Vec::new();
    
    for num in unique.iter() {
        if num % 2 == 0 {
            even.push(num);
        } else {
            odd.push(num);
        }
    }
    even.sort();
    odd.sort_by(|a, b| b.cmp(a));
    
    output_vec.extend(even.drain(..));
    output_vec.extend(odd.drain(..));
    
    output_vec
}
