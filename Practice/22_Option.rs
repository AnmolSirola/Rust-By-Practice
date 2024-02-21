/* 
enum Option<T> {
    Some(T),
    None
}

let some_value: Option<i32> = Some(3);
let none_value: Option<i32> = None;

-------------------------------------------------------------
*/

struct Survey {
    q1: Option<i32>,
    q2: Option<String>,
    q3: Option<bool>,
}

fn main() {
    // Creating an instance of the Survey struct
    let survey = Survey {
        q1: Some(10),
        q2: None,
        q3: Some(true),
    };

    // Matching on the q1 field of the Survey struct
    match survey.q1 {
        Some(data) => {
            // If q1 has Some value, print the response
            println!("q1 response is {}", data);
        }
        None => {
            // If q1 is None, print a message indicating no response
            println!("no response for q1");
        }
    };

    // Matching on the q2 field of the Survey struct
    match survey.q2 {
        Some(data) => {
            // If q2 has Some value, print the response
            println!("q2 response is {}", data);
        }
        None => {
            // If q2 is None, print a message indicating no response
            println!("no response for q2");
        }
    };

    // Matching on the q3 field of the Survey struct
    match survey.q3 {
        Some(data) => {
            // If q3 has Some value, print the response
            println!("q3 response is {}", data);
        }
        None => {
            // If q3 is None, print a message indicating no response
            println!("no response for q3");
        }
    };
}
