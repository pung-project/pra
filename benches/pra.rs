#[macro_use]
extern crate criterion;
extern crate rand_distr;
extern crate rand;

use criterion::black_box;
use criterion::Criterion;
use rand_distr::{Distribution, Poisson};
use rand::Rng;
use std::cmp::min;

const M_BOUND: u64 = 1000000;
const P_BOUND: u64 = 100;
const P_HON_BOUND: u64 = 20;
const K: u64 = 10;


fn sra_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = Poisson::new(50f64).unwrap(); // Poisson distribution with mean 50.

    c.bench_function("sra", move |b| {
        let num_processes: u64 = dist.sample(&mut rng);
        let hon = rng.gen_range(0, P_HON_BOUND);
        let mal = min(num_processes, P_BOUND) - hon;
        let proc = pra::gen_proc(mal, hon);

        b.iter(|| {
            black_box(pra::sra::ra(K, M_BOUND, &proc))
        })
    });
}

fn rra_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = Poisson::new(50f64).unwrap(); // Poisson distribution with mean 50.

    c.bench_function("rra", move |b| {
        let num_processes: u64 = dist.sample(&mut rng);
        let hon = rng.gen_range(0, P_HON_BOUND+1);
        let mal = min(num_processes, P_BOUND) - hon;
        let proc = pra::gen_proc(mal, hon);

        b.iter(|| {
            black_box(pra::rra::ra(K, P_BOUND, &proc))
        })
    });
}

fn dpra_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = Poisson::new(50f64).unwrap(); // Poisson distribution with mean 50.
    let lambda = 10;

    c.bench_function("dpra", move |b| {
        let num_processes: u64 = dist.sample(&mut rng);
        let hon = rng.gen_range(0, P_HON_BOUND+1);
        let mal = min(num_processes, P_BOUND) - hon;
        let proc = pra::gen_proc(mal, hon);

        b.iter(|| black_box(pra::dpra::ra(K, lambda as f64, P_HON_BOUND, &proc)))
    });
}

criterion_group!(benches, sra_bench, rra_bench, dpra_bench);
criterion_main!(benches);
