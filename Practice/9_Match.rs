fn main(){
    let some_bool: bool = true:

    match some_bool{
        true => println!("It is true"),
        false => println!("It is false"),
    }
}

fn main(){

    let some_int: i32 = 5;

    match some_int{
        1 => println!("It is one"),
        2 => println!("It is two"),
        3 => println!("It is three"),
        4 => println!("It is four"),
        5 => println!("It is five"),
        _ => println!("It is something else"), // _ indicates anything else other than the above in our case other than 1,2,3,4,5
    }
}