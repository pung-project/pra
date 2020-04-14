extern crate rand;
extern crate rand_distr;
extern crate statistical;

use rand::Rng;
use pra;
use std::cmp::{max, min};
use rand_distr::{Distribution, Poisson};

// Comparison of RRAs as the bounds change
fn ra_comparison() {
    // Fix k, m_bound, set p_bound = x * p_hon_bound, then vary p_hon_bound
    let iterations = 100000;
    let lambda: f64 = 10.0;
    let k = 10;
    let m_bound = 2000;     // 2K friends
    let mut rng = rand::thread_rng();
    let dist = Poisson::new(50f64).unwrap(); // Poisson distribution with mean 50.

    println!("----------ra_comparison.log------------");
    println!("#k p_hon_bound p_bound m_bound SRA_utilization \
              RRA_utilization DPRA_utilization SRA_std RRA_std DPRA_std");

    for mut p_hon_bound in (0..=25).step_by(5) {
        p_hon_bound = max(1, p_hon_bound);
        let p_bound = 10 * p_hon_bound;

        let mut baseline;
        let mut sra_res = vec![];
        let mut rra_res = vec![];
        let mut dpra_res = vec![];

        for _ in 0..iterations {

            let num_processes: u64 = dist.sample(&mut rng);
            let hon = rng.gen_range(0, p_hon_bound + 1);
            let mal = min(num_processes, p_bound) - hon;

            let proc = pra::gen_proc(mal, hon);

            // baseline processes min(k, proc)
            baseline = min(k, proc.len() as u64) as f64;

            // SRA
            let u = pra::sra::ra(k, m_bound, &proc);
            sra_res.push(u.len() as f64 / baseline);

            // RRA
            let u = pra::rra::ra(k, p_bound, &proc);
            rra_res.push(u.len() as f64 / baseline);

            // DPRA
            let u = pra::dpra::ra(k, lambda, p_hon_bound, &proc);
            dpra_res.push(u.len() as f64 / baseline);
        }

        println!(
            "{} {} {} {} {} {} {} {} {} {}",
            k,
            p_hon_bound,
            p_bound,
            m_bound,
            statistical::mean(&sra_res), // SRA
            statistical::mean(&rra_res), // RRA
            statistical::mean(&dpra_res), // DPRA
            statistical::standard_deviation(&sra_res, None),
            statistical::standard_deviation(&rra_res, None),
            statistical::standard_deviation(&dpra_res, None),
        );
    }
}

fn main() {
    ra_comparison();
}
