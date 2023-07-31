use std::fmt; // include fmt lib

#[derive(Debug)] //custom struct so debug is necessary to make it printable
struct Range(i64, i64); // create a strcut with 64bit integers

impl fmt::Display for Range { //custom display function for MinMax struct
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct PointNamed { //same as above but with named fields and using float instead of integers
    x: f64,
    y: f64
}

impl fmt::Display for PointNamed { // //custom display output, using named fields
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

#[derive(Debug)] // final struct, to reflect complex numbers
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // use of i will print imag: (x)i to reflect imaginary numbers
        write!(f, "real: {}, imag: {}i", self.real, self.imag)
    }
}


fn main1 ()
{
    let minmax = Range(0,14); 

    println!("Compare structs:");
    println!("Display: {}", minmax); //(0,14)
    println!("Debug: {:?}",minmax);  //Range(0,14)

    let b_range = Range(-300, 300);
    let s_range = Range(-5,5);

    println!("The big range is {0} and the small range is {1}", b_range, s_range);

    let p = PointNamed{x: 2.2, y:4.4};

    println!("Compare structs:");
    println!("Display {}", p); //x: 2.2, y:4.4
    println!("Debug {:?}", p); // PointNamed { x: 2.2, y:4.4}

    let c = Complex{real: 1.0, imag: 3.2};

    println!("Compare structs:");
    println!("Display {}", c); //real: 1.0, imag: 3.2i
    println!("Debug {:?}", c); // Complex { real: 1.0, imag: 3.2 }



}