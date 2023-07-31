#[derive(Debug)] 
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person <'a> {
    name: &'a str,
    age: u8
}
 fn main()
 {
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name. ",
            "Bill",
        "Bob",
    actor="actor's");

    println!("{:?} will print!", Structure(3)); //? means print with debug
    println!("{:?} will print!", Deep(Structure(7))); 

    let name = "Ethan";
    let age = 23;
    let ethan = Person{name, age};

    println!("{:#?}", ethan);   
 }
 