use std::io;

pub fn question8() {

    println!("Enter Subject 1 Marks?");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1)
        .expect("Failed to read line");
    let number1: f64 = number1.trim().parse()
        .expect("Please type a number!");

    println!("Enter Subject 2 Marks?");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2)
        .expect("Failed to read line");
    let number2: f64 = number2.trim().parse()
        .expect("Please type a number!");
        
        result(number1,number2);
}

fn result(num1:f64,num2:f64)
{
    let total_marks : f64 = num1+ num2;
    println!("Total Marks Obtain By Candidate is {}",total_marks);
    let percentage:f64 = (total_marks/200.0)*100.0;
    println!("Percentage Obtain By Candidate is {}",percentage);
    if percentage > 70.0
    {
        println!("Candidate Is Pass");
    }
    else
    {
        println!("Candidate Is Fail");
    }
}
