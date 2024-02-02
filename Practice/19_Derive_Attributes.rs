
enum Direction{
    Left,
    Right,
    Up,
    Down,
}

fn print_direction(direction: Direction){
    match direction{
        Direction::Left => println!("Go left!"),
        Direction::Right => println!("Go right!"),
        Direction::Up => println!("Go up!"),
        Direction::Down => println!("Go down!"),
    };
}

struct Employee{
    id: i32,
    age:i32
}

fn print_employee(emp:Employee){
    println!("id:{:?}", emp.id);
    println!("age:{:?}", emp.age);
}

fn main(){
    let direction = Direction::Right;
    print_direction(direction);

    let emp = Employee
    {
        id: 0,
        age: 21
    };

    print_employee(emp);
}