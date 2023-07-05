// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    let mut changed_data = vec![];
    changed_data = data
    .iter()
    .map(|num| num*3 )
    .filter(|num| num > &10)
    .collect();
    
    for datus in changed_data{
        println!("{:?}", datus);
    }
}
