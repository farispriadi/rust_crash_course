// Vectors resizeable array

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // re-assign value
    numbers[2]= 20;

    // add on vectors
    numbers.push(5);
    numbers.push(6);

    // pop on last value
    numbers.pop();

    println!("{:?}", numbers);

    // get single val
    println!("Single value:  {}", numbers[0]);

    println!("Length {}", numbers.len());

    // Array are stack allocated
    println!("Vectors occupied {} bytes",mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice : {:?}",slice);

    // Loop through vector
    for x in numbers.iter(){
        println!("Loop : {}",x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Numbers vectors {:?}",numbers);
}
