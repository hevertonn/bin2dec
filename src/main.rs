use std::io;

fn main() {
    println!("This is a lazy binary to decimal converter");
    println!("Type Ctrl + c to exit!");

    'convert_loop: loop {
        let mut binary: String = String::new();

        println!("Enter a number to be converted: ");

        io::stdin()
            .read_line(&mut binary)
            .expect("Failed to read the number");

        binary = binary.chars().rev().collect();

        let mut decimal: u32 = 0;
        let mut index: u32 = 0;
        let base: u32 = 2;

        for i in binary.chars() {
            if i == '\n' {
            } else if i == '0' {
                index += 1;
            } else if i == '1' {
                decimal += base.pow(index);
                index += 1;
            } else {
                println!("The binary is broken");
                continue 'convert_loop;
            }
        }

        println!("The number converted is: {}", decimal);
    }
}
