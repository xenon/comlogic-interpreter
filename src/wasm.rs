use crate::clterm::env;
use crate::clterm::CLTerm;
use crate::term::Term;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CLTermWasm {
    term: CLTerm,
}

#[wasm_bindgen]
impl CLTermWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        CLTermWasm {
            term: CLTerm::from_str("").expect("This should never be an error."),
        }
    }
    #[wasm_bindgen]
    pub fn from_str(term: &str) -> Result<CLTermWasm, JsValue> {
        let term = CLTerm::from_str(term);
        match term {
            Ok(t) => Ok(CLTermWasm { term: t }),
            Err(_) => Err(JsValue::NULL),
        }
    }
    #[wasm_bindgen]
    pub fn to_str(&self) -> String {
        format!("{}", self.term)
    }

    #[wasm_bindgen]
    pub fn has_redex(&self) -> bool {
        self.term.has_redex(env)
    }
    #[wasm_bindgen]
    pub fn reduce(&mut self) -> () {
        self.term.reduce(env)
    }
    #[wasm_bindgen]
    pub fn reduce_n(&mut self, times: u32) {
        if times == 0 {
            return;
        }
        for _ in 0..times {
            if self.has_redex() {
                self.reduce();
            } else {
                break;
            }
        }
    }
}
