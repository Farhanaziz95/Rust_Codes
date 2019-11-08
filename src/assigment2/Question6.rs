use std::io;

pub fn question6() {

    println!("Enter Numerator?");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1)
        .expect("Failed to read line");
    let number1: i32 = number1.trim().parse()
        .expect("Please type a number!");

    println!("Enter Denominator?");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2)
        .expect("Failed to read line");
    let number2: i32 = number2.trim().parse()
        .expect("Please type a number!");

        if number1%number2 == 0
        {
            println!("Number {} is Completely Divisible by {}",number1,number2)
        }
        else
        {
            println!("Number {} is Not Completely Divisible by {}",number1,number2)            
        }

}