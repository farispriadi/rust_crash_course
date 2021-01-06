// Array :  Fixed list where  element is same data types

use std::mem;

pub fn run(){
    let mut numbers: [i32; 4] = [1,2,3,4];

    // re-assign value
    numbers[2]= 20;

    println!("{:?}", numbers);

    // get single val
    println!("Single value:  {}", numbers[0]);

    println!("Length {}", numbers.len());

    // Array are stack allocated
    println!("Array occupied {} bytes",mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice : {:?}",slice)
}
