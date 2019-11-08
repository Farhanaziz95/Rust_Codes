use std::io;
pub fn fibonacci() {
    //println!("Hello world");
    //Q1
    println!("First Number?");
    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let  mut a: u32 = a.trim().parse()
        .expect("Please type a number!");

    println!("Second Number?");
    let mut b = String::new();
    io::stdin().read_line(&mut b)
        .expect("Failed to read line");

    let mut b: u32 = b.trim().parse()
        .expect("Please type a number!");

    let mut count = 0;

    println!("How many Times To Run?");
    let mut toberun = String::new();
    io::stdin().read_line(&mut toberun)
        .expect("Failed to read line");

    let mut toberun: u32 = toberun.trim().parse()
        .expect("Please type a number!");


    while(count < toberun)
    {
        let c = b + a ;

        println!("{}", c);

        
        a = b;
        b = c;

        count += 1;
     }
     
}