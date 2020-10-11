//
//

use std::mem;

pub fn run() {

    let mut numbers: [i32; 12] = [0,1,2,3,4,5,6,7,8,9,0xA,0xB];

    numbers[3] = 13;

    println!("Numbers {:?}",numbers);

    // one member
    println!("Numbers[0] = {}",numbers[0]);

    // get array length
    println!("Length of array = {}",numbers.len());

    // memory size
    println!("Memory size of array = {}",mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[3..8];

    println!("Slices = {:?}",slice);
}