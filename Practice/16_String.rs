struct User{
    user_name: String,
    id:i32
}

fn print_name(user_name:&str) {
    println!("user ame: {}", user_name);
}

fn main(){

    let mut user_list = vec![
        User{
            user_name: "jhon".to_string(),
            id: 0
        },
        User{
            user_name: "jack",to_owned(),
            id: 1
        },
        User{
            user_name:String::new("mike"),
            id: 2
        }

    ];

    for user in user_list{
        print_name(&user.user_name);
        println!("user Id {}", user.id);
    }
}
