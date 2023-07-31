
macro_rules! call_pi { // macro that prints pi to 5dp
    () => {
        println!("Pi is roughly {0:.5}", "3.141692");
    };
}

#[allow(dead_code)]
fn main() {
    //prints hellow world
    println!("Hello, world!");

    //position argumemnts, 0 based index
    println!("My name is {0} and currently I am trying to learn {1}", "Ethan", "Rust");

    //subject name based indexing
    println!("{food} {opinion} {person}",
                food = "Pizza, burgers and chips",
            opinion = "are very bad for",
            person = "you.");

    //number padding
    println!("{number:0>10}",number=1); //output is 0000000001
    println!("{number:0<10}", number =1); //output is 1000000000

    //number padding using variable names
    println!("{number:0>width$}", number=1, width=5); //output is 00001
    println!("{number:0<width$}", number=1, width=5); //output is 10000

    #[allow(dead_code)]
    struct Structure(i32);

   // println!("This struct `{}` wont print...", Structure(3));

    call_pi!(); 

}
