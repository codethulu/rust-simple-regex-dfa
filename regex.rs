#[derive(PartialEq)]
enum StateType {
    CHAR,
    LOOP,
}
struct State {
    e: u8,
    accepts: bool,
    kind: StateType
}

impl Solution {
    fn new_nfa(p: &[u8]) -> Vec<State> {
        let mut nfa = Vec::new();
        let mut i = 0;
        while i < p.len() {
            if p[i] == b'*' {
                i += 1;
                continue;
            }
            if i + 1 < p.len() && p[i+1] == b'*' {
                nfa.push(State{e: p[i], accepts: false, kind: StateType::LOOP});
                i += 2;
            } else {
                nfa.push(State{e: p[i], accepts: false, kind: StateType::CHAR});
                i += 1;
            }
        }
        
        for state in nfa.iter_mut().rev() {
            state.accepts = true;
            if let StateType::CHAR = state.kind {
                break;
            }
        }
        
        return nfa;
    }
    
    fn run_nfa(nfa: &[State], s: &[u8]) -> bool {

        //return if a is same as b or a is b'.'
        fn is_same(a: u8, b: u8) -> bool {
            (a == b) || (a == b'.')
        }
        if nfa.is_empty() {
            //if NFA empty and string is empty, then return true
            return s.is_empty();
        } else if s.is_empty() {
            //does NFA accept empty string?
            return (nfa[0].kind == StateType::LOOP && nfa[0].accepts);
        }
        
        let mut curr_states = vec![false; nfa.len()];
        curr_states[0] = true;
        
        for (i, c) in s.iter().enumerate() {
            let is_last_char = (i == s.len() - 1);
            for (j, state) in nfa.iter().enumerate().rev() {
                if !curr_states[j] {
                    continue;
                }
                match state.kind {
                    StateType::CHAR => {
                        curr_states[j] = false;
                        if is_same(state.e, *c) {
                            if state.accepts && is_last_char {
                                return true;
                            } 
                            if j != nfa.len() - 1 {
                                curr_states[j+1] = true;
                            }
                        }
                    }
                    StateType::LOOP => {
                        for k in j..nfa.len() {
                            let next_state = &nfa[k];
                            if let StateType::LOOP = next_state.kind {
                                if is_same(next_state.e, *c) {
                                    if is_last_char && next_state.accepts {
                                        return true;
                                    }
                                    curr_states[k] = true;
                                } else {
                                    curr_states[k] = false;
                                }
                            } else {
                                if next_state.e == b'.' || next_state.e == *c {
                                    if next_state.accepts && is_last_char {
                                        return true;
                                    }
                                    if k != nfa.len() - 1 {
                                        curr_states[k+1] = true;
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
            if is_last_char {
                return false;
            }
        }
        
        return false;
    }
    
    pub fn is_match(s: String, p: String) -> bool {
        let nfa = Solution::new_nfa(p.as_bytes());
        return Solution::run_nfa(&nfa, s.as_bytes());
    }
}