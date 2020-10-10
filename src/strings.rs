// Strings : 
// Primitive type - immutable string in memory.
// String = growable, heap allocated data structure - use when you need to modify string contents

pub fn run() {

    let hello = "hello world";  // primitive immutable
    let hello_str = String::from("Hello String World");  // Growable/modifiable type

    // find a length
    println!("Length of primitive = {} Lenght of heap version = {}",hello.len(), hello_str.len());

}