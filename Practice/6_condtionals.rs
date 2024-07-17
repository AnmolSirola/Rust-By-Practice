/*fn main(){

    let cond = 2 < 3;

    println!("{}", cond);
}
*/

fn main(){

    let cond = (2 as f32) <= 3.3;

    let con2 = !(true || cond);

    println!("{}", cond2);
}

//control flow

fn main(){

    let food = "cookie"

    if food == "cookie" {
        println!("I like cookies too!");

    } else if food == "fruit"{
        println!("That sounds healthy!");

    } else{
        println!("Oh, that's bad!");
    }
}

fn main(){

    let mut counter = 5;

    while counter >= 1 {
        println!("Counter is {}", counter);
        counter = counter - 1;
    };

    println!("Lift off!");
}

fn main(){

    let mut i = 1;
    
    loop{
        println!("i is {}", i);
        i = i + 1;
    
        if i == 10{
                break;
        }
    }   
}