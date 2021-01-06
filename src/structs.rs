// Struct : used to create custom data types


// Traditional Struct
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

// Tuples Struct
struct Color2(u8,u8,u8);


struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}",self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 100;

    println!("Color RGB : {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(200,0,50);
    c2.1 =  100;

    println!("Color2 RGB : {} {} {}", c2.0, c2.1, c2.2);
    
    let mut p = Person::new("Faris", "Priadi");

    println!("Person Full name {}", p.full_name());
    p.set_last_name("Kadarisman");
    println!("Person {} {}", p.first_name, p.last_name);

    println!("Person name tuple {:?}", p.to_tuple());
}
