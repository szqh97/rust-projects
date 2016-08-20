fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}
fn add_one(i: i32) -> i32 {
    i + 1
}

fn factory() -> Box<Fn(i32) ->i32> {
    let num = 5;
    Box::new(move |x| x + num)
    
}

pub fn main() {
//    let f = add_one;
//    let answer = call_with_one(&f);
//    assert_eq!(2, answer)
//
    let answer = call_with_one(&add_one);
    assert_eq!(2, answer);

    let f = factory();
    let ans = f(1);
    assert_eq!(6, ans);
}
