use abstract_turing::lazy_beaver_limited;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fmt;

struct Parameters {
    states: u8,
    max_steps: u64,
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.states, self.max_steps)
    }
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lazy_beaver_limited");
    for param in &[
        Parameters {
            states: 1,
            max_steps: 10,
        },
        Parameters {
            states: 2,
            max_steps: 10,
        },
        Parameters {
            states: 3,
            max_steps: 10,
        },
        Parameters {
            states: 3,
            max_steps: 100,
        },
        Parameters {
            states: 4,
            max_steps: 100,
        },
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(param), &param, |b, &param| {
            b.iter(|| lazy_beaver_limited(param.states, param.max_steps))
        });
    }
}

criterion_group!(name = benches;
                 config = Criterion::default();
                 targets = benchmark);
criterion_main!(benches);
