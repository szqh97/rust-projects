pub fn main() {
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        .filter(|&(idx, _)| idx %2 == 0)
        .map(|(idx, val)| val)
        .fold(0u64, |sum, acm| sum + acm);
    println!("{:?}", val);


}
