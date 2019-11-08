
#[allow(non_snake_case)]
#[allow(dead_code)]

pub mod assigment 
{
    pub mod a
    {

        pub fn question2()
        {
            let mut v = Vec::new();

            v.push("Hello");
            v.push("Vector is a growable array");
            v.push("It stores the data on the heap");

            while !v.is_empty()
            {
                let total_lenth = v.len();
                println!("Element on Vector Before Print : {}",total_lenth );
                let last_word = v.remove(total_lenth-1);
                println!("{}",last_word );
            }

        }

    }

    pub mod b
    {
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

    }

}

#[allow(non_snake_case)]
#[allow(dead_code)]

pub fn question4()
{
    crate::assigment3::question4::assigment::a::question2();
    self::assigment::b::question3();
}