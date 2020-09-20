# Busy Beaver Problem (and related problems)
Here we use the usual definition of a turning machine--there is a double-sided, infinite tape of cells. Each cell can be 0 or 1, and they are initially all zeros. The turing machine uses its current state and the symbol under the read head, together with a fixed lookup table, to:
- HALT or
- Write a (fixed) value to the current cell, then move left or right (no staying still), then transition to a (fixed) new state.

Busy beaver problem. BB(x) = N. For each integer x, what is the highest number N such that there is a Turing machine with x states, halting after EXACTLY N steps? This function is uncomputable.

Lazy beaver problem - [Introduced](https://www.scottaaronson.com/blog/?p=4916#comment-1850265) by "Job", I read it in [Scott Aaronson](https://www.scottaaronson.com/blog/?p=4916)'s [busy beaver problem survey](https://www.scottaaronson.com/papers/bb.pdf). This function is computable, and a python script is included which can calculate small values.
LB(x) = N. For each integer x, what is the least number N such that there is NO Turing machine with x states halting after EXACTLY N steps?

Beeping beaver problem or "quasihalting" problem. Introduced by Harvey Friedman and Scott Aaronson in the same survey, problem reformulation by Nick Drozd.
BBB(x) = N. For each integer x, what is the greatest number N such that there is a Turing machine which is in state s_0 on step N, but never returns to state s_0 afterwards? Note that the Turing machine does not actually need the "halt" operation in this formulation.

# This repository

lazy_beaver.py is provided which calculates small values of LB(x) (good up until about x=5). Run `python3 lazy_beaver.py | tee lazy_beaver.log` on a machine which will stay powered on and otherwise idle for at least a week.

lazy_beaver.rs is exactly the same, but re-written in Rust for speed. Run `cd lazy_beaver && cargo run | tee lazy_beaver.log`.

# Speedups used

There are (4n+1)^(2n) machines with n states, so we want algorithmic optimizations. This program includes the following optimizations to calculate on less machines:
- (~60x for 4-state machines, emperically) Work with abstract machines. Transition information is not written down until it's used. This means for example that all `(4n+1)^(2n-1)` machines that halt on the first step, are computed together in constant time. We "split" a machine into more concrete machines each time the information in a transition is needed.
- (2x) Assume the first tape move is to the left. Interchanging "left" and "right" in a machine turns it into an equivalent machine.
- ((n-1)! x, ex. 6x for 4-state, 24x for 5-state) Assume the states are accessed in numerical order. Permuting the states of the machine, as long as the initial state is the same, turns the machine into an equivalent machine. 
- (Note that permuting the non-zero symbols on both read and write is also valid, but this is a no-op for 2-symbol tapes. 0 and 1 can't be swapped because the tape starts filled with all 0s.)
- No optimizations are used to detect infinite loops.

# Results (may will be updated)

```
2020-09-19 run of lazy_beaver.py
LB(0) = 2 [2 machines searched, 1s search time]
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
```

```
