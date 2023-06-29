// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

/// Manage Tickets
fn main() {
    let tickets: Vec<Ticket> = vec![
        generate_ticket("VIP", "Hakrim Lee"),
        generate_ticket("Backstage", "Hippy Brown"),
        generate_ticket("Standard", ""),
    ];
    
    for ticket in tickets {
        println!("{:?}", ticket);
    }

}

/// spec for Ticket struct
#[derive(Debug)]
struct Ticket{
    seat: Seat,
    price: f64,
}
impl Ticket{
    fn generate(seat: Seat, price: f64) -> Self{
        Self{
            seat: seat,
            price: price
        }
    }
}

/// spec for Seat enum
#[derive(Debug)]
enum Seat {
    Backstage(String),
    Vip(String),
    Standard
}

/// ticket 등록 함수
fn generate_ticket(seat: &str, name: &str) -> Ticket {
    match seat{
        "VIP" => Ticket::generate(Seat::Vip(String::from(name)), 132.99),
        "Backstage" => Ticket::generate(Seat::Backstage(String::from(name)), 59.99),
        "Standard" => Ticket::generate(Seat::Standard, 23.99),
        other => Ticket::generate(Seat::Standard, 23.99),
    }
}