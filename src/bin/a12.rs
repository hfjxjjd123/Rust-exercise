// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

fn main() {
    let boxy = ShippingBox::create_box(3, 10, Colors::Yellow);
    boxy.print_dimensions();
    boxy.print_weight();
    boxy.print_color();

}

struct ShippingBox{
    dimensions: i32,
    weight: i32,
    color: Colors
}

enum Colors{
    Red,
    Yellow,
    Green
}

impl ShippingBox{
    fn create_box(dimensions: i32, weight: i32, color: Colors) -> Self{
        Self{
            dimensions: dimensions,
            weight: weight,
            color: color
        }
        
    }

    fn print_dimensions(&self){
        println!("dimensions: {:?}", self.dimensions)
    }
    fn print_weight(&self){
        println!("weight: {:?}", self.weight)
    }
    fn print_color(&self){
        match self.color{
            Colors::Red => println!("color: red"),
            Colors::Yellow => println!("color: yellow"),
            Colors::Green => println!("color: green"),
        }
    }
}
