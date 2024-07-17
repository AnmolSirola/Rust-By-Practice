fn main() {
    // Creating a vector with initial values
    let mut numbers: Vec<i32> = vec![10, 20, 30, 40];

    // Printing the original vector
    println!("Original Vector: {:?}", numbers);

    // Accessing elements in a vector using indexing
    let second_element = numbers[1];
    println!("Second Element: {}", second_element);

    // Modifying an element in the vector
    numbers[2] = 35;
    println!("Modified Vector: {:?}", numbers);

    // Adding an element to the end of the vector
    numbers.push(50);
    println!("Vector after push: {:?}", numbers);

    // Removing an element from the vector
    let removed_element = numbers.pop();
    println!("Vector after pop: {:?}, Removed Element: {:?}", numbers, removed_element);

    // Iterating through the vector
    println!("Iterating through the Vector:");
    for num in &numbers {
        println!("{}", num);
    }

    // Using .len() to get the number of elements in the vector
    let num_elements = numbers.len();
    println!("Number of Elements in the Vector: {}", num_elements);

    // Creating a vector with a repeated value
    let repeated_vector: Vec<i32> = vec![5; 3]; // Creates a vector with three elements, all set to 5
    println!("Repeated Vector: {:?}", repeated_vector);
}
