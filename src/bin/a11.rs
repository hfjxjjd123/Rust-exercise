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

fn main(){
    let item = Item{
        quantity: 12,
        id: 92
    };

    show_id(&item);
    show_quantity(&item);
}

struct Item{
    quantity: i32,
    id: i32
}

fn show_quantity(item: &Item){
    println!("quantity: {:?}", item.quantity);
}

fn show_id(item: &Item){
    println!("id: {:?}", item.id);
}