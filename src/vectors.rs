//
// vectors:  Resizable arrays
//
//

use std::mem;

pub fn run() {

    let mut numbers: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9,0xA,0xB, 0xc, 0xd, 0xe, 0xf];

    numbers[3] = 13;
    println!("Numbers {:?}",numbers);

    // now add a item to the vector
    numbers.push(16);
    numbers.push(17);
    println!("Pushing two Numbers {:?}",numbers);

    // now remove the last item from the vector
    numbers.pop();
    println!("Popping a Number {:?}",numbers);

    // one member
    println!("Numbers[0] = {}",numbers[0]);

    // get vector length
    println!("Length of vector = {}",numbers.len());

    // memory size
    println!("Memory size of vector = {}",mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[3..8];

    println!("Slices = {:?}",slice);

    // Iterate through all the numbers in the vector
    for x in numbers.iter() {
        println!("Numbers {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!( "Mutating (x=x*2) Numbers {:?}",numbers);
}