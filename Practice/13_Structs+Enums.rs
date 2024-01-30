enum flavours{
    vanilla,
    chocolate,
    strawberry,
}

struct Drink{
    fla: flavours,
    fluid_ounce: f32
}

fn print_drink_info(drink: Drink){

    match drink.fla{
        flavours::vanilla => println!("Vanilla"),
        flavours::chocolate => println!("Chocolate"),
        flavours::strawberry => println!("Strawberry"),
    }

    println!("{} fluid ounces", drink.fluid_ounce);
}

fn main(){
    let drink1 = Drink{
        fla: flavours::vanilla,
        fluid_ounce: 12.0
    };

    let drink2 = Drink{
        fla: flavours::chocolate,
        fluid_ounce: 16.0
    };

    print_drink_info(drink1);
    print_drink_info(drink2);
}