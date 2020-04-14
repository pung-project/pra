extern crate rand;
extern crate rand_distr;

use rand::Rng;
use pra;
use std::cmp::{max, min};
use rand_distr::{Distribution, Poisson};
use rand::seq::SliceRandom;

fn latency_exp() {
    let iterations = 1000;
    let lambda: f64 = 10.0;

    let m_bound = 2000;     // 2K friends
    let p_bound = 100;      // At most 100 hon+malicious callers
    let p_hon_bound = 20;   // At most 10 honest

    println!("----------latency_comp.log------------");
    println!("#capacity p_hon_bound p_bound m_bound SRA RRA DPRA Baseline");

    let mut rng = rand::rngs::OsRng;

    for mut k in (0..=25).step_by(5) {
        k = max(k, 1);
        let mean_num_proc: f64 = k as f64;
        let dist = Poisson::new(mean_num_proc).unwrap();

        let mut sra_count = 0;
        let mut rra_count = 0;
        let mut dpra_count = 0;
        let mut fifo_count = 0;

        let frac_honest = p_hon_bound as f64 / p_bound as f64;

        for _ in 0..iterations {
            let num_processes: u64 = dist.sample(&mut rng);
            let hon = min(p_hon_bound - 1, ((num_processes as f64) * frac_honest).round() as u64);
            let mal = num_processes - hon;

            let target_p = pra::Process {
                id: rng.gen::<u64>() % m_bound,
                t: pra::ProcessType::Honest,
                w: 1.0,
            };

            let mut proc = pra::gen_proc(mal, hon);
            proc.push(target_p);
            proc.shuffle(&mut rng);

            // SRA
            let mut proc_clone = proc.clone();
            let mut u = pra::sra::ra(k, m_bound, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            sra_count += 1;

            while !u.contains(&target_p) {
                // all clients keep redial until the target's call is picked up
                sra_count += 1;
                u = pra::sra::ra(k, m_bound, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }

            // RRA
            proc_clone = proc.clone();
            u = pra::rra::ra(k, p_bound, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            rra_count += 1;

            while !u.contains(&target_p) {
                rra_count += 1;
                u = pra::rra::ra(k, p_bound, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }

            // DPRA

            proc_clone = proc.clone();
            u = pra::dpra::ra(k, lambda, p_hon_bound, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            dpra_count += 1;

            while !u.contains(&target_p) {
                dpra_count += 1;
                u = pra::dpra::ra(k, lambda, p_hon_bound, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }

            // Baseline
            proc_clone = proc.clone();
            u = pra::fifo::ra(k, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            fifo_count += 1;

            while !u.contains(&target_p) {
                fifo_count += 1;
                u = pra::fifo::ra(k, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }
        }

        println!(
            "{} {} {} {} {} {} {} {}",
            k,
            p_hon_bound,
            p_bound,
            m_bound,
            sra_count as f64 / iterations as f64, // SRA
            rra_count as f64 / iterations as f64, // RRA
            dpra_count as f64 / iterations as f64, // DPRA
            fifo_count as f64 / iterations as f64, // Baseline 
        );
    }
}

fn weighted_latency_exp() {
    let iterations = 1000;
    let lambda: f64 = 10.0;

    let m_bound = 2000; // 2K friends
    let p_bound = 100; // At most 100 hon+malicious callers
    let p_hon_bound = 20; // At most 20 honest

    println!("----------latency_comp_weight.log------------");
    println!("#capacity p_hon_bound p_bound WRRA WDPRA Baseline");

    let mut rng = rand::thread_rng();

    for mut k in (0..=25).step_by(5) {
        k = max(k, 1);
        let mean_num_proc: f64 = k as f64;
        let dist = Poisson::new(mean_num_proc).unwrap();

        let mut wrra_count = 0;
        let mut wdpra_count = 0;
        let mut fifo_count = 0;

        let frac_honest = p_hon_bound as f64 / p_bound as f64;

        for _ in 0..iterations {
            let num_processes: u64 = dist.sample(&mut rng);
            let hon = min(p_hon_bound - 1, ((num_processes as f64) * frac_honest).round() as u64);
            let mal = num_processes - hon;

            // Weighted version
            let hon_w = min(p_hon_bound - 5, ((num_processes as f64) * frac_honest).round() as u64);
            let mal_w = num_processes - hon_w;

            let target_p = pra::Process {
                id: rng.gen::<u64>() % m_bound,
                t: pra::ProcessType::Honest,
                w: 1.0,
            };

            let target_p_w = pra::Process {
                id: rng.gen::<u64>() % m_bound,
                t: pra::ProcessType::Honest,
                w: 5.0,
            };

            let mut proc = pra::gen_proc(mal, hon);
            proc.push(target_p);
            proc.shuffle(&mut rng);

            let mut proc_w = pra::gen_proc(mal_w, hon_w);
            proc_w.push(target_p_w);
            proc.shuffle(&mut rng);

            // WRRA
            let mut proc_clone = proc_w.clone();
            let mut u = pra::rra::wra(k, p_bound, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            wrra_count += 1;

            while !u.contains(&target_p_w) {
                wrra_count += 1;
                u = pra::rra::wra(k, p_bound, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }

            // WDPRA
            proc_clone = proc_w.clone();
            u = pra::dpra::wra(k, lambda, p_hon_bound, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            wdpra_count += 1;

            while !u.contains(&target_p_w) {
                wdpra_count += 1;
                u = pra::dpra::wra(k, lambda, p_hon_bound, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }

            // Baseline
            proc_clone = proc.clone();
            u = pra::fifo::ra(k, &proc_clone);
            proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            fifo_count += 1;

            while !u.contains(&target_p) {
                fifo_count += 1;
                u = pra::fifo::ra(k, &proc_clone);
                proc_clone.retain(|&x| u.iter().find(|&&y| y == x) == None );
            }
        }

        println!(
            "{} {} {} {} {} {}",
            k,
            p_hon_bound,
            p_bound,
            wrra_count as f64 / iterations as f64, // WRRA
            wdpra_count as f64 / iterations as f64, // WDPRA
            fifo_count as f64 / iterations as f64, // Baseline
        );
    }
}


fn main() {
    latency_exp();
    weighted_latency_exp();
}
