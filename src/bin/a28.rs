// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug, PartialEq)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}


struct Shoes(Color);
impl Shoes{
    fn new(color: Color) -> Self{
        Self(color)
    }
    fn print_color(&self){
        println!("shoes color: {:?}", self.0);
    }
}

struct Shirt(Color);
impl Shirt{
    fn new(color: Color) -> Self{
        Self(color)
    }
    fn print_color(&self){
        println!("shirt color: {:?}", self.0);
    }
}

struct Pant(Color);
impl Pant{
    fn new(color: Color) -> Self{
        Self(color)
    }
    fn print_color(&self){
        println!("pant color: {:?}", self.0);
    }
}


fn main(){
    let pant = Pant(Color::Red);
    let shoes = Shoes(Color::White);
    let shirt = Shirt(Color::Green);

    Pant::print_color(&pant);
    Shoes::print_color(&shoes);
    Shirt::print_color(&shirt);

}