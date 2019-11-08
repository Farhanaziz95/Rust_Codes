pub fn question1() {
  
    let number = 10;

    let tup = square(number);

    println!("{:?}",tup);

}
fn square(num: u32) -> (u32,u32)
    {
        let square = num*num;

        (num,square)
    }
