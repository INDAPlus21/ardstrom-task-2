use std::io;
use std::io::prelude::*;

fn main() {
    
    //kattis input
    let input = io::stdin();

    let mut numbers: Vec<usize> = input
        .lock()
        .lines()   
        .next().unwrap().ok().unwrap()
        .split_whitespace()
        .map(|_number| _number.parse::<usize>().ok().unwrap())
        .collect();
    let top_side = numbers[0]+1;
    let left_side = numbers[1]+1;

    while numbers[0] > 0{
        let mut looped_x_position = numbers[1];
        while looped_x_position > 0{
            if left_side - looped_x_position > 9 &&
                top_side - numbers[0] > 9 &&
                numbers[0] > 9 &&
                looped_x_position > 9{
                print!(".");
            }else if   left_side - looped_x_position <= top_side - numbers[0] &&
                        left_side - looped_x_position < numbers[0]  && 
                        left_side - looped_x_position < looped_x_position{ //left side
                print!("{}", left_side - looped_x_position);

            }else if   top_side - numbers[0] < looped_x_position &&  
                        top_side - numbers[0] <= numbers[0]{ //top side
                print!("{}", top_side - numbers[0]);

            }else if    looped_x_position >= numbers[0]{ // bottom side
                print!("{}", numbers[0]);

            }else{
                print!("{}", looped_x_position);//right side
            }
            
            looped_x_position = looped_x_position - 1;
        }
        println!("");
        numbers[0] = numbers[0] - 1;
    }   
    
}