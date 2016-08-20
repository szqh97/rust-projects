fn main() {
    let a: i32;
    a = 100;
    let b  = a;
    println!("{}", a);
    println!("{}", b);
    //let a:String = String::from("aaa");
    let b = a;
    println!("{}", a);

    println!("{:?}", foo("aa", "bb"));
}

fn foo<'a, 'b: 'a>(x: &'a str, y: &'b str)-> &'a str {
    if true {
        x
    }
    else {
        y
    }
    
}
