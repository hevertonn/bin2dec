use std::io;

fn main() {
    let mut binary_number: String = String::new();

    println!("Enter a number to be convert: ");

    io::stdin()
        .read_line(&mut binary_number)
        .expect("Failed to read line");

    let mut length: usize = binary_number.len();

    binary_number[2]

    // while length != 0 {

    //     length -=1
    // }
}
