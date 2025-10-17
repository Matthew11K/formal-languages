use rand::prelude::*;
use trs_lab::cfg;
use trs_lab::*;

fn main() {
    let mut rng = StdRng::seed_from_u64(cfg::RNG_SEED);

    let t = cfg::rules_t();
    let tp = cfg::rules_tprime();

    let uni_t = unit_invariants_ok(&t);
    let uni_tp = unit_invariants_ok(&tp);
    println!(
        "Unit invariants  T: {}  |  T′: {}",
        if uni_t { "OK" } else { "FAIL" },
        if uni_tp { "OK" } else { "FAIL" }
    );

    let mut ok_t = 0usize;
    let mut ok_tp = 0usize;

    for _ in 0..cfg::NUM_TRIALS {
        if fuzz_sig_once(&t, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS) {
            ok_t += 1;
        }
        if fuzz_sig_once(&tp, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS) {
            ok_tp += 1;
        }
    }

    println!("Fuzz invariants ({}) trials:", cfg::NUM_TRIALS);
    println!("  T :  ok = {}, fail = {}", ok_t, cfg::NUM_TRIALS - ok_t);
    println!("  T′:  ok = {}, fail = {}", ok_tp, cfg::NUM_TRIALS - ok_tp);
}
