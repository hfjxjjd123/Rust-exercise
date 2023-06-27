// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main(){
    let a = 10;
    let b = 99;

    print(sum(a,b));

}

fn print(num: i32){
    println!("{:?}", num);
}

fn sum(a: i32, b: i32) -> i32 {
    return a+b;
}


