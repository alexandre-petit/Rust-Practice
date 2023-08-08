//Two red beads are placed between every two blue beads.
//There are N blue beads.
//After looking at the arrangement below work out the number of red beads.

//B RR B RR B RR B RR B RR B

//Implement count_red_beads(n) so that it returns the number of red beads.
//If there are less than 2 blue beads return 0.

fn count_red_beads(n: u32) -> u32 {
    let mut red: u32;
    
    if n < 2 {
        red = 0;
    } else {
        red = (n - 1) * 2;
    }
    red
}
