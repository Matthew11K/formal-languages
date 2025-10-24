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

    let mut ok_r2_t = 0usize;
    let mut ok_r2_tp = 0usize;
    let mut ok_r3_t = 0usize;
    let mut ok_r3_tp = 0usize;

    for _ in 0..cfg::NUM_TRIALS {
        if fuzz_sig_once(&t, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS) {
            ok_t += 1;
        }
        if fuzz_sig_once(&tp, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS) {
            ok_tp += 1;
        }
        if fuzz_profile_once(&t, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS, 2) {
            ok_r2_t += 1;
        }
        if fuzz_profile_once(&tp, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS, 2) {
            ok_r2_tp += 1;
        }
        if fuzz_profile_once(&t, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS, 3) {
            ok_r3_t += 1;
        }
        if fuzz_profile_once(&tp, &mut rng, cfg::GEN_MIN_LEN, cfg::GEN_MAX_LEN, 1, cfg::CHAIN_MAX_STEPS, 3) {
            ok_r3_tp += 1;
        }
    }

    let n = cfg::NUM_TRIALS;
    println!("Fuzz invariants ({} trials):", n);
    println!("  Signature   (b, |cd|, bx):     T  ok = {}, fail = {}", ok_t, n - ok_t);
    println!("                                   T′ ok = {}, fail = {}", ok_tp, n - ok_tp);
    println!("  Residual r2 (mod 2 profile):    T  ok = {}, fail = {}", ok_r2_t, n - ok_r2_t);
    println!("                                   T′ ok = {}, fail = {}", ok_r2_tp, n - ok_r2_tp);
    println!("  Residual r3 (mod 3 profile):    T  ok = {}, fail = {}", ok_r3_t, n - ok_r3_t);
    println!("                                   T′ ok = {}, fail = {}", ok_r3_tp, n - ok_r3_tp);
}
