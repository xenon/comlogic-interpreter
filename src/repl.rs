use std::{
    io::{self, BufRead},
    str::FromStr,
};

pub mod clterm;
pub mod term;

use crate::clterm::CLTerm;
use crate::term::Term;
fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let t = CLTerm::from_str(&l);
                match t {
                    Ok(mut x) => {
                        println!("{}", format!("{}", x));
                        while x.has_redex(clterm::env) {
                            x.reduce(clterm::env);
                            println!(" {}", format!("{}", x));
                        }
                    }
                    Err(e) => println!("Error: {}", format!("{}", e)),
                }
            }
            Err(e) => println!("Line Error?: {}", e),
        }
    }
}
