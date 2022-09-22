#[macro_use]
extern crate pest_derive;

mod calc;
pub use calc::*;
// use crate::calc::{eval, parse, Calculator, Rule};

// mod calc;

// fn main() {
//     let parse_result = parse("GET_NOW-GET_UPDATE_TIME").unwrap();
//     println!("{:?}", parse_result);
//     println!("{:?}", eval(parse_result));

//     println!("Hello, world!");
// }
