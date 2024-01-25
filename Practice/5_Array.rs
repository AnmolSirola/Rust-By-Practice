fn main(){
    let mut arr: [i32; 5] = [1,2,3,4,5];
    arr[4] = 3;
    
    //This will print "3" because the 4th element is changed to 3
    println!("{}", arr[4]);
}