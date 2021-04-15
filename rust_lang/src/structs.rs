//Structs - used to create custom data types

//Traditional Struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Color(u8,u8,u8);

//struct with functions
struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    //construct a person
    fn new(first: &str, last:$str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //get fullname
    fn fullname(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut c = Color{
        red:255,
        green:0,
        blue:0
    };

    c.red = 200;

    println!("Color: {} {} {}",c.red,c.green,c.blue);

    // tuple struct
    let mut c = Color(255,0,0);
    println!("Color: {} {} {}",c.0,c.1,c.2);

    
    let mut p = Person::new("John","Doe");
    println("Person {} {}", p.first_name, p.last_name);

    // get fullname
    println!("Person {}", p.fullname());
    
}