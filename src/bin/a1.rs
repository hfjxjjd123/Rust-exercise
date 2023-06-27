// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let first_name = "hakrim";
    let last_name = "lee";

    display_name("first", first_name);
    display_name("last", last_name);
}

fn display_name(mode: &str, name: &str) {
    if mode == "first" {
        println!("{:?}", name);
    } else if mode == "last"{
        println!("{:?}", name);
    } else{
        println!("Wrong Form");
    }
}