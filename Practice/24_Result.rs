
enum Result<T,E>{
    Ok(T),
    Err(E)
}

let ok_value: Result<i32, String> = Ok(6);
let err_value: Result<i32, String> = Err("Some error occured" .to_string());

-----------------------------------------------------------------------------------------------------

fn main(){

   // let menu_choice = get_choice("start"); 1st

   // let menu_choice:Result<MenuChoice, String> = get_choice("start"); 2nd
   // println!("menu choice is{:?}", menu_choice);

    let menu_choice:Result<MenuChoice, String> = get_choice("mainmenu");
    match menu_choice{
        Ok(inner_menu_choice) => {
            print_choice(&inner_menu_choice);
        },
        Err(e) =>{
            println!("{:?}", e)
        }
    }

}

#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    Start,
    Quit
}

fn get_choice(choice: &str) -> Result<MenuChoice,String>{  
    match choice{
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("wrong_choice".to_string())
    }
       
}

fn print_choice(menu::&MenuChoice) {
    println!("menu choice is {:?}", menu);
}