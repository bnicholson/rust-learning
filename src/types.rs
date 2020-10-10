
// Primitive types
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

pub fn run() {
    // default is i32
    let x = 1; 
    println!("x = {}",x);

    // float default is f64
    let y = 2.5;
    println!("Y = {}",y);

    // add an explict type
    let z: i64 = 4545454545;
    println!("z = {}", z);

    // Float work
    let alpha = 1 / 3;
    println!("alpha = {}",alpha);

    //  Max values
    println!("Max i32: {}", std::i32::MAX);
    println!("Max f64: {}", std::f64::MAX); 

    // Boolean
    let is_active = true;
    let is_greater : bool = 10 > 5;    

    // Char
    let a1 = 'a';
    let f1 = '\u{1F600}';

    // print stuff
    println!("{:?}", (x,y,z,alpha,!is_active, is_greater,a1,f1));
}