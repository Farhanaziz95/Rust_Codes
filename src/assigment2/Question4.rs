use std::io;

pub fn question4() {
    println!("Hello, world!");

    println!("Enter Radius Of Circle?");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1)
        .expect("Failed to read line");
    let number1: f64 = number1.trim().parse()
        .expect("Please type a number!");

    let pi =3.141592654;    

    println!("The Are Of Circle is  {}",(number1 * number1) * pi);
   }