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
    let boxy = Box::generate_box(80, 7, Color::Black);

    boxy.show_vol();
    boxy.show_weight();
    boxy.show_color();
    boxy.show_props();
}

struct Box{
    volume: i32,
    weight: i32,
    color: Color
}
impl Box{
    fn generate_box(vol: i32, weight: i32, color: Color) -> Self{
        Self {
            volume: vol,
            weight: weight,
            color: color,
        }
    }

    fn show_vol(&self){
        println!("volume: {:?}L", self.volume);
    }    
    fn show_weight(&self){
        println!("weight: {:?}kg", self.weight);
    }
    fn show_color(&self){
        self.color.print();
    }
    fn show_props(&self){
        self.show_vol();
        self.show_weight();
        self.show_color();
    }
}

enum Color{
    Brown,
    Black,
    Purple
}
impl Color{
    fn print(&self){
        let color = match self{
            Color::Brown => "brown",
            Color::Black => "black",
            Color::Purple => "purple",
        };

        println!("color: {:?}", color);
    }
}