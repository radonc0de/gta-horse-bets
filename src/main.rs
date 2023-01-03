use std::env;

fn main() {
    let str_odds: Vec<String> = env::args().skip(1).collect();
    let odds: Vec<f64> = str_odds.into_iter().map(|a| {a.parse::<f64>().unwrap()}).collect();
    if odds.len() < 6 {
        return;
    }
    let mut probs: Vec<f64> = Vec::new();
    probs.push(1.0/(1.0+(odds[0]*((1.0/odds[1])+(1.0/odds[2])+(1.0/odds[3])+(1.0/odds[4])+(1.0/odds[5])))));
    let mut highest_prob = probs[0];
    let mut highest_horse = 1;
    for i in 1..6{
        probs.push((odds[0]/odds[i])*probs[0]);
        if probs[i] > highest_prob {
            highest_prob = probs[i];
            highest_horse = i+1;
        }
    }
    if highest_prob > 0.7 {
        println!("Bet Grade: A");
    }else if highest_prob > 0.65 {
        println!("Bet Grade: B+");
    }else if highest_prob > 0.6 {
        println!("Bet Grade: B");
    }else if highest_prob > 0.55 {
        println!("Bet Grade: C+");
    }else if highest_prob > 0.50 {
        println!("Bet Grade: C");
    }else{
        println!("Bet Grade: F");
    }
    println!("Best Bet: Horse {} [Win% = {} Loss% = {}] [Bet Multiplier = {}]", highest_horse, highest_prob, 1.0-highest_prob, odds[highest_horse-1]);
    

    
}
