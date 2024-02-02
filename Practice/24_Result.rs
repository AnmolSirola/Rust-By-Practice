
enum Result<T,E>{
    Ok(T),
    Err(E)
}

let ok_value: Result<i32, String> = Ok(6);
let err_value: Result<i32, String> = Err("Some error occured" .to_string());

-----------------------------------------------------------------------------------------------------

