


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

/* 용례
fn main(){
    let lists = vec![1, 2, 3, 4, 5];
    let mut lists_iter = lists.iter();

    while let Some(num) = lists_iter.next() {
        println!("{:?}", num);
    }
}
*/