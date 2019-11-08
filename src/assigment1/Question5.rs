pub fn Question5()
{
 let student_data: (&str , i8 , &str , &str ) =  ("Farhan Aziz",24,"IOT051142","6:45 to 9:45");

 let (name , age , roll_number , class) = student_data;

    println!("Student Name = {}",name );
    println!("Student Age = {}", age);
    println!("Student Roll Number = {}", roll_number);
    println!("Student Class = {}", class);
}