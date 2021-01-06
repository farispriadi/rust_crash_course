// conditionals

pub fn run(){
    let age = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // IF Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Do you have any driver license?");
    }else if age < 21 && check_id {
        println!("Wow, you have a driver license ?");
    }else{
        println!("Baru lulus sma");
    }


    // Shorthand if
    let is_of_age = if age >=21 { true } else { false };
    println!("Is of Age {}",is_of_age);
}
