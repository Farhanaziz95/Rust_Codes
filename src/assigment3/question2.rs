pub fn question2()
{
    let mut v = Vec::new();

    v.push("Hello");
    v.push("Vector is a growable array");
    v.push("It stores the data on the heap");

    while (!v.is_empty())
    {
        let total_lenth = v.len();
        println!("Element on Vector Before Print : {}",total_lenth );
        let last_word = v.remove(total_lenth-1);
        println!("{}",last_word );
    }

}