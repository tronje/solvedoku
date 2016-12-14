extern crate solver;
use solver::variable::*;

fn main() {
    let var = Variable {x: 42, y: 100, domain: vec![1, 3, 3 ,7]};
    println!("{}", var);
}
