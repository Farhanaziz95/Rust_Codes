use std::io;

pub fn question7() {
    
    println!("Enter Number?");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1)
        .expect("Failed to read line");
    let number1: i32 = number1.trim().parse()
        .expect("Please type a number!");


        if number1%2 == 0
        {
            println!("Number {} is Even",number1)
        }
        else
        {
            println!("Number {} is Odd",number1)
        }        

}

