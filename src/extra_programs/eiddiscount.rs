#[allow(non_snake_case)]
#[allow(dead_code)]


use std:: io;

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]

pub fn eiddiscount(){
   
    let mut input = String::new();
    println!("Enter Actual Price:");
    io::stdin().read_line(&mut input).expect("Failed");

    let mut input1 = String::new();
    println!("Enter Discounted Price:");
    io::stdin().read_line(&mut input1).expect("Failed");
    
    let mut input2 = String::new();
    println!("Enter Customer payment amount:");
    io::stdin().read_line(&mut input2).expect("Failed");    
   


    let actualprice:f64 = input.trim().parse().unwrap();
    let discountedamount:f64 = input1.trim().parse().unwrap();
    let mut percentdiscount : f64 = 0.00;
    let customerpaymentamount:f64 = input2.trim().parse().unwrap();
    let mut balance:f64 = 0.00;
   
   let data : (f64,f64,f64) = myfunction(actualprice,discountedamount,customerpaymentamount);
   
   let ( afterdiscout, percentdiscount , balance ) = data;

    println!("Actualprice: {}",actualprice);
    println!("Discounted Amount: {}",discountedamount);
    println!("Percent Discount: {}",percentdiscount);
    println!("Customer Payment Amount: {}",customerpaymentamount);
    println!("Balance: {}",balance);
   
  if percentdiscount <= 10.0
  {
      println!("Azadi Offer");
  }
  else  if percentdiscount > 10.0 || percentdiscount <= 20.0
  {
      println!("Eid Offer");
  }
  else if percentdiscount > 20.0
  {
      println!("Clearance Sale");
  }
  else
  {
      println!("");
  }
 
   


fn myfunction(actualprice:f64 ,discountedamount:f64 ,customerpaymentamount:f64 ) -> (f64,f64,f64)
{
    let ActualDiscountedAmount :f64 = actualprice -  discountedamount;
    let percentdiscount:f64 = ActualDiscountedAmount/actualprice * 100.0;
    let balance:f64 = customerpaymentamount - discountedamount;

    let mut datacollect : (f64,f64,f64) = (ActualDiscountedAmount,percentdiscount,balance);
    datacollect
    
}

}