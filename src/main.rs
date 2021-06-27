use std::io::{self, BufRead};
use std::str::FromStr;

mod clterm;
mod term;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                use clterm::CLTerm;
                let t = CLTerm::from_str(&l);
                match t {
                    Ok(mut x) => {
                        use term::Term;
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
