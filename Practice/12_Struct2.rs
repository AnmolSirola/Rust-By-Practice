fn main{
    println("Hello, world!")

    let item1 = GroceryItem{
        stock: 5,
        price: 60.0
    };

    println( "The quantity of item1 is: {} the price of the item is {}.", item1.stock, item1.price);
}

struct GroceryItem{
    stock: i32,
    price: f32,
}

struct Demo{
    f1: i32,
    f2: f32,
    f3: String,
    f4: bool,
}