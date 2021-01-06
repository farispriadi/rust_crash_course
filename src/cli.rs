use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);

    let command = args[1].clone();

    println!("command {}", command);

    if command == "hello" {
        println!("Hi Faris , How are you ?");
    }else if command == "status"{
        println!("Status 100 %");
    }else{
        println!("No command found");
    }
}
