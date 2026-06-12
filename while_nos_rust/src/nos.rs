use crate::ast::*;
use crate::semantics::*;

// The main Natural Operational Semantics function:
// nos: (Stm, State) -> State
pub fn nos(c: (Stm, State)) -> State {
    let (stm, state) = c;

    match stm {
        // Assignment: [ass]
        Stm::Ass(x, e) => update(&x, &e, &state),

        // Skip: [skip]
        Stm::Skip => state,

        // Composition: [comp]
        Stm::Comp(s1, s2) => {
            let s_tag = nos(((*s1).clone(), state));
            nos(((*s2).clone(), s_tag))
        }

        // If: [if_tt] and [if_ff]
        Stm::If(b, s1, s2) => {
            if solve_b(&b, &state) == "tt".to_string() {
                nos((*s1, state))
            } else {
                nos((*s2, state))
            }
        }

        // While: [while_tt] and [while_ff]
        Stm::While(b, s) => {
            if solve_b(&b, &state) == "tt".to_string() {
                let s_tag = nos(((*s).clone(), state));
                nos((Stm::While(b, s), s_tag))
            } else {
                state
            }
        }

        // DoWhile: [do_while_tt] and [do_while_ff]
        Stm::DoWhile(s, b) => {
            // calc s'
            let s_tag = nos(((*s).clone(), state));
            if solve_b(&b, &s_tag) == "tt".to_string() {
                // if B[s'] == tt - run the DoWhile on s'
                nos((Stm::DoWhile(s, b), s_tag))
            } else {
                // o.w, just run statement on s (s') and return it
                s_tag
            }
        }
    }
}