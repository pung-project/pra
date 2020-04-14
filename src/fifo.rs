use super::Process;

pub fn ra(k: u64, proc: &[Process]) -> Vec<Process> {
    if proc.len() as u64 > k {
        proc[0..(k as usize)].to_vec()
    } else {
        proc.to_vec()
    }
}
