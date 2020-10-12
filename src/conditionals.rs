//

pub fn run() {
    let age = 21;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    if age >= 21 && (check_id || knows_person_of_age)  {
        println!("Age = {}, What kind of drink would you like?",age);
    } else if age < 21 && check_id {
        println!("Age = {}, Sorry, you have to leave.",age);
    } else {
        println!("Bartender: I'll need to see your id. ");
    }

    // Shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}",is_of_age);


}   