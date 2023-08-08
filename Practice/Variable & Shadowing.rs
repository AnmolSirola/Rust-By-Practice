fn main() {
    let x = 4;
    println!("x is:{}" , x);
    {
      let x = 5;    
      println!("x is:{}" , x);
    }
    let x = x + 3;
    println!("x is:{}" , x);
}
// Output
//x is:4
//x is:5
//x is:7
//