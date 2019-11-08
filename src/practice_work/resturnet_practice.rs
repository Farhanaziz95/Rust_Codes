use std::io;
pub fn resturent() {

    let zinger = 200;
    let pizza = 100;
    
    let mut sum = 0;

    loop {

        println!("What do u want to order?");
    println!("1=> Pizza {} , 2=>zinger  {}",pizza , zinger );


     let mut toberun = String::new();
     io::stdin().read_line(&mut toberun)
         .expect("Failed to read line");

     let mut toberun: u32 = toberun.trim().parse()
         .expect("Please type a number!");

         
         println!("how much quentity?");
     let mut quentity = String::new();
     io::stdin().read_line(&mut quentity)
         .expect("Failed to read line");

     let mut quentity: u32 = quentity.trim().parse()
         .expect("Please type a number!");

         if toberun == 1
         {
             sum += 100 * quentity;

         }
         else if toberun == 2
        {
            sum += 200 * quentity;
        }


        println!("Do u Want to order more? Note: Y For Yes , N For No");

        let mut con = String::new();

        io::stdin().read_line(&mut con)
            .expect("Failed to read line");

        let con : char = con.trim().parse()
         .expect("Please type a number!");

        match con {
            'y' => continue,
            'n' => break,
            _ => continue,
            
        }
        };

        println!("Your Total Bill Is {}" ,sum );

}
