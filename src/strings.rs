// Strings : 
// Primitive type - immutable string in memory.
// String = growable, heap allocated data structure - use when you need to modify string contents

pub fn run() {

    let hello = "hello ";  // primitive immutable
    let mut hello_str = String::from("Hello String ");  // Growable/modifiable type

    // find a length
    println!("Length of primitive = {} Lenght of heap version = {}",hello.len(), hello_str.len());


    // add to the end of the string
    hello_str.push('W');
    hello_str.push_str("orld!");

    // Get the capacity and whether it's empty or not
    println!("hello_str capacity = {}",hello_str.capacity());
    println!("hello_str is_empty = {}",hello_str.is_empty());

    // Contains
    println!("hello_str contains(\"orld\") = {}",hello_str.contains("orld"));
    println!("hello_str contains(\"jack\") = {}",hello_str.contains("jack"));

    // replace
    println!("hello_str replace {} {}",hello_str,hello_str.replace("World","There"));

    // split by whitespace
    for word in hello_str.split_whitespace() {
        println!("Word: {}",word);
    }

    // Capacity
    let mut s = String::with_capacity(25);
    s.push('A');
    s.push('B');
    s.push('c');
    println!("s = {} Capacity: {}",s,s.capacity());
    s.push_str("Now is the time for all good men to come to the aid of their country.");
    println!("s = {} Capacity: {}",s,s.capacity());   

    // Assertion
    assert_eq!(s.len(),72);

    // Print all the vars now
    println!("Hello String = {}",hello);
    println!("Hello_Str = {}",hello_str);


}