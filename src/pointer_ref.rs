

pub fn run(){
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Array value {:?}",(arr1, arr2));

    // Vector
    // if non primitive value the first variable will no longer hold  that value , so use & to
    // point the resource
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    
    println!("Vector value {:?}", (&vec1, vec2));
}
