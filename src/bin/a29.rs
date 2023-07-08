
// inner closure
// 자체실습

fn math(a:i32, b:i32, op: Box<dyn Fn(i32, i32)->i32>) -> i32 {
    op(a, b)
}

fn main(){
    println!("{:?}", math(2,2, Box::new(|a,b| a+b)));
    println!("{:?}", math(2,2, Box::new(|a,b| a*b)));
    println!("{:?}", math(2,2, Box::new(|a,b| a/b)));
}