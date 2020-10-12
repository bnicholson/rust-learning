//
//

pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}",count);

        if count >= 20 {
            break;
        }
    }

    // while (fizzbuzz)
    count = 0;
    while count < 100 {
        if count % 15 == 0 {
            println!("Fizzbuzz {}",count);
        } else if count % 3 == 0{
            println!("Fizz {}",count);
        } else if count % 5 == 0 {
            println!("Buzz {}",count);

        }
        count += 1;
    }

    // now do a range for
    for x in 1..99 {
        if x % 15 == 0 {
            println!("Fizzbuzz {}",x);
        } else if x % 3 == 0{
            println!("Fizz {}",x);
        } else if x % 5 == 0 {
            println!("Buzz {}",x);
        }
    }  
}