// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery{
    quantity: i32,
    id: i32
}

fn main() {
    let grocery_item = Grocery{
        quantity: 32,
        id: 1234
    };

    print_quantity(&grocery_item);
    print_id(&grocery_item);

}

fn print_quantity(item: &Grocery){
    println!("quantity: {:?}", item.quantity)
}
fn print_id(item: &Grocery){
    println!("id: {:?}",item.id )
}
