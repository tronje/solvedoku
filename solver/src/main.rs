extern crate solver;
use solver::csp as csp;

fn main() {
    let var = csp::Variable::new(4, 2, 0, vec![1, 3, 3, 7]);
    println!("{}", var);
}
