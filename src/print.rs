pub fn run(){
    // print to console
    println!("Hello from print.rs file");

    //print basic format number
    println!("Number : {}",1);

    // Basic Formating
    println!("{} is from {}","Faris","Bandung");

    // Positional Arguments
    println!("{0} is from {1}, {0} is love to {2}","Faris","Bandung","code");

    // Named Arguments
    println!("{name} works as {jobs}",name="Faris",jobs="Programmer");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10,10,10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 +10 = {}",10+10);
}
