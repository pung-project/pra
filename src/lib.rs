use rand::distributions::{Distribution, Uniform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessType {
    Honest,
    Malicious,
    Dummy,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Process {
    pub id: u64,
    pub t: ProcessType,
    pub w: f64,
}

pub fn gen_proc(mut mal: u64, mut hon: u64) -> Vec<Process> {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0..2);
    let tot = mal + hon;
    let mut proc = Vec::with_capacity(tot as usize);

    for i in 0..tot {
        let coin = uniform.sample(&mut rng);

        let ty = if coin == 0 && hon > 0 {
            hon -= 1;
            ProcessType::Honest
        } else if coin == 1 && mal > 0 {
            mal -= 1;
            ProcessType::Malicious
        } else if hon == 0 {
            mal -= 1;
            ProcessType::Malicious
        } else {
            hon -= 1;
            ProcessType::Honest
        };

        let p = Process {
            id: i,
            t: ty,
            w: 1.0,
        };
        proc.push(p);
    }
    proc
}

pub mod dpra;
pub mod sra;
pub mod rra;
pub mod fifo;
