//Given an array/list [] of n integers ,
//find maximum triplet sum in the array Without duplications .

//Notes :
//Array/list size is at least 3 .

//Array/list numbers could be a mixture of positives,
//negatives and zeros .

//Repetition of numbers in the array/list could occur,
//So (duplications are not included when summing).

//Input >> Output Examples
//1- maxTriSum ({3,2,6,8,2,3}) ==> return (17)
//Explanation:
//As the triplet that maximize the sum {6,8,3} in order , their sum is (17)

//Note : duplications are not included when summing , (i.e) the numbers added only once .

use std::collections::HashSet;

fn max_tri_sum(arr: &[i32]) -> i32 {
    let mut output = 0;
    
    let unique: HashSet<_> = arr.iter().cloned().collect();
    let mut sorted_vec: Vec<i32> = unique.into_iter().collect();
    sorted_vec.sort_by(|a,b| b.cmp(a));
    
    let three_largest = sorted_vec.iter().take(3);
    
    for num in three_largest {
        output += num;
    }
    
    output
}
