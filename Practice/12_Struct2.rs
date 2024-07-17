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
