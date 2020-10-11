//


pub fn run() {
    let person : (&str,&str,i8) = ("Barry","Iowa",17);

    // println
    println!("Person = {:?}",person);
    println!("{} is from {} and is {}",person.0,person.1,person.2);
}