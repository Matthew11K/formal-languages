use rand::prelude::*;
use std::collections::{HashSet, VecDeque};

pub type Rule = (String, String);
pub type Rules = Vec<Rule>;

pub mod cfg {
    use super::Rules;

    pub const ALPHABET: &[u8] = b"abdc";

    pub const RNG_SEED: u64 = 123;
    pub const NUM_TRIALS: usize = 2000;

    pub const GEN_MIN_LEN: usize = 6;
    pub const GEN_MAX_LEN: usize = 24;

    pub const PAIR_MIN_STEPS: usize = 5;
    pub const CHAIN_MAX_STEPS: usize = 12;
    pub const PAIR_TRIES: usize = 200;

    pub const MAX_DEPTH: usize = 200;
    pub const MAX_VISITED: usize = 2_000_000;

    pub const KB_TAIL_MAX: usize = 6;

    pub fn rules_t() -> Rules {
        vec![
            ("caa".into(), "ac".into()),
            ("acb".into(), "adb".into()),
            ("ad".into(), "daaa".into()),
            ("bd".into(), "bc".into()),
        ]
    }

    fn extend_tail(r: &mut Rules, n: usize) {
        let mut xs = vec![String::new()];
        for _ in 0..n {
            let mut next = Vec::new();
            for s in &xs {
                let mut c = s.clone();
                c.push('c');
                next.push(c);
                let mut d = s.clone();
                d.push('d');
                next.push(d);
            }
            xs.extend(next);
        }
        xs.sort_unstable_by(|a, b| (a.len(), a).cmp(&(b.len(), b)));
        xs.dedup();
        for x in xs {
            let lhs = format!("{x}cdcb");
            let rhs = format!("{x}cddb");
            r.push((lhs, rhs));
        }
    }

    pub fn rules_tprime() -> Rules {
        let mut r = vec![
            ("caa".into(), "ac".into()),
            ("acb".into(), "adb".into()),
            ("daaa".into(), "ad".into()),
            ("bc".into(), "bd".into()),
            ("accb".into(), "acdb".into()),
            ("adcb".into(), "addb".into()),
            ("bdaa".into(), "bac".into()),
        ];
        extend_tail(&mut r, KB_TAIL_MAX);
        crate::dedup(&r)
    }
}

pub fn dedup(rules: &[(String, String)]) -> Rules {
    let mut seen = HashSet::<(String, String)>::new();
    let mut out = Vec::new();
    for (l, r) in rules {
        let k = (l.clone(), r.clone());
        if seen.insert(k.clone()) {
            out.push(k);
        }
    }
    out
}

pub fn step_forward(word: &str, rules: &Rules) -> Vec<String> {
    let mut out = HashSet::<String>::new();
    for (lhs, rhs) in rules {
        let mut i = 0usize;
        while let Some(rel) = word[i..].find(lhs) {
            let p = i + rel;
            let mut s = String::with_capacity(word.len() - lhs.len() + rhs.len());
            s.push_str(&word[..p]);
            s.push_str(rhs);
            s.push_str(&word[p + lhs.len()..]);
            out.insert(s);
            i = p + 1;
        }
    }
    out.remove(word);
    let mut v: Vec<String> = out.into_iter().collect();
    v.sort_unstable();
    v
}

pub fn step_undirected(word: &str, rules: &Rules) -> Vec<String> {
    let mut out = HashSet::<String>::new();
    for (lhs, rhs) in rules {
        let mut i = 0usize;
        while let Some(rel) = word[i..].find(lhs) {
            let p = i + rel;
            let mut s = String::with_capacity(word.len() - lhs.len() + rhs.len());
            s.push_str(&word[..p]);
            s.push_str(rhs);
            s.push_str(&word[p + lhs.len()..]);
            out.insert(s);
            i = p + 1;
        }
        let mut j = 0usize;
        while let Some(rel) = word[j..].find(rhs) {
            let p = j + rel;
            let mut s = String::with_capacity(word.len() - rhs.len() + lhs.len());
            s.push_str(&word[..p]);
            s.push_str(lhs);
            s.push_str(&word[p + rhs.len()..]);
            out.insert(s);
            j = p + 1;
        }
    }
    out.remove(word);
    let mut v: Vec<String> = out.into_iter().collect();
    v.sort_unstable();
    v
}

pub fn random_word(min_len: usize, max_len: usize, rng: &mut StdRng) -> String {
    let len = rng.random_range(min_len..=max_len);
    let mut s = String::with_capacity(len);
    for _ in 0..len {
        let idx = rng.random_range(0..crate::cfg::ALPHABET.len());
        s.push(crate::cfg::ALPHABET[idx] as char);
    }
    s
}

pub fn random_chain_t(
    w0: &str,
    rules_t: &Rules,
    min_steps: usize,
    max_steps: usize,
    rng: &mut StdRng,
) -> Vec<String> {
    let mut w = w0.to_string();
    let mut chain = vec![w.clone()];
    let steps = rng.random_range(min_steps..=max_steps);
    for _ in 0..steps {
        let ns = step_forward(&w, rules_t);
        if ns.is_empty() {
            break;
        }
        let pick = rng.random_range(0..ns.len());
        w = ns[pick].clone();
        chain.push(w.clone());
    }
    chain
}

pub fn produce_pair(
    rng: &mut StdRng,
    rules_t: &Rules,
    min_len: usize,
    max_len: usize,
    need_steps: usize,
    max_steps: usize,
    attempts: usize,
) -> Option<(String, String)> {
    for _ in 0..attempts {
        let w0 = random_word(min_len, max_len, rng);
        let ch = random_chain_t(&w0, rules_t, need_steps, max_steps, rng);
        if ch.len() >= need_steps + 1 {
            let w1 = ch.last().unwrap().clone();
            if w1 != w0 {
                return Some((w0, w1));
            }
        }
    }
    None
}

pub fn meet_bfs(
    a: &str,
    b: &str,
    rules: &Rules,
    max_depth: usize,
    max_nodes: usize,
) -> bool {
    if a == b {
        return true;
    }
    let mut ql = VecDeque::new();
    let mut qr = VecDeque::new();
    let mut vl = HashSet::<String>::new();
    let mut vr = HashSet::<String>::new();
    ql.push_back(a.to_string());
    qr.push_back(b.to_string());
    vl.insert(a.to_string());
    vr.insert(b.to_string());
    let mut depth = 0usize;
    let mut seen = 0usize;
    while !ql.is_empty() && !qr.is_empty() && depth < max_depth {
        depth += 1;
        let l = ql.len();
        for _ in 0..l {
            let u = ql.pop_front().unwrap();
            for v in step_undirected(&u, rules) {
                if seen >= max_nodes {
                    return false;
                }
                seen += 1;
                if !vl.insert(v.clone()) {
                    continue;
                }
                if vr.contains(&v) {
                    return true;
                }
                ql.push_back(v);
            }
        }
        let r = qr.len();
        for _ in 0..r {
            let u = qr.pop_front().unwrap();
            for v in step_undirected(&u, rules) {
                if seen >= max_nodes {
                    return false;
                }
                seen += 1;
                if !vr.insert(v.clone()) {
                    continue;
                }
                if vl.contains(&v) {
                    return true;
                }
                qr.push_back(v);
            }
        }
    }
    false
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InvSig {
    pub b: usize,
    pub cd: usize,
    pub bx: String,
}

pub fn signature(w: &str) -> InvSig {
    let mut b = 0usize;
    let mut cd = 0usize;
    let mut bx = String::new();
    for ch in w.bytes() {
        match ch {
            b'b' => {
                b += 1;
                bx.push('b');
            }
            b'c' | b'd' => {
                cd += 1;
                bx.push('x');
            }
            _ => {}
        }
    }
    InvSig { b, cd, bx }
}

pub fn unit_invariants_ok(rules: &Rules) -> bool {
    rules.iter().all(|(l, r)| signature(l) == signature(r))
}

pub fn fuzz_sig_once(
    rules: &Rules,
    rng: &mut StdRng,
    min_len: usize,
    max_len: usize,
    min_steps: usize,
    max_steps: usize,
) -> bool {
    let w0 = random_word(min_len, max_len, rng);
    let chain = random_chain_t(&w0, rules, min_steps, max_steps, rng);
    let s0 = signature(&chain[0]);
    chain[1..].iter().all(|w| signature(w) == s0)
}

pub fn cd_block_lengths(w: &str) -> Vec<usize> {
    let mut lens = Vec::<usize>::new();
    let mut cur = 0usize;
    for ch in w.bytes() {
        match ch {
            b'b' => {
                lens.push(cur);
                cur = 0;
            }
            b'c' | b'd' => cur += 1,
            _ => {}
        }
    }
    lens.push(cur);
    lens
}

pub fn residual_profile_mod(w: &str, m: usize) -> Vec<usize> {
    let lens = cd_block_lengths(w);
    lens.into_iter().map(|k| k % m).collect()
}

pub fn fuzz_profile_once(
    rules: &Rules,
    rng: &mut StdRng,
    min_len: usize,
    max_len: usize,
    min_steps: usize,
    max_steps: usize,
    m: usize,
) -> bool {
    let w0 = random_word(min_len, max_len, rng);
    let chain = random_chain_t(&w0, rules, min_steps, max_steps, rng);
    let r0 = residual_profile_mod(&chain[0], m);
    chain[1..].iter().all(|w| residual_profile_mod(w, m) == r0)
}