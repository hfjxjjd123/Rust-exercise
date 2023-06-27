// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

fn main() {
    let drink = Drink{
        flavor: Flavor::Sour,
        ounce: 12
    };
    
    show_drink(drink);
}

fn show_drink(drink: Drink){
    //print flavor
    match drink.flavor{
        Flavor::Cool => println!("flavor: Cool"),
        Flavor::Sweet => println!("flavor: Sweet"),
        Flavor::Sour => println!("flavor: Sour"),
    }
    //print ounce
    println!("oz: {:?}", drink.ounce);
}

enum Flavor{
    Cool,
    Sweet,
    Sour
}

struct Drink{
    flavor: Flavor,
    ounce: i32
}