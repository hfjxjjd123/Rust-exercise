// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn main() {
    let peoples = vec![
        People{
            name: "hakrim".to_owned(),
            age: 24,
            favorite_color: "noeul".to_owned()
        },
        People{
            name: "songess".to_owned(),
            age: 8,
            favorite_color: "yellow".to_owned()
        },
        People{
            name: "Cho gu".to_owned(),
            age: 5,
            favorite_color: "black".to_owned()
        }
    ];

    for people in &peoples{
        if people.age <= 10{
            println!("name: {:?} \nage: {:?} \nfavoirite color: {:?}", &people.name, people.age, &people.favorite_color);
            println!();
        }
    }
}

struct People{
    name: String,
    age: i32,
    favorite_color: String,
}

