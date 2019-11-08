#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]

pub fn question1()
{
    struct distances 
    {
        feet: i32,
        inch: i32
    };

    impl distances 
    {
        fn addfeet(&self , distance : &distances) -> i32 
        {
        self.feet + distance.feet
        }

        fn addinch(&self , distances : &distances) -> i32 
        {
        self.inch + distances.inch
        }
    }

    let a : distances = distances {
                            feet : 12,
                            inch : 10,
                        };
    let b : distances = distances {
                            feet : 10,
                            inch : 20,
                        };

    let addedfeet = a.addfeet(&b);

    let addedinch = a.addinch(&b);

    println!("Value After Adding Both Feets {}", addedfeet);
    println!("Value After Adding Both Inches {}", addedinch);


}
