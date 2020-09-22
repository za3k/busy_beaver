# Busy Beaver Problem (and related problems)

## Turing machines
Here we use the usual definition of a Turing machine. There is a single one-dimensional tape of cells, extending infinitely to the left and right. Each cell can be 0 or 1, and they are initially all 0. The Turing machine has a "read-head" hovering over a particular cell, and an internal state, initially state 0. Using only the cell value under the read-head and the current state, it looks up in a fixed lookup table (the "Turing machine"):
- HALT or
- Write a (fixed) value to the current cell, then move left or right (no staying still), then transition to a (fixed) new state.

**Busy beaver problem**. BB(x) = N. For each integer x, what is the highest number N such that there is a Turing machine with x states, halting after EXACTLY N steps? This function is uncomputable.

**Lazy beaver problem** - [Introduced](https://www.scottaaronson.com/blog/?p=4916#comment-1850265) by "Job", I read it in [Scott Aaronson](https://www.scottaaronson.com/blog/?p=4916)'s [busy beaver problem survey](https://www.scottaaronson.com/papers/bb.pdf). This function is computable, and a python script is included which can calculate small values.
LB(x) = N. For each integer x, what is the least number N such that there is NO Turing machine with x states halting after EXACTLY N steps?

**Beeping beaver problem** or "quasihalting" problem. Introduced by Harvey Friedman and Scott Aaronson in the same survey, problem reformulation by [Nick Drozd](https://nickdrozd.github.io/2020/08/13/beeping-busy-beavers.html).
BBB(x) = N. For each integer x, what is the greatest number N such that there is a Turing machine which is in state s_0 on step N, but never returns to state s_0 afterwards? Note that the Turing machine does not actually need the "halt" operation in this formulation.

# This repository

lazy_beaver.py and lazy_beaver.rs calculate values of LB(n) (works up until about n=5). 

- Run `cd lazy_beaver && cargo run | tee lazy_beaver.log` for the (faster) Rust version.
- Run `python3 lazy_beaver.py | tee lazy_beaver.log` for the Python version.
- Keep the machine powered on as long as possible and wait.

# Speedups used

There are (4n+1)^(2n) machines with n states, and we need to run each for LB(n) steps. We want algorithmic optimizations, not just runtime ones. This program includes the following optimizations to calculate on less machines:
- `2x` Interchanging "left" and "right" in a machine turns it into an equivalent machine. Assume the first tape move is to the right.
- `1x` Permuting the (non-zero) symbols is valid, but there's only one non-zero symbol.
- `(n-1)! x` (6x for 4-state, 24x for 5-state) Permuting the (non-start) states gives an equivalent machine. So assume the states are accessed in numerical order.
- `emperical` (140x for 4-state, 340x for 5-state) Work with abstract groups of machines, rather than specific ones. Transition information is initially not written down. While executing the turing machine, if we ever need to read transition information, and it's not present, we "split" the machine group (4n+1) ways, writing down all possible values for that transition, and resume execution on each of the 4n+1 new groups, throwing out the original. This means for example that all `(4n+1)^(2n-1)` machines that halt on the first step, are computed together in constant time.
- `40x` Rewritten from naive Python to (fairly naive) Rust. In theory it should run in L1 cache only now.
- No optimizations are used to detect infinite loops (ex, move right and stay in the same state forever).
- The application runs one one thread on one computer.
- `1/log n x` If we knew LB(n) in advance, we could run all machines for LB(n) steps, and confirm that some machine stops for each lower number of steps, and none of them stop in exactly LB(n) steps. But we don't know LB(n), so we have to pick another strategy.
  - Strategy 1: Run all Turing machines in parallel, keeping them in RAM. If one halts, throw it out. This is very fast, but requires RAM to store both all TMs not known to halt, and their working tapes, which quickly becomes too big.
  - Strategy 2: Guess LB(x) < N. Sequentially, run each TM for N steps, and write down whether it halted, and in how many steps. This needs one bit for each possible number of steps, and working tape for one machine. But, if you guess N too low, you have to increase your guess again and re-run from the start. By repeatedly doubling N, you can make running times only nearly optimal for the machines that run indefinitely. But, for the machines that stop in less than N steps, you run them over and over. I chose to use powers of 10 as guesses for N. I'm not sure what the slowdown is overall, but it looks like the rerun and the original one take comparable time, so I'm guessing overall the process takes `log n`x longer.
    - In the real program, this interacts with the abstract groups of machines speedup. We need to store up to (4N+1)(2N) groups in a stack. When a machine group splits, we just add the new groups to the stack, and restart the run for each group.

# Results (may be updated)

```
2020-09-19 run of lazy_beaver.py
LB(1) = 2 [4 machines searched, 1s search time]
LB(2) = 7 [168 machines searched, 1s search time]
LB(3) = 22 [23029 machines searched, 18s search time]
LB(4) = 72 [4244698 machines searched, 3425s search time]
```

```
2020-09-19 run of lazy_beaver.rs on same machine
LB(1) = 2 [4 machines, 0s, 6x speedup]
LB(2) = 7 [168 machines, 0s, 39x speedup]
LB(3) = 22 [23029 machines, 0s, 209x speedup]
LB(4) = 72 [4244681 machines, 88s, 1643x speedup]
LB(5) > 100 [1015422314 machines, 21383s]
LB(5) = 427 [1015960061 machines, 235676s, 16417x speedup]
```
