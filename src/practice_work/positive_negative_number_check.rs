pub fn positive_negative_number_check(number:i32)
{

    fn check_positive(number:i32)
    {
        if number < 0
        {
            println!("value {} is Nagitive",number);
        }   
        else
        {
            println!("value {} is Positive",number);

        }
    }

    check_positive(number);
    
}

