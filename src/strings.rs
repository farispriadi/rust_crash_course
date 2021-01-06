// Primitive str = Immutable
// String = Growable
//

pub fn run(){
    let mut hello = String::from("hello ");

    // get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");


    // capacity in bytes
    println!("Capacity : {}", hello.capacity());

    // is empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains substring
    println!("Contains 'World' : {}", hello.contains("World!"));

    // Replace 
    println!("Replace 'World' with 'There' : {},", hello.replace("World","There"));


    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}",word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');


    // Aseertion, pass when no exception
    assert_eq!(2, s.len());


    assert_eq!(10,s.capacity());

    println!("{}", hello);
}    
