


/* 컴파일 에러 - expected expression, found let statement
fn main() {
    let A = 10;
    let B = 10;

    println!("{:?}", let A = B);
}
*/


//문제없음 -> if let, while let이라는 구문이 정해져있다는 것을 알 수 있음
fn main() {
    let A = 10;
    let B = 20;

    if let A=B{
        println!("what?");
    }
}
