/*

Requirements:
- Determine if a customer is able to enter in a theater  
- Restricted entrance require that the age of the customer 
  is at least 21

Notes:
- Use a struct to store at least the age of a customer
- Use a function to determine if a customer can make a restricted entry
- Return a result from the function
- The Err variant should detail the reason why they cannot make a purchase

*/

struct Customer{
    age: i32
}

fn entry(customer:&Customer) -> Result<(), String> {

    if customer.age <=21{
        return Err("age restricted". to_string());
    }

    Ok(())
}

fn main(){
    let customer = Customer{
        age: 20
    };  

    let can_entry = entry(&customer);
    println!("can entry {:?}", can_entry);

}