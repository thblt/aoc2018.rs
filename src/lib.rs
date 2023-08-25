use std::str::FromStr;
use std::fmt::Debug;
use std::io::stdin;

pub mod matrix;
pub use crate::matrix::Matrix;

pub fn input_lines<T: Debug+FromStr>() -> Vec<T>
where <T as FromStr>::Err: Debug
{
    stdin()
    .lines()
    .map(|x| T::from_str(&x.unwrap()).unwrap())
    .collect::<Vec<T>>()

}
