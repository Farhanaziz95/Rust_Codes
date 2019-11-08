pub fn Question1()
{
    const PI : f32  = 3.145;//constant variable 
    let metro_deal : i16 = 900;// simple variable
    let mut remaining_days : i8 = 3;// mutable variable

    loop
    {
        

        println!("PI Value is : {}" , PI);
        println!("Metro Giving Deal of : {}", metro_deal);
        println!("Remaing Days of Deal : {}", remaining_days);

        remaining_days = remaining_days - 1;
        if remaining_days == 0 {
            break;
        }
    }
}