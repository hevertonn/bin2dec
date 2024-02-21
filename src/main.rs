use std::io;

fn main() {
    let mut binary_string: String = String::new();

    println!("Enter a number to be convert: ");

    io::stdin()
        .read_line(&mut binary_string)
        .expect("Failed to read line");

    binary_string = binary_string.chars().rev().collect();

    let mut decimal_number: i64 = 0;
    let mut index: i64 = 0;
    
    for i in binary_string.chars() {
        if i == '0' {
            index += 1  
        }
        if i == '1' {
            decimal_number += result(index);
        }
    }

    println!("The number converted is {}", decimal_number);
}

fn result(i: i64) -> i64 {
    let mut id = i;
    let mut res = 1;

    while id != 0 {
        res *= 2;
        id -= 1;
    }
    return res;
}
