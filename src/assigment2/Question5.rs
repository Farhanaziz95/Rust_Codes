use std::io;

pub fn question5() {
    
    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");
    let number: i32 = number.trim().parse()
        .expect("Please type a number!");

    println!("You Entered: {}", number);

    if number < 0
    {
    println!("Value {} is Nagitive",number);
        
    }
    else if number > 0
    {
    println!("Value {} is Positive",number);

    }
    else
    {
    println!("Value {} is Zero",number);

    }
}