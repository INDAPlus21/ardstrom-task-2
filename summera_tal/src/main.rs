use std::io;
use std::io::prelude::*;
//using viola kattis template
/// Kattis calls main function to run your solution. 
fn main() {
    // get standard input stream
    //let mut main_string = String::new();
    //io::stdin().read_line(&mut main_string);

    let input = io::stdin();
    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    
    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();
    
        
    let mut numbers_in_text= Vec::new(); 
    numbers_in_text= lines[1].trim().split(' ').collect();
    let mut numbers: Vec<u32> = Vec::new();
    for i in numbers_in_text{
        numbers.push(i.parse().expect(""));
    }
    numbers.sort_unstable();
    numbers.reverse();
    
    let mut final_number= 0;
    for i in 0..((numbers.len()+1)/2) {
        final_number = final_number + &numbers[i];
    }

    println!("{}", final_number);

    
}