
enum Access{
    Full,
}

fn one_two_three() -> (i32, i32, i32) {

    (1, 2, 3)
}

fn main() {

    let numbers = one_two_three();

    let (x, y ,x) = one_two_three();

    println!("{:?} {:?}", x, numbers.1);
    println!("{:?} {:?}", y, numbers.1);
    println!("{:?} {:?}", z, numbers.2);
}