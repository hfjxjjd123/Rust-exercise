// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

fn main() {
    let customer1 = Customer{age: 28};
    let valid = purchase(customer1);

    match valid{
        Ok(_) => println!("clean"),
        Err(e) => println!("{:?}", e)
    }
}

//구조체
struct Customer{
    age: i32,
}

//함수
fn purchase(customer: Customer) -> Result<(), String> {
    if customer.age < 21{
        Err("not enough age!!".to_owned())
    }else{
        Ok(())
    }
}
