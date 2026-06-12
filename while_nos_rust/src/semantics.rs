use std::collections::HashMap;
use crate::ast::*;

// The 'state' is a map from variable name (String) to value (i32).
pub type State = HashMap<VarName, i32>;

// solve_a: AExp -> State -> i32
pub fn solve_a(e: &AExp, s: &State) -> i32 {
    match e {
        AExp::Num(m) => *m,
        // Default to 0 if the variable is not found
        AExp::Var(x) => *s.get(x).unwrap_or(&0), 
        AExp::Add(e1, e2) => solve_a(e1, s) + solve_a(e2, s), 
        AExp::Mult(e1, e2) => solve_a(e1, s) * solve_a(e2, s),
        AExp::Sub(e1, e2) => solve_a(e1, s) - solve_a(e2, s),
        AExp::Iand(e1, e2) => solve_a(e1, s) & solve_a(e2, s),

        // Section 3
        AExp::Shl(e1, e2) => solve_a(e1, s) * 2_i32.pow(solve_a(e2, s) as u32),
        AExp::Shr(e1, e2) => solve_a(e1, s) / 2_i32.pow(solve_a(e2, s) as u32), /* It automatically does floor*/
    }
}

// BVal is simply the Rust 'bool' type, mapping to OCaml's "tt" and "ff"
pub type BVal = String;

// solve_b: BExp -> State -> BVal (bool)
pub fn solve_b(e: &BExp, s: &State) -> BVal {
    match e {
        BExp::True => "tt".to_string(), 
        BExp::False => "ff".to_string(), 
        BExp::Neg(e1) => {
            let b1 = solve_b(e1, s);
            if b1 == "tt".to_string() {
                "ff".to_string()
            } else {
                "tt".to_string()
            }
        },
        BExp::Beq(e1, e2) => {
            if solve_b(e1, s) == solve_b(e2, s) {
                "tt".to_string()
            } else {
                "ff".to_string()
            }
        },
        BExp::Aeq(e1, e2) => {
            if solve_a(e1, s) == solve_a(e2, s) {
                "tt".to_string()
            } else {
                "ff".to_string()
            }
        },
        BExp::Gte(e1, e2) => {
            if solve_a(e1, s) >= solve_a(e2, s) {
                "tt".to_string()
            } else {
                "ff".to_string()
            }
        },
        BExp::And(e1, e2) => {
            let b1 = solve_b(e1, s);
            let b2 = solve_b(e2, s);
            if b1 == "tt".to_string() && b2 == "tt".to_string() {
                "tt".to_string()
            } else {
                "ff".to_string()
            }
        },
    }
}

// state update: to get a new state
pub fn update(x: &VarName, e: &AExp, s: &State) -> State {
    let mut new_state = s.clone();
    let value = solve_a(e, s);
    new_state.insert(x.clone(), value);
    new_state
}



// ----------- Test Cases States  --------
// Initial state s0 (x = 1)
pub fn s0() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 1); 
    s
}

// Initial state s1 (x = 5)
pub fn s1() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 5);
    s
}

// Initial state s2 (x = 10, y = 5)
pub fn s2() -> State {
    let mut s = HashMap::new();
    s.insert("x".to_string(), 10);
    s.insert("y".to_string(), 5);
    s
}

// section 7:

// Initial state s3 (a = 5, x = 3)
pub fn s3() -> State {
    let mut s = HashMap::new();
    s.insert("a".to_string(), 5);
    s.insert("x".to_string(), 3);
    s
}

// Initial state s4 (a = 2, b = 8)
pub fn s4() -> State {
    let mut s = HashMap::new();
    s.insert("a".to_string(), 2);
    s.insert("b".to_string(), 8);
    s
}