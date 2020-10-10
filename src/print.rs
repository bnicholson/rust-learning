pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    println!("{2} {1} {0}",1,2,3);

    // named arguments
    println!("{name} like to play {activity}", name = "John", activity = "baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x}  Octal: {:o}",10,10,10);

    // Debug trait
    println!(" {:?} ", (12, true, "hello")  );

    // basic math
    println!("10 + 10 = {}", 10 + 10 );
    
}

