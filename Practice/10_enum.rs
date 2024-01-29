fn main(){

    let go = Direction::left;

    match go{
        Direction::Left => println!("Go left!"),
        Direction::Right => println!("Go right!"),
        Direction::Up => println!("Go up!"),
        Direction::Down => println!("Go down!"),
    };
}

enum Direction{
    Left,
    Right,
    Up,
    Down,
}