// FUnctions - let's learn about rust functions.

pub fn run() {
    println!("Time to learn about Rust Functions");

    greet("Hello", "Barry");

    let result: u32 = add(5,10);

    println!("Adding two numbers {} {} = {}", 5, 10, result);

    // CLosures
    let n3: i32 = 5;
    let add_nums = | n1: i32, n2: i32 | n1 + n2 + n3;
    println!("Closure {} + {} + n3 = {}",3,4,add_nums(3,4));
}

fn greet(greeting: &str,name: &str) {
    println!("{} {} nice to meet you",greeting, name);
}

fn add(first: u32, second: u32) -> u32 {
    first + second
}