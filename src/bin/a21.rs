// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        _ => None,
    }
}

fn main() {
    find_user("matt")
    .map(|id| User{ user_id: id, name: "matt".to_owned()})
    .map(|user| println!("{:?}", user));
}
