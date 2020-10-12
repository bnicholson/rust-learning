//

// Traditional structure
struct Color {
    red: u32,
    green: u32,
    blue: u32
}

// Tuple structure
struct ColorTuple(u32,u32,u32);

// Person stuff
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Build a new person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get the full name
    fn full_name(&self) -> String {
        format!("{} {}",self.first_name,self.last_name)
    }

    // Set the last name
    fn set_last_name(&mut self,last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 128
    };

    c.red = 200;

    println!("C Color {} {} {}",c.red, c.green, c.blue);

    // Color Tuple printing
    let mut ct = ColorTuple(255,0,128);
    ct.0 = 200;

    println!("Color Tuple {} {} {}",ct.0,ct.1,ct.2);

    // Now go create a new Person
    let p = Person::new("Barry","Nicholson");
    println!("Person {} {}",p.first_name,p.last_name);
    println!("Person fullname = {}",p.full_name());

    // Now try out mutating the Person
    let mut md = Person::new("Mary","Doe");
    md.set_last_name("Smith");
    println!("Mary Doe's now {}",md.full_name());


}