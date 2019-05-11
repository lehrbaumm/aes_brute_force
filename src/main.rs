extern crate rayon;
extern crate crypto;
extern crate hex;
extern crate itertools;

use rayon::prelude::*;
use itertools::itertools;

fn main() {
    let first_block = "833437EB20E7BF555B9193326DE6E9E9";
    let hex_first_block = hex::decode(first_block).unwrap();
    println!("{:?}", hex_first_block);
    let all_u8 = (0..127 as u8).into_par_iter()
        .map(|i| {
            i
        }).collect::<Vec<u8>>();
    let bruteforce_range: Vec<[u8; 128]> = Vec::new();
    let combin = bruteforce_range.into_par_iter()
        .combinations(4).collect::<Vec<[u8;4]>>();
    println!("{:?}", all_u8);
    println!("Hello, world!");
}
