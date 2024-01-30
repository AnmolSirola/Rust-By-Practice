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

/* 
fn main(){

    let variants = Color::Red;

    match variants{
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
    };
}

enum Color{
    Red,
    Green,
    Blue,
}
*/

enum Color{
    Red,
    Green,
    Blue,
}

fn print_color(color: Color){

    match color{
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
    };
}

fn main(){

    let color = Color::Red;
    print_color(color);
}