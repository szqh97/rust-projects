/*
struct RefBody<'a> {
    loc: &'a i32,
}

#[derive(Copy, Clone)]
struct A {
    a: i32,
}
impl A {
    pub fn show(self) {
        println!("{}", self.a);
    }
}
*/

#[derive(Copy, Clone)]
struct A {
    a: i32,
}
impl A {
    pub fn show(&self){
        println!("{}", self.a);
    }

    pub fn add_two(&mut self) {
        self.add_one();
        self.add_one();
        self.show();
    }
    pub fn add_one(&mut self) {
        self.a += 1;
    }
}

fn main() {
    let mut ast = A{a: 12i32};
    ast.show();
    ast.add_two();
    println!("{}", ast.a);
    println!("...................");

    
    
}
