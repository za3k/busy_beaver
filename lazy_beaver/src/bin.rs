use std::time::Instant;

fn main() {
    let mut max_steps = 100;
    let start = Instant::now();
    for n in 1..=5 {
        let theory_machines: u64 = (n as u64*4+1).pow(n as u32*2);
        loop { 
            let result = abstract_turing::lazy_beaver_limited(n, max_steps);
            match result {
                Err(machines) => {
                    print!("LB({}) > {} [{} machines, {}s]\n", n, max_steps, machines, start.elapsed().as_secs());
                    max_steps *= 10;
                }
                Ok((machines, steps)) => {
                    print!("LB({}) = {} [{} machines, {}s, {}x speedup]\n", n, steps, machines, start.elapsed().as_secs(), theory_machines/machines);
                    break;
                }
            }
        }
    }
}
