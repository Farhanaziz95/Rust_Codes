use std::io;

pub fn string_merathon() {
    
    println!("Enter Any String?");
     let mut b = String::new();
     io::stdin().read_line(&mut b)
         .expect("Failed to read line");


     println!("How many Times To Run?");
     let mut toberun = String::new();
     io::stdin().read_line(&mut toberun)
         .expect("Failed to read line");

     let mut toberun: u32 = toberun.trim().parse()
         .expect("Please type a number!");

    let mut count = 0;
         while count < toberun
         {

             print!("{}",b);
             count +=1;
         }

}