// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
    
    let light = Color::Green;

    show_light_state(light);
}

enum Color{
    Red,
    Green,
    Yellow
}

fn show_light_state(color: Color){
    //R,B,Y 매칭
    match color{
        Color::Red => println!("RED"),
        Color::Green => println!("GREEN"),
        Color::Yellow => println!("YELLOW"),
    }
}
