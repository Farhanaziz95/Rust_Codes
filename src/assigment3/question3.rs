pub fn question3()
{
    let first_name : &str = "Farhan";
    let first_name : String = String::from(first_name);
    
    let mut roll_no = String::new();

    roll_no.push_str("051142");
    roll_no.push('B');

    let concat = first_name + " " + &roll_no;

    println!("{}", concat);
}