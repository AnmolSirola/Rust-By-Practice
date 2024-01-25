
// You can place the test function above, below anywhere in the code. 
 
fn test(){   //2
    println!("Test has been called..");
}

fn main(){   //1 
    println!("Hello World");  
    test();
    add_Numbers(20, 30);  
}


fn add_Numbers(a: i32, b: i32){   //3 
    println!("The sum is: {}", x + y);
}