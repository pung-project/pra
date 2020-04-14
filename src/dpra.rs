use super::{Process, ProcessType};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::cmp::{max, min};

// poly functions that we're using

// Alpenhorn uses delta = 10^-4.
// We need an h(lambda) such that 0.5e^(1-h(lambda)/g(lambda)) = 10^-4
// when lambda = 10, h(lambda) = 13.28, and 0.5e^(1-h(lambda)/g(lambda)) = 10^-4
fn h(lambda: f64) -> f64 {
    1.328 * lambda
}

// Alpenhorn uses epsilon = ln(2). We get this when lambda = 10
fn g(lambda: f64) -> f64 {
   lambda / (10.0 * 2.0_f64.ln())
}

fn laplace(location: f64, scale: f64) -> f64 {
    let mut rng = rand::thread_rng();
    // gen_range samples from [low, high). Below samples from (-0.5, 0.5]
    let u: f64 = 0.0 - rng.gen_range(-0.5, 0.5);
    location - scale * u.signum() * (1.0 - 2.0 * u.abs())
}

pub fn ra(k: u64, lambda: f64, p_hon_bound: u64, proc: &[Process]) -> Vec<Process> {
    let mut rng = rand::rngs::OsRng;

    let location = p_hon_bound as f64 * h(lambda);
    let scale = p_hon_bound as f64 * g(lambda);

    let n: u64 = max(laplace(location, scale).ceil() as i64, 0) as u64;
    let p_len: u64 = proc.len() as u64;
    let t = p_len + n;
    let m = min(t, k);

    let mut p_clone = proc.clone().to_owned();

    //TODO: Add back snapping mechanism...

    let mut u = Vec::new();

    for i in 0..m {
        let r = rng.gen::<f64>();
        if r < (p_clone.len() as f64 / (t - i) as f64) {
            // Sample without replacement (not efficient!)
            let idx: usize = rng.gen_range(0, p_clone.len());
            u.push(p_clone[idx]);
            p_clone.remove(idx);
        }
    }

    u
}

pub fn wra(k: u64, lambda: f64, p_hon_bound: u64, proc: &[Process]) -> Vec<Process> {

    let mut rng = rand::rngs::OsRng;
    let mut q = Vec::new();

    let location = p_hon_bound as f64 * h(lambda);
    let scale = p_hon_bound as f64 * g(lambda);
    let n: u64 = max(laplace(location, scale).ceil() as i64, 0) as u64;
    let p_len: u64 = proc.len() as u64;

    for i in 0..max(n, 0) {
        q.push(Process {
            id: p_len + i,
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

    for _ in 0..min(n + p_len, k) as usize {
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
