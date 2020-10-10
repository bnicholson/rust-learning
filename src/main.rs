mod print;
mod vars;
mod types;
mod strings;


fn main() {

    println!("----------------Misc --------------------------");
    println!("Hello, utc universe");
    let now = chrono::Local::now();
    println!("{}", now.format("Local %b %-d, %-I:%M:%S.%f").to_string());
    let now_utc = chrono::Utc::now();
    println!("{}", now_utc.format("  Utc %b %-d, %-I:%M:%S.%f").to_string());


    print::run();
    println!("-------------------------------------------------");

    vars::run();

    println!("------------------------------------------------");
    types::run();

    println!("---------- Strings --------------------------------------");
    strings::run();   

}
