//Your task is to find the first element of an array that is not consecutive.

//By not consecutive we mean not exactly 1 larger than the previous element of the array.

//E.g. If we have an array [1,2,3,4,6,7,8]
//then 1 then 2 then 3 then 4 are all consecutive but 6 is not,
//so that's the first non-consecutive number.

//If the whole array is consecutive then return null.


fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    let my_vec = arr;
    
    let mut last = arr[0] - 1;
    
    for num in my_vec.iter() {
        eprintln!("{}", num);
        let current_num = *num;
        if current_num != last + 1 {
            return Some(current_num);
        }
        last = current_num
    }
    None
}
