mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers_ref;


fn main() {

    println!("---------- Start --------------------------");
    println!("Hello, utc universe");
    let now = chrono::Local::now();
    println!("{}", now.format("Local %b %-d, %-I:%M:%S.%f").to_string());
    let now_utc = chrono::Utc::now();
    println!("{}", now_utc.format("  Utc %b %-d, %-I:%M:%S.%f").to_string());


    print::run();
    println!("---------- Print ---------------------------------------");

    vars::run();

    println!("---------- Types --------------------------------------");
    types::run();

    println!("---------- Strings --------------------------------------");
    strings::run();   

    println!("---------- Tuples --------------------------------------");
    tuples::run();

    println!("---------- Arrays --------------------------------------");
    arrays::run();

    println!("---------- Vectors --------------------------------------");
    vectors::run();    

    println!("---------- Conditionals --------------------------------------");
    conditionals::run();        

    println!("---------- Loops --------------------------------------");
    loops::run(); 
 
    println!("---------- Functions --------------------------------------");
    functions::run(); 

    println!("---------- Pointers_ref --------------------------------------");
    pointers_ref::run(); 
}
