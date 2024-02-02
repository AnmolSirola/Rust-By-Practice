
enum Option<T> {
    Some(T),
    None
}

let some_value: Option<i32> = Some(3);
let none_value: Option<i32> = None;

-------------------------------------------------------------

struct Survey{
    q1: Option<i32>,
    q2: Option<String>,
    q3: Option<bool>
}

fn main(){
    let survey = Survey{
        q1: Some(10),
        q2: None,
        q3: Some(true)
    };

    match survey.q1 {
        Some(data) => {
            println!("q1 response is {}", data)
        },
        None => println!("no response for q1")
    };

    match survey.q2 {
        Some(data) => {
            println!("q2 response is {}", data)
        },
        None => println!("no response for q2")
    };

    match survey.q3 {
        Some(data) => {
            println!("q3 response is {}", data)
        },
        None => println!("no response for q3")
    };
}