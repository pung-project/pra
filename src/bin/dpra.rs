extern crate rand;

use pra;
use pra::{Process, ProcessType};
use rand::Rng;
use std::cmp::min;
use std::collections::HashMap;

// Measure the utilization of DPRA
fn dpra_util() {
    let iterations = 10000; // Number of iterations
    let k = 10; // Resource capacity
    let num_processes = 100; // Total number of processes
    let mut rng = rand::rngs::OsRng;

    for p_hon_bound in (5..=20).step_by(5) {
        // vary bound by 5

        println!("----------dpra_util_{}.log------------", p_hon_bound);
        println!("# bound lambda utilization");

        for i in 0..5 {
            // Different amount of leakage. Recall that \epsilon = 1/g(lambda)

            let lambda = (10 as f64).powi(i);
            let mut baseline = 0;
            let mut dpra_sum = 0;

            for _ in 0..iterations {
                // pick # of honest processes from [0, p_hon_bound]
                let hon = rng.gen_range(0, p_hon_bound + 1);
                let mal = num_processes - hon;

                let proc = pra::gen_proc(mal, hon);

                // baseline processes min(k, proc)
                baseline += min(k, proc.len() as u64);

                // DPRA
                let u = pra::dpra::ra(k, lambda, p_hon_bound, &proc);
                dpra_sum += u.len();
            }
            println!("{} {} {}", p_hon_bound, lambda, dpra_sum as f64 / baseline as f64);
        }
        println!("\n");
    }
}

// counts number of malicious processes
fn mal_count(proc: &Vec<Process>) -> usize {
    let mut count = 0;
    for p in proc {
        if p.t == ProcessType::Malicious {
            count += 1;
        }
    }

    count
}

// Runs the challenger and adversary and creates a historgram
fn dpra_priv(lambda: f64) {
    let iterations = 100000; // Number of iterations
    let k = 10; // Resource capacity
    let num_processes = 100; // Total number of processes
    let mut rng = rand::thread_rng();
    let p_hon_bound = 10;

    let mut tmals_b0: HashMap<usize, usize> = HashMap::new();
    let mut tmals_b1: HashMap<usize, usize> = HashMap::new();

    println!("----------dpra_priv_{}.log------------", lambda as u64); 

    for _ in 0..iterations {
        // pick # of honest processes from [0, p_hon_bound]
        let hon = rng.gen_range(0, p_hon_bound + 1);
        let mal = num_processes - hon;

        let p_mal = pra::gen_proc(mal, 0); // b = 0
        let p_mal_and_hon = pra::gen_proc(mal, hon); // b = 1

        let u_0 = pra::dpra::ra(k, lambda, p_hon_bound, &p_mal); // b = 0
        let u_1 = pra::dpra::ra(k, lambda, p_hon_bound, &p_mal_and_hon); // b = 1

        // Count malicious processes serviced in each case
        let tmal = mal_count(&u_0);
        let entry = tmals_b0.entry(tmal).or_insert(0);
        *entry = *entry + 1;

        let tmal = mal_count(&u_1);
        let entry = tmals_b1.entry(tmal).or_insert(0);
        *entry = *entry + 1;
    }

    // print histogram
    println!("# lambda t_mal b=0 b=1");
    for i in 0..=k as usize {
        let t0 = match tmals_b0.get(&i) {
            Some(v) => *v,
            None => 0,
        };

        let t1 = match tmals_b1.get(&i) {
            Some(v) => *v,
            None => 0,
        };

        println!("{} {} {} {}", lambda, i, t0, t1);
    }
    println!("\n");
}

fn main() {
    dpra_util();
    dpra_priv(5f64);
    dpra_priv(10f64);
}
