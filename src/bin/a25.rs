// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

fn main() {
    perimeter(Triangle{a: 10, b: 20, c:30});
    perimeter(Square{side:15});

}

fn perimeter(figure: impl Figure){
    println!("perimeter: {:?}", figure.get_perimeter());
}

trait Figure{
    fn get_perimeter(&self) -> i32;
}

struct Triangle{
    a: i32,
    b: i32, 
    c: i32
}
impl Figure for Triangle{
    fn get_perimeter(&self) -> i32{
        self.a + self.b + self.c
    }
}

struct Square{
    side: i32
}
impl Figure for Square{
    fn get_perimeter(&self) -> i32{
        4 * self.side
    }
}