extern crate qbf;

use qbf::parser;
use qbf::introduce;

use std::fs::File;
use std::io::Read;

use qbf::problem::Solution;
use qbf::expand_solve::solve;

fn main() {
    std::thread::Builder::new().stack_size(8*1024*1024*1024).spawn(|| {
        let args: Vec<_> = std::env::args().collect();
        if !args.len() < 2 {
            panic!("Expected filename");
        }

        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let parsed = parser::parse(s.as_ref());
        let qbf = introduce::construct_problem(parsed);

        match solve(qbf) {
            Solution::Sat => println!("sat"),
            Solution::Unsat => println!("unsat")
        }
    }).unwrap().join().unwrap();
}
