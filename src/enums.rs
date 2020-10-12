// Enumerations

enum Movement {
    Up,
    Down,
    Left,
    Right,
    Back,
    Forward
}

fn move_avatar(m: Movement) {
    // Do something depending upon movement type
    match m {
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving Down"),
        Movement::Left => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right"),
        Movement::Back => println!("Avatar moving Back"),
        Movement::Forward => println!("Avatar moving Foward")
    }
}

pub fn run() {
    let avatar_up = Movement::Up;
    let avatar_down = Movement::Down;
    let avatar_left = Movement::Left;
    let avatar_right = Movement::Right;
    let avatar_back = Movement::Back;
    let avatar_forward = Movement::Forward;

    move_avatar(avatar_up);
    move_avatar(avatar_down);
    move_avatar(avatar_left);
    move_avatar(avatar_right);
    move_avatar(avatar_back);
    move_avatar(avatar_forward);  
}