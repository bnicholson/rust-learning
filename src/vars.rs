// variables hold primitive data or references to data


pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);
    age = 38;

    println!("I had a birthday.  My name is {} and I am {}", name, age);

    // Define a constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Brad",38);
    println!("{} is {} years old.",my_name, my_age);
}