
//lifetime 실습

fn main(){
    let ans1;

        let answer = Answer::Yes;
        ans1 = Form{ answer: &answer };

    println!("{:?}", ans1 );

}

#[derive(Debug)]
enum Answer{
    Yes,
    No
}

#[derive(Debug)]
struct Form<'a>{
    answer: &'a Answer
}