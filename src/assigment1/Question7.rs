#[allow(non_snake_case)]
#[allow(dead_code)]


pub fn Question7()
{
    let arry1 = [1,2,3,4,5,6,7];
    let arry2 = [8.1,8.2,8.3];

    let tup = (arry1,arry2,9,10);
    
    println!("values of first Array = {:?}",arry1);
    println!("values of second Array = {:?}",arry2);
    println!("values in tuples = {:?}",tup);
    
    println!("value of second element of tuples array 1 = {:?}",tup.0[1]);
    println!("value of second element of tuples array 2 = {:?}",tup.1[1]);
}