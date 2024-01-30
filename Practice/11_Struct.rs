
struct ShippingBox{
    depth: i32,
    width: i32,
    height: i32,
}

let my_box = ShippingBox{
    depth: 10,
    width: 20,
    height: 30,
};

let tall = my_box.height;
println!("The height of the box is {}.", tall);