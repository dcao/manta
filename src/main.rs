use rayon::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let vec: &[i32] = &[1, 2, 4, 6, 10];
    let res: i32 = vec.par_iter().map(|&i| i * i).sum();

    println!("{} {:#?}", res, args);
}
