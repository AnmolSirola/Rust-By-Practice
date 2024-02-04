
use std::io;

fn read_input() -> io::result<String>{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn main(){
   
 /*  
    let result = read_input();
    match result {
        Ok(data) =>{
            println!("User inout is {:?}", data)
        },

        Err(e) => println!("{:?}", e)
    }
*/ 

    let mut counter = 0;
    let mut user_input = vec![];
    while counter <=2 {
        match read_input(){
            Ok(data) =>{
                user_input.push(data);
            }
            Err(e) =>{
                println!("{:?}", e)
            }
        }
    }

    for i in user_input{
        println!("User input is {:?} anb upper case is {:?}", i, i.to_uppercase())
    }
}