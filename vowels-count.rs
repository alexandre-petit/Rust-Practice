//Return the number (count) of vowels in the given string.
//We will consider a, e, i, o, u as vowels for this Kata (but not y).
//The input string will only consist of lower case letters and/or spaces.

fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let vowels = "aeiou";

    for c in vowels.chars() {
      vowels_count += string.chars().filter(|&l| l == c).count();
    }
  
  vowels_count
}
