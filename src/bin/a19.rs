// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stuffs = HashMap::new();
    stuffs.insert(0,Furniture{kind: "Chairs".to_owned(), nums: 5});
    stuffs.insert(1,Furniture{kind: "Beds".to_owned(), nums: 3});
    stuffs.insert(2,Furniture{kind: "Tables".to_owned(), nums: 2});
    stuffs.insert(3,Furniture{kind: "Couches".to_owned(), nums: 0});

    for (num, furniture) in stuffs{
        if furniture.nums != 0 {
            println!("#{:?} -> {:?} stuffs", num, furniture.nums);
        } else{
            println!("#{:?} out of stocks", num);
        }
        
    }

}

struct Furniture{
    kind: String,
    nums: i32
}

