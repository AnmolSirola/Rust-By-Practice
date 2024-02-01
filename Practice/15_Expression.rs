
fn main(){

    let my_num = 3;

    let is_it_5 = if my _num < 5{
        true
    } else{
        false
    };

    let is_it_5 = my_num < 5;
}
------------------------------------------------------------
fn main(){

    let my_num = 3;

    let message = match my_num {
        1 => "hello",
        _ => "GoodBye",
    };
}

---------------------------------------------------------------
enum Menu{
    Burger,
    Fries,
    Drink,
 }    
    fn main(){

        let paid = true;
        let item = Menu::Drink;
        let drink_type = "water";
        let order_placed = match item{
            Menu::Drink => {
                if drink_type = "water"{
                    true 
                }else{
                        false
                    }
            },
            _ => true,
        };
    }   
------------------------------------------------------------

enum Access{
    Admin,
    Manager,
    Staff,
    Guest,
}


fn main(){

    //secret file: Admin
    let trying_to_access = Access:Guest;
    let can_acess =  match trying_to_access{
        Access::Admin = true,
        _=> false
    };

    println!("Can access {}", can_access);
}
