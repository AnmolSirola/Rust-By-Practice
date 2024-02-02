/*
Requirements:
Print out the details of a student's locker assignment
Lockers use numbers and are options for students

Notes:
Use a struct containing the student's name and locker assignment
The locker assignment should use an Option<i32> // number
*/

struct Locker{
    name: String,
    locker: Option<i32>
}

fn main(){

    let student = Locker{
        name: "jhon".to_string(),
        locker: Some(3)
    };

    match student.locker {
        Some(data) =>{
            println!("locker id is {:?}", data)
        },

        None => println!("locker has not been assigned")
    }
}