mod aaa {
    const X: i32 = 10;

    fn print_aaa() {
        println!("{}", 42);
    }
    mod BBB {
        fn print_bbb() {
            println!("{}", 37);
        }
    }
}

mod ccc {
    fn print_ccc() {
        println!("{}", 25);
    }
}
