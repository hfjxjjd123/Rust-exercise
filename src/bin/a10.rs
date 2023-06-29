// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main(){
    let num = 77;
    let is_over_100 = num>100 ;

    print_size(is_over_100);
}

fn print_size(is_over_100: bool){
    println!("Is over than 100?: {:?}", is_over_100);
}