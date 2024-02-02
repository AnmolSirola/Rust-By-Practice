
struct Person{
    age: i32,
    name: String,
    color: String
}

fn main(){

    let mut persons = vec![
        Person{
            age: 21,
            name: "Anmol".to_string(),
            color: "Blue".to_string(),
        }

        Person{
            age: 22,
            name: "Saum".to_string(),
            color: "Yellow".to_string(),
        }

        Person{
            age: 16,
            name: "Harshi".to_string(),
            color: "Red".to_string(),
        }

        Person{
            age: 20,
            name: "San".to_string(),
            color: "Green".to_string(),
        }

    ];

    for person in persons{
        if person.age<= 10{
            print_color(person);
        }
    }
}

fn print_color(person:Person) {
    println!("name:{}", person.name);
    println!("color:{}", person.color);
}
