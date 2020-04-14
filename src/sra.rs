use super::Process;
use rand::Rng;

pub fn ra(k: u64, m_bound: u64, proc: &[Process]) -> Vec<Process> {
    let mut u = Vec::new();

    let mut rng = rand::rngs::OsRng;
    let r = rng.gen::<u64>();

    for p in proc {
        for i in 0..k {
            if p.id == (r + i) % m_bound {
                u.push(*p);
            }
        }
    }

    u
}
