
enum Discount{
    Percent(f64),
    Flat(i32)
}

struct Ticket{
    event:String,
    price:i32
}

fn main() {
    let n = 6;
    match n {
        3 => println!("three {}", n),
        other => println!("other {}", other) // its just like _
    }

    let discount = Discount::Flat(30);

    match discount{
        Discount::Flat(2) => println!("flat discount is 32"),
        Discount::Flat(amount) => println!("flat discount is {}", amount),
        Discount::Flat(_) => println!("flat discount is is 2"),
        _ => {}
    }
}