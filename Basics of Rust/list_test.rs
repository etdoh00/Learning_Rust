use std::fmt;

struct List(Vec<i32>); //create a list object that takes a vector as param

impl fmt::Display for List { //creating display function for list struct
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0; //first element 
        write!(f,"[")?; //opening bracket
        for(count, v) in vec.iter().enumerate() { //takes 2 indexes, count will encapsulate the index in the vector, v takes the value at index "count"
            if count != 0 { write!(f,",")?;} // comma seperate values
            write!(f, " {0}:{1} ", count, v)?; //output in format {index:value} for each value
        }
        write!(f,"]") //close vector 
    }    
}

fn main()
{
    let v = List(vec![10,20,30]); //test
    println!("{}",v);
}





