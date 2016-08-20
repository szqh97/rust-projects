use std::collections::HashMap;
pub fn main() {
    let m = (1..20).fold(1u64, |mul, x| mul * x);
    let m1 = (1..20).map(|x| x+1);
    let m2: Vec<_> = (1..20).filter(|x| x%2 == 0).collect();

    let v = vec![1,2,3,4,5,6];
    let v_take = v.iter()
        .cloned()
        .take(2)
        .collect::<Vec<_>>();
    let v_skip: Vec<_> = v.iter()
        .cloned()
        .skip(2)
        .collect();
    println!("{:?}", v_take);
    println!("{:?}", v_skip);

    let names = vec!["WaySLOG", "Mike", "ELton"];
    let score = vec![60, 80, 100];
    let score_map: HashMap<_,_> = names.iter()
        .zip(score.iter())
        .collect();
    println!("{:?}",score_map);
}
