/*
 Requirements:
 - Print 10, 20, "thirty", and 40 in a loop
 - Print the total number of elements in a vector

 Notes:
 - Use a vector to store 4 numbers
 - Iterate through the vector using a for ..in loop
 - Determine whether to print the number or print "thristy" inside a loop
 - Use the .len() function to print the number of elements in a vector 
*/

fn main() {
    // Create a vector to store numbers
    let numbers = vec![10, 20, 30, 40];

    // Iterate through the vector using a for .. in loop
    for num in &numbers {
        // Determine whether to print the number or "thirty"
        if *num == 30 {
            println!("thirty");
        } else {
            println!("{}", num);
        }
    }

    // Print the total number of elements in the vector using .len() function
    println!("Total number of elements: {}", numbers.len());
}
