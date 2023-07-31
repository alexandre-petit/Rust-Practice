//Let us begin with an example:

//A man has a rather old car being worth $2000.
//He saw a secondhand car being worth $8000.
//He wants to keep his old car until he can buy the secondhand one.

//He thinks he can save $1000 each month but the prices of his old car and of the new one decrease of 1.5 percent per month.
//Furthermore this percent of loss increases of 0.5 percent at the end of every two months.
//Our man finds it difficult to make all these calculations.

//Can you help him?

//How many months will it take him to save up enough money to buy the car he wants,
//and how much money will he have left over?

fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut available = 0.0;
    let mut old = old as f64;
    let mut new = new as f64;
    let mut perc = perc;
    
    
    let mut months = 0;
    
    while (available + old) < new {
        
        months += 1;
            if months % 2 == 0 {
                perc += 0.5;
        }
        
        old *= (100.0 - perc) / 100.0;
        new *= (100.0 - perc) / 100.0;
        available += saving as f64;
    }
    return (months, (available+old-new) as i32);
}       
