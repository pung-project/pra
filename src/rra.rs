use super::{Process, ProcessType};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::cmp:: min;

pub fn ra(k: u64, p_bound: u64, proc: &[Process]) -> Vec<Process> {
    assert!(p_bound >= proc.len() as u64);

    let mut rng = rand::rngs::OsRng;
    let mut p_clone = proc.clone().to_owned();
    let mut u = Vec::new();

    for i in 0..min(k, p_bound) {
        let r = rng.gen::<f64>();
        if r < (p_clone.len() as f64 / (p_bound - i) as f64) {
            // Sample without replacement (this impl is not efficient, but it's good enough).
            let idx: usize = rng.gen_range(0, p_clone.len());
            u.push(p_clone[idx]);
            p_clone.remove(idx);
        }
    }

    u
}

pub fn wra(k: u64, p_bound: u64, proc: &[Process]) -> Vec<Process> {

    // Check weights repect bound
    let mut sum: f64 = 0.0;
    for p in proc.iter() {
        sum += p.w;
    }

    assert!(sum as u64 <= p_bound);

    let mut rng = rand::rngs::OsRng;
    let mut q = Vec::new();

    for i in proc.len() as u64..p_bound {
        q.push(Process {
            id: i,
            t: ProcessType::Dummy,
            w: 1.0,
        });
    }

    let mut pi = proc.to_owned();
    pi.append(&mut q);

    // get weights
    let mut weights = Vec::new();
    for i in &pi {
        weights.push(i.w);
    }

    let mut u = Vec::new();

    for _ in 0..k as usize {
        let dist = WeightedIndex::new(&weights).unwrap();
        let sampled_idx: usize = dist.sample(&mut rng);

        let is_dummy = pi[sampled_idx].t;

        if is_dummy != ProcessType::Dummy {
            u.push(pi[sampled_idx]);
        }

        pi.remove(sampled_idx);
        weights.remove(sampled_idx);
    }

    u
}
