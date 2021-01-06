// Variables hold  primitive data or references to data
// Variables are immutables by default
// Rust is block-scope language

pub fn run(){
    let name="Faris";
    let mut age=30; // mut make variable to be mutable
    
    println!("My Name is {} and I am {}",name, age); // to hide warning that age with 30 is never read

    age = 31;

    println!("My Name is {} and I am {}",name, age);


    // define Constant
    const ID: i32 = 001; // we can use const using Upper Case
    println!("ID : {}",ID);

    // Multiple assignment
    let (my_name, my_age) = ("Faris",30);
    println!("My name {}, my age {}", my_name, my_age);
}    
