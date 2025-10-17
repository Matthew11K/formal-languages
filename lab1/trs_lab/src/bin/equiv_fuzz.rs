use rand::prelude::*;
use trs_lab::cfg;
use trs_lab::*;

fn main() {
    let mut rng = StdRng::seed_from_u64(cfg::RNG_SEED);

    let t = cfg::rules_t();
    let tp = cfg::rules_tprime();

    let mut ok = 0usize;
    let mut skip = 0usize;

    for _ in 0..cfg::NUM_TRIALS {
        let pair = produce_pair(
            &mut rng,
            &t,
            cfg::GEN_MIN_LEN,
            cfg::GEN_MAX_LEN,
            cfg::PAIR_MIN_STEPS,
            cfg::CHAIN_MAX_STEPS,
            cfg::PAIR_TRIES,
        );
        if let Some((w, w2)) = pair {
            let hit =
                meet_bfs(&w, &w2, &tp, cfg::MAX_DEPTH, cfg::MAX_VISITED) ||
                meet_bfs(&w2, &w, &tp, cfg::MAX_DEPTH, cfg::MAX_VISITED);
            if hit { ok += 1; } else { skip += 1; }
        } else {
            skip += 1;
        }
    }

    let total = cfg::NUM_TRIALS;
    let linked_pct = (ok as f64) * 100.0 / (total as f64);
    let unresolved_pct = (skip as f64) * 100.0 / (total as f64);
    
    println!("T′BFS summary");
    println!("  pairs checked : {total}");
    println!("  reachable     : {ok}  ({linked_pct:.1}%)");
    println!("  no path found : {skip} ({unresolved_pct:.1}%)");
}
