// Functions


pub fn run(){
    greeting("Assalamualaikum", "Faris");

    //Bind functions values to variables
    let get_sum = add(5,6);
    println!("Result add : {}", get_sum);


    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure add : {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, Nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1+n2
}
