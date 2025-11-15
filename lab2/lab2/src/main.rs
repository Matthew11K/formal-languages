use rand::Rng;
use regex::Regex;
use std::collections::BTreeSet;

fn dfa_accept(word: &str) -> bool {
    let mut state: usize = 0;
    for ch in word.chars() {
        state = match (state, ch) {
            (0, 'a') => 9,   (0, 'b') => 11,
            (1, 'a') => 9,   (1, 'b') => 13,
            (2, 'a') => 4,   (2, 'b') => 11,
            (3, 'a') => 4,   (3, 'b') => 13,
            (4, 'a') => 9,   (4, 'b') => 6,
            (5, 'a') => 10,  (5, 'b') => 5,
            (6, 'a') => 4,   (6, 'b') => 5,
            (7, 'a') => 14,  (7, 'b') => 1,
            (8, 'a') => 14,  (8, 'b') => 0,
            (9, 'a') => 14,  (9, 'b') => 2,
            (10,'a') => 14,  (10,'b') => 3,
            (11,'a') => 14,  (11,'b') => 12,
            (12,'a') => 8,   (12,'b') => 14,
            (13,'a') => 7,   (13,'b') => 12,
            (14, _) => 14,
            _ => 14,
        };
    }

    matches!(state, 0 | 1 | 2 | 3 | 4 | 5 | 6)
}

fn epsilon_closure_nfa(states: &BTreeSet<usize>) -> BTreeSet<usize> {
    let mut closure = states.clone();
    let mut stack: Vec<usize> = states.iter().cloned().collect();
    while let Some(s) = stack.pop() {
        match s {
            6 | 9 => {
                if !closure.contains(&0) {
                    closure.insert(0);
                    stack.push(0);
                }
            }
            _ => {}
        }
    }
    closure
}

fn nfa_accept(word: &str) -> bool {
    let mut current: BTreeSet<usize> = BTreeSet::new();
    current.insert(0);
    current = epsilon_closure_nfa(&current);

    for ch in word.chars() {
        if ch != 'a' && ch != 'b' {
            return false;
        }

        let mut next: BTreeSet<usize> = BTreeSet::new();

        for &s in current.iter() {
            match (s, ch) {
                (0, 'a') => {
                    next.insert(1);
                    next.insert(4);
                }
                (0, 'b') => {
                    next.insert(2);
                }

                (1, 'b') => {
                    next.insert(0);
                }

                (2, 'b') => {
                    next.insert(3);
                }

                (3, 'a') => {
                    next.insert(1);
                }

                (4, 'b') => {
                    next.insert(5);
                }

                (5, 'a') => {
                    next.insert(6);
                }

                (6, 'b') => {
                    next.insert(6);
                    next.insert(7);
                }

                (7, 'a') => {
                    next.insert(8);
                }

                (8, 'b') => {
                    next.insert(9);
                }

                (9, 'b') => {
                    next.insert(7);
                }

                _ => {}
            }
        }

        current = epsilon_closure_nfa(&next);
    }

    let accepting = [0_usize, 6, 9];
    current.iter().any(|s| accepting.contains(s))
}

fn no_aaa_accept(word: &str) -> bool {
    let mut state: u8 = 0;

    for ch in word.chars() {
        match (state, ch) {
            (0, 'a') => state = 1,
            (0, 'b') => state = 0,

            (1, 'a') => state = 2,
            (1, 'b') => state = 0,

            (2, 'a') => state = 3,
            (2, 'b') => state = 0,

            (3, 'a') | (3, 'b') => state = 3,

            (_, _) => return false,
        }
    }

    matches!(state, 0 | 1 | 2)
}

fn pka_accept(word: &str) -> bool {
    nfa_accept(word) && no_aaa_accept(word)
}

fn main() {
    let re_original =
        Regex::new(r"^((bba|a)b|aba(b)*(bab)*)*$").unwrap();
    let re_extended =
        Regex::new(r"^(?:(?:bb)?ab|aba(?:b)*(?:bab)*)*$").unwrap();

    let mut rng = rand::rng();

    for _ in 0..100_000 {
        let len: usize = rng.random_range(0..10);
        let random_string: String = (0..len)
            .map(|_| if rng.random_bool(0.5) { 'a' } else { 'b' })
            .collect();

        let r_orig = re_original.is_match(&random_string);
        let r_ext  = re_extended.is_match(&random_string);
        let r_dfa  = dfa_accept(&random_string);
        let r_nfa  = nfa_accept(&random_string);
        let r_pka  = pka_accept(&random_string);

        assert!(
            r_orig == r_ext &&
            r_ext  == r_dfa &&
            r_dfa  == r_nfa &&
            r_nfa  == r_pka,
            "Mismatch on `{}`: regex={}, ext_regex={}, dfa={}, nfa={}, pka={}",
            random_string, r_orig, r_ext, r_dfa, r_nfa, r_pka
        );
    }

    println!("All tests passed successfully!");
}
