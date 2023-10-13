use std::fmt;
use std::iter::Peekable;
use std::str::{CharIndices, FromStr};

use crate::term::Term;

#[derive(Clone, Debug, PartialEq)]
pub enum CLTerm {
    Empty,
    Atom(String),
    List(Vec<Box<CLTerm>>),
}

pub enum CLSub {
    Sub(usize),
    List(Vec<Box<CLSub>>),
}

pub struct CLFun {
    arity: usize,
    fun: CLSub,
}

#[derive(Debug)]
pub enum CLTermError {
    TooManyOpenParens,
    TooManyCloseParens,
    EmptyTerm,
}

impl Term for CLTerm {
    type Substitution = CLSub;
    type Environment = fn(&str) -> Option<CLFun>;

    fn has_redex(&self, env: Self::Environment) -> bool {
        match self {
            CLTerm::Empty | CLTerm::Atom(_) => false,
            CLTerm::List(v) if v.len() == 0 => false,
            CLTerm::List(v) => {
                let (car, cdr) = v.split_first().unwrap();
                match &**car {
                    CLTerm::Empty => unreachable!(),
                    CLTerm::Atom(a) => {
                        let comb = env(&a);
                        match comb {
                            Some(CLFun { arity: n, .. }) if cdr.len() >= n => true,
                            _ => CLTerm::List(cdr.to_vec()).has_redex(env),
                        }
                    }
                    CLTerm::List(nest_car) => {
                        CLTerm::List([&nest_car, cdr].concat()).has_redex(env)
                    }
                }
            }
        }
    }

    fn substitute(sub: &Self::Substitution, args: &[Box<Self>]) -> Self {
        match sub {
            CLSub::Sub(n) if *n <= args.len() => (*args[(*n as usize)]).clone(),
            CLSub::Sub(_) => unreachable!(),
            CLSub::List(v) => {
                let mut v2 = Vec::new();
                for sub in v {
                    v2.push(Box::new(Self::substitute(sub, args)));
                }
                CLTerm::List(v2)
            }
        }
    }

    fn reduce(&mut self, env: Self::Environment) -> () {
        match self {
            CLTerm::Empty => (),
            CLTerm::Atom(_) => (),
            CLTerm::List(v) if v.len() == 0 => (),
            CLTerm::List(v) => {
                let (car, cdr) = v.split_first().unwrap();
                match &**car {
                    CLTerm::Empty => unreachable!(),
                    CLTerm::Atom(a) => {
                        let comb = env(&a);
                        match comb {
                            Some(CLFun { arity: n, fun: f }) if cdr.len() >= n => {
                                let sub = Self::substitute(&f, &cdr[0..n]);
                                let mut newcdr = cdr.to_vec();
                                if newcdr.len() > n {
                                    newcdr.drain(0..n);
                                    newcdr.insert(0, Box::new(sub));
                                    *self = CLTerm::List(newcdr)
                                } else {
                                    // Prevents extra nestings
                                    *self = sub
                                }
                            }
                            _ => {
                                let mut res = CLTerm::List(cdr.to_vec());
                                res.reduce(env);
                                match res {
                                    CLTerm::Empty => unreachable!(),
                                    CLTerm::Atom(_) => {
                                        let mut v2: Vec<Box<CLTerm>> = Vec::new();
                                        v2.push(Box::new(CLTerm::Atom(a.to_string())));
                                        v2.push(Box::new(res));
                                        *self = CLTerm::List(v2)
                                    }
                                    CLTerm::List(mut v2) => {
                                        v2.insert(0, Box::new(CLTerm::Atom(a.to_string())));
                                        *self = CLTerm::List(v2)
                                    }
                                }
                            }
                        }
                    }
                    CLTerm::List(nest_car) => {
                        *self = CLTerm::List([&nest_car, cdr].concat().to_vec());
                        self.reduce(env);
                    }
                }
            }
        }
    }
}

impl fmt::Display for CLTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn subfmt(s: &CLTerm, f: &mut fmt::Formatter<'_>, outer: bool) -> fmt::Result {
            match &*s {
                CLTerm::Empty => write!(f, ""),
                CLTerm::Atom(x) => write!(f, "{}", x),
                CLTerm::List(v) => {
                    let iter = &mut v.iter();
                    if outer {
                        write!(f, "(")?;
                    }
                    if v.len() > 0 {
                        subfmt(&v[0], f, true)?;
                        iter.next();
                        for term in iter {
                            write!(f, " ")?;
                            subfmt(&term, f, true)?;
                        }
                    }
                    if outer {
                        write!(f, ")")?;
                    }
                    Ok(())
                }
            }
        }
        subfmt(self, f, false)
    }
}

impl fmt::Display for CLTermError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLTermError::TooManyOpenParens => write!(f, "Too many open parens"),
            CLTermError::TooManyCloseParens => write!(f, "Too many close parens"),
            CLTermError::EmptyTerm => write!(f, "Empty or incomplete empty subterm"),
        }
    }
}

impl FromStr for CLTerm {
    type Err = CLTermError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn sub_atom(iter: &mut Peekable<CharIndices>) -> Result<CLTerm, CLTermError> {
            let mut atom: Vec<char> = Vec::new();
            while let Some((_, c)) = iter.peek() {
                match c {
                    ' ' | '\t' | '\n' | '(' | ')' => break,
                    _ => {
                        let (_, ch) = iter.next().unwrap();
                        atom.push(ch);
                    }
                }
            }
            Ok(CLTerm::Atom(atom.into_iter().collect()))
        }
        fn sub_term(
            iter: &mut Peekable<CharIndices>,
            mut num: i32,
        ) -> Result<(CLTerm, i32), CLTermError> {
            let mut res = Vec::new();
            while let Some((_, c)) = iter.peek() {
                match c {
                    ' ' | '\t' | '\n' => {
                        iter.next();
                    }
                    '(' => {
                        iter.next();
                        let (term, val) = sub_term(iter, num + 1)?;
                        num = val;
                        if term == CLTerm::Empty {
                            return Err(CLTermError::EmptyTerm);
                        }
                        res.push(Box::new(term));
                    }
                    ')' => {
                        num -= 1;
                        iter.next();
                        break;
                    }
                    _ => {
                        let term = sub_atom(iter)?;
                        res.push(Box::new(term));
                    }
                }
            }
            if res.len() == 0 {
                Ok((CLTerm::Empty, num))
            } else if res.len() == 1 {
                let val = res.pop().unwrap();
                Ok((*val, num))
            } else {
                Ok((CLTerm::List(res), num))
            }
        }
        let (term, val) = sub_term(&mut s.char_indices().peekable(), 0)?;
        return if val < 0 {
            Err(CLTermError::TooManyCloseParens)
        } else if val > 0 {
            Err(CLTermError::TooManyOpenParens)
        } else {
            Ok(term)
        };
    }
}

pub fn env(s: &str) -> Option<CLFun> {
    match s {
        "I" => Some(CLFun {
            arity: 1,
            fun: CLSub::Sub(0),
        }),
        "K" => Some(CLFun {
            arity: 2,
            fun: CLSub::Sub(0),
        }),
        "S" => {
            let mut v1 = Vec::new();
            v1.push(Box::new(CLSub::Sub(0)));
            v1.push(Box::new(CLSub::Sub(2)));
            let mut v2 = Vec::new();
            v2.push(Box::new(CLSub::Sub(1)));
            v2.push(Box::new(CLSub::Sub(2)));
            let inner = CLSub::List(v2);
            v1.push(Box::new(inner));
            let outer = CLSub::List(v1);
            Some(CLFun {
                arity: 3,
                fun: outer,
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod reductions {
    use super::*;
    #[test]
    fn epsilon() {
        let input = CLTerm::from_str("").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn one_term() {
        let input = CLTerm::from_str("x").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn i_combinator() {
        let input = CLTerm::from_str("I x").unwrap();
        let output = CLTerm::from_str("x").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }

    #[test]
    fn i_combinator_arity_0() {
        let input = CLTerm::from_str("I").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn i_combinator_arity_1() {
        let input = CLTerm::from_str("I x y z").unwrap();
        let output = CLTerm::from_str("x y z").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }

    #[test]
    fn k_combinator() {
        let input = CLTerm::from_str("K x y").unwrap();
        let output = CLTerm::from_str("x").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }

    #[test]
    fn k_combinator_arity_0() {
        let input = CLTerm::from_str("K").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn k_combinator_arity_1() {
        let input = CLTerm::from_str("K x").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn k_combinator_arity_2() {
        let input = CLTerm::from_str("K x y z a").unwrap();
        let output = CLTerm::from_str("x z a").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }

    #[test]
    fn s_combinator_0() {
        let input = CLTerm::from_str("S x y z").unwrap();
        let output = CLTerm::from_str("x z (y z)").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }

    #[test]
    fn s_combinator_1() {
        let input = CLTerm::from_str("S (a b c) (S x y z) (d e f)").unwrap();
        let output = CLTerm::from_str("(a b c) (d e f) ((S x y z) (d e f))").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }

    #[test]
    fn s_combinator_arity_0() {
        let input = CLTerm::from_str("S").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn s_combinator_arity_1() {
        let input = CLTerm::from_str("S x").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn s_combinator_arity_2() {
        let input = CLTerm::from_str("S x y").unwrap();
        let mut out = input.clone();
        out.reduce(env);
        assert!(!input.has_redex(env));
        assert_eq!(input, out);
    }

    #[test]
    fn s_combinator_arity_3() {
        let input = CLTerm::from_str("S x y z a b c").unwrap();
        let output = CLTerm::from_str("(x z (y z)) a b c").unwrap();
        let mut res = input.clone();
        res.reduce(env);
        assert!(input.has_redex(env));
        assert_eq!(output, res);
    }
}

#[cfg(test)]
mod errors {
    use super::*;
    #[test]
    fn no_terms_0() {
        let input = CLTerm::from_str("()");
        assert!(input.is_err());
    }

    #[test]
    fn no_terms_1() {
        let input = CLTerm::from_str("((()))");
        assert!(input.is_err());
    }

    #[test]
    fn not_closed_0() {
        let input = CLTerm::from_str("(");
        assert!(input.is_err());
    }

    #[test]
    fn not_closed_1() {
        let input = CLTerm::from_str("(x y ( z )");
        assert!(input.is_err());
    }

    #[test]
    fn not_closed_2() {
        let input = CLTerm::from_str("((((((x)))))");
        assert!(input.is_err());
    }

    #[test]
    fn not_opened_0() {
        let input = CLTerm::from_str(")");
        assert!(input.is_err());
    }

    #[test]
    fn not_opened_1() {
        let input = CLTerm::from_str("(x y ( z )))");
        assert!(input.is_err());
    }

    #[test]
    fn not_opened_2() {
        let input = CLTerm::from_str("(((((x))))))");
        assert!(input.is_err());
    }

    #[test]
    fn complex_0() {
        let input = CLTerm::from_str("S (x) (y z) (a b c) (d");
        assert!(input.is_err());
    }

    #[test]
    fn complex_1() {
        let input = CLTerm::from_str("S (x) (y z)) (a b c)");
        assert!(input.is_err());
    }
}
