#[derive(Debug)]
struct Person<'a> {
    age: &'a u8,
}

fn main() {
   let x = 20u8;
   let sb = Person{
       age: &x,
   };

   println!("{:?}", sb);

   let mut num = 5;
   {
       let plus_num = |x: i32| x + num;
       let z = plus_num(11);
   }
   let y = &mut num;

   let nums = vec![1,2,3];

   let takes_nums = move ||nums;
}
