extern crate rand;

use pra;
use pra::{Process, ProcessType};
use rand::Rng;
use std::collections::HashMap;

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
fn rra_priv() {
    let iterations = 100000; // Number of iterations
    let k = 10;              // Resource capacity
    let num_processes = 100; // Total number of processes
    let mut rng = rand::rngs::OsRng;
    let p_hon_bound = 10;
    let p_bound = 100;

    let mut tmals_b0: HashMap<usize, usize> = HashMap::new();
    let mut tmals_b1: HashMap<usize, usize> = HashMap::new();

    println!("----------rra_priv.log------------");

    for _ in 0..iterations {
        // pick # of honest processes from [0, p_hon_bound]
        let hon = rng.gen_range(0, p_hon_bound + 1);
        let mal = num_processes - hon;

        let p_mal = pra::gen_proc(mal, 0); // b = 0
        let p_mal_and_hon = pra::gen_proc(mal, hon); // b = 1

        let u_0 = pra::rra::ra(k, p_bound, &p_mal); // b = 0
        let u_1 = pra::rra::ra(k, p_bound, &p_mal_and_hon); // b = 1

        // Count malicious processes serviced in each case
        let tmal = mal_count(&u_0);
        let entry = tmals_b0.entry(tmal).or_insert(0);
        *entry = *entry + 1;

        let tmal = mal_count(&u_1);
        let entry = tmals_b1.entry(tmal).or_insert(0);
        *entry = *entry + 1;
    }

    // print histogram
    println!("#t_mal b=0 b=1");
    for i in 0..=k as usize {
        let t0 = match tmals_b0.get(&i) {
            Some(v) => *v,
            None => 0,
        };

        let t1 = match tmals_b1.get(&i) {
            Some(v) => *v,
            None => 0,
        };

        println!("{} {} {}", i, t0, t1);
    }
    println!("\n");
}

fn main() {
    rra_priv();
}
