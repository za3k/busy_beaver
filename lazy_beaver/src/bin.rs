use std::time::Instant;

fn main() {
    let mut max_steps = 10;
    let start = Instant::now();
    for n in 1..=10 {
        let theory_machines: u64 = (n as u64 * 4 + 1).pow(n as u32 * 2);
        loop {
            let (info, result) = abstract_turing::lazy_beaver_limited(n, max_steps);
            match result {
                None => {
                    println!(
                        "LB({}) > {} [{}/{}h/{}nh/{}? machines, {}s]",
                        n,
                        max_steps,
                        info.0,
                        info.1,
                        info.2,
                        info.0 - info.1 - info.2,
                        start.elapsed().as_secs()
                    );
                    max_steps *= 10;
                }
                Some(steps) => {
                    println!("LB({}) = {} [{}/{}h/{}nh/{}? machines, {}s, {}x speedup, {:.2}% unclassified]",
                             n,
                             steps,
                             info.0,
                             info.1,
                             info.2,
                             info.0-info.1-info.2,
                             start.elapsed().as_secs(),
                             theory_machines/info.0,
                             ((info.0-info.1-info.2) as f64)/(info.0 as f64)*100f64);
                    break;
                }
            }
        }
    }
}
